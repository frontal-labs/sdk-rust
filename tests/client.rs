use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
};

use frontal_sdk::{Client, EndpointPath, Error};
use serde::{Deserialize, Serialize};

struct MockResponse {
    url: String,
    request: mpsc::Receiver<String>,
}

fn mock_response(status: &str, headers: &[(&str, &str)], body: &str) -> MockResponse {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock server");
    let address = listener.local_addr().expect("read mock server address");
    let (sender, request) = mpsc::channel();
    let status = status.to_owned();
    let headers = headers
        .iter()
        .map(|(name, value)| format!("{name}: {value}\r\n"))
        .collect::<String>();
    let body = body.to_owned();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        let mut request_bytes = Vec::new();
        let mut buffer = [0; 2048];

        loop {
            let count = stream.read(&mut buffer).expect("read request");
            if count == 0 {
                break;
            }
            request_bytes.extend_from_slice(&buffer[..count]);

            let request_text = String::from_utf8_lossy(&request_bytes);
            if let Some(header_end) = request_text.find("\r\n\r\n") {
                let content_length = request_text[..header_end]
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                    .unwrap_or(0);
                if request_bytes.len() >= header_end + 4 + content_length {
                    break;
                }
            }
        }

        let request_text = String::from_utf8_lossy(&request_bytes).into_owned();
        let _ = sender.send(request_text);
        let response = format!(
            "HTTP/1.1 {status}\r\n{headers}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream
            .write_all(response.as_bytes())
            .expect("write response");
    });

    MockResponse {
        url: format!("http://{address}/v1/"),
        request,
    }
}

fn client_for(server: &MockResponse) -> Client {
    Client::builder("frt_test_key")
        .base_url(&server.url)
        .expect("valid mock server URL")
        .build()
        .expect("build client")
}

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    id: String,
}

#[derive(Serialize)]
struct CreateItem<'a> {
    name: &'a str,
}

#[tokio::test]
async fn get_sends_bearer_auth_and_encodes_query() {
    let server = mock_response(
        "200 OK",
        &[("Content-Type", "application/json")],
        r#"{"id":"item-1"}"#,
    );
    let client = client_for(&server);

    let item: Item = client
        .get_with_query("items", &[("search", "a b&c")])
        .await
        .expect("decode item response");

    assert_eq!(
        item,
        Item {
            id: "item-1".to_owned()
        }
    );
    let request = server.request.recv().expect("capture request");
    assert!(request.starts_with("GET /v1/items?search=a+b%26c HTTP/1.1\r\n"));
    assert!(request
        .to_ascii_lowercase()
        .contains("authorization: bearer frt_test_key"));
}

#[tokio::test]
async fn post_serializes_typed_json() {
    let server = mock_response(
        "201 Created",
        &[("Content-Type", "application/json")],
        r#"{"id":"item-2"}"#,
    );
    let client = client_for(&server);

    let item: Item = client
        .post("items", &CreateItem { name: "example" })
        .await
        .expect("decode created item");

    assert_eq!(item.id, "item-2");
    let request = server.request.recv().expect("capture request");
    assert!(request.starts_with("POST /v1/items HTTP/1.1\r\n"));
    assert!(request
        .to_ascii_lowercase()
        .contains("content-type: application/json"));
    assert!(request.ends_with(r#"{"name":"example"}"#));
}

#[tokio::test]
async fn api_errors_keep_status_request_id_and_message() {
    let server = mock_response(
        "403 Forbidden",
        &[
            ("Content-Type", "application/json"),
            ("X-Request-Id", "req-123"),
        ],
        r#"{"message":"access denied"}"#,
    );
    let client = client_for(&server);

    let error = client
        .get::<Item>("items/item-1")
        .await
        .expect_err("non-success response should fail");

    match error {
        Error::Api(error) => {
            assert_eq!(error.status().as_u16(), 403);
            assert_eq!(error.request_id(), Some("req-123"));
            assert_eq!(error.message(), "access denied");
            assert_eq!(
                error.body().and_then(|body| body.get("message")),
                Some(&serde_json::json!("access denied"))
            );
        }
        other => panic!("expected API error, received {other:?}"),
    }
}

#[tokio::test]
async fn empty_delete_accepts_no_content_response() {
    let server = mock_response("204 No Content", &[], "");
    let client = client_for(&server);

    let endpoint = EndpointPath::new("/items/item-1").expect("valid endpoint path");
    client
        .delete_empty(endpoint)
        .await
        .expect("204 response has no JSON body");

    assert!(server
        .request
        .recv()
        .expect("capture request")
        .starts_with("DELETE /v1/items/item-1 HTTP/1.1\r\n"));
}

#[tokio::test]
async fn post_empty_accepts_success_without_json() {
    let server = mock_response("202 Accepted", &[], "");
    let client = client_for(&server);

    client
        .post_empty("jobs", &serde_json::json!({ "name": "example" }))
        .await
        .expect("empty success response");

    let request = server.request.recv().expect("capture request");
    assert!(request.starts_with("POST /v1/jobs HTTP/1.1\r\n"));
    assert!(request.ends_with(r#"{"name":"example"}"#));
}

#[tokio::test]
async fn malformed_success_response_returns_decode_error() {
    let server = mock_response("200 OK", &[], "not-json");
    let client = client_for(&server);

    let error = client
        .get::<Item>("items")
        .await
        .expect_err("malformed response should fail decoding");

    assert!(matches!(error, Error::Decode(_)));
}

#[tokio::test]
async fn builder_rejects_invalid_configuration_and_unsafe_paths() {
    assert!(Client::new("  ").is_err());
    assert!(Client::builder("frt_key")
        .timeout(std::time::Duration::ZERO)
        .build()
        .is_err());
    assert!(Client::builder("frt_key").base_url("file:///tmp").is_err());

    let client = Client::new("frt_key").expect("build client");
    let error = client
        .get::<Item>("../other")
        .await
        .expect_err("reject parent path before sending a request");
    assert!(matches!(error, Error::InvalidPath(_)));
}

#[test]
fn debug_output_redacts_api_key() {
    let client = Client::new("frt_private_value").expect("build client");
    let builder = Client::builder("frt_private_value");

    assert!(!format!("{client:?}").contains("frt_private_value"));
    assert!(!format!("{builder:?}").contains("frt_private_value"));
}
