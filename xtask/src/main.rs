use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
    process,
};

use serde::Deserialize;

const TYPESCRIPT_REFERENCE_COMMIT: &str = "8d0c43f5a1416255f0eaad0d828da774eb936f73";
const FUNCTIONS_TYPESCRIPT_REFERENCE_COMMIT: &str = "a82138eef859e6f0aed4f50f656cd0e015da8f58";

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Endpoint {
    method: String,
    path: String,
}

#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
struct UnmatchedRoute {
    service: String,
    method: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct ConformanceReport {
    matched: u64,
    unmatched: u64,
    openapi_matched: u64,
    openapi_unmatched: u64,
    typescript_reference_commit: String,
    openapi_unmatched_routes: Vec<UnmatchedRoute>,
}

fn main() {
    let result = match env::args().nth(1).as_deref() {
        Some("contract") => check_contract(),
        Some(command) => Err(format!("unknown xtask command: {command}")),
        None => Err("expected a command: cargo xtask contract".to_owned()),
    };

    if let Err(error) = result {
        eprintln!("xtask: {error}");
        process::exit(1);
    }
}

fn facade_binds_service(facade: &str, service: &str) -> bool {
    let signature = format!("pub fn {service}(&self)");
    let Some((_, accessor)) = facade.split_once(&signature) else {
        return false;
    };
    let accessor = accessor.split("\n    #[cfg").next().unwrap_or(accessor);
    accessor.contains("ServiceClient::new") && accessor.contains(&format!("\"{service}\""))
}

fn check_contract() -> Result<(), String> {
    let root = project_root();
    let inventory_path = root.join("contracts/sdk-endpoints.json");
    let inventory: BTreeMap<String, Vec<Endpoint>> = read_json(&inventory_path)?;
    let typescript_reference: BTreeMap<String, Vec<Endpoint>> =
        read_json(&root.join("contracts/references/typescript-sdk-endpoints.json"))?;
    if inventory != typescript_reference {
        return Err(format!(
            "{} differs from the frozen TypeScript endpoint inventory at {TYPESCRIPT_REFERENCE_COMMIT} with Functions routes sourced from {FUNCTIONS_TYPESCRIPT_REFERENCE_COMMIT}",
            inventory_path.display()
        ));
    }
    let resources = fs::read_to_string(root.join("src/resources/mod.rs"))
        .map_err(|error| format!("read resources/mod.rs: {error}"))?;
    let facade = fs::read_to_string(root.join("src/lib.rs"))
        .map_err(|error| format!("read lib.rs: {error}"))?;
    let manifest = fs::read_to_string(root.join("Cargo.toml"))
        .map_err(|error| format!("read Cargo.toml: {error}"))?;
    let registry = parse_service_registry(&resources)?;
    let expected = inventory
        .iter()
        .filter(|(_, endpoints)| !endpoints.is_empty())
        .map(|(service, _)| service.clone())
        .collect::<BTreeSet<_>>();

    if !registry.is_subset(&expected) {
        return Err(format!(
            "SDK service registry contains services absent from the endpoint inventory; inventory={expected:?}, SDK={registry:?}"
        ));
    }
    // Keep removals explicit so a deleted service cannot silently shrink SDK coverage.
    let excluded = expected
        .difference(&registry)
        .cloned()
        .collect::<BTreeSet<_>>();
    let intentional_exclusions = BTreeSet::from(["sandbox".to_owned()]);
    if excluded != intentional_exclusions {
        return Err(format!(
            "unexpected service exclusions; expected={intentional_exclusions:?}, actual={excluded:?}"
        ));
    }

    let mut openapi_operations = BTreeSet::new();
    for file in [
        "contracts/openapi/api.openapi.json",
        "contracts/openapi/ai.openapi.generated.json",
        "contracts/openapi/workflows.openapi.json",
    ] {
        let document: serde_json::Value = read_json(&root.join(file))?;
        let paths = document
            .get("paths")
            .and_then(serde_json::Value::as_object)
            .ok_or_else(|| format!("{file} has no paths object"))?;
        for (path, operations) in paths {
            let Some(operations) = operations.as_object() else {
                continue;
            };
            for method in operations.keys() {
                if is_http_method(method) {
                    openapi_operations.insert((method.to_ascii_uppercase(), normalize_path(path)));
                }
            }
        }
    }

    let mut endpoint_count = 0usize;
    let mut openapi_match_count = 0usize;
    let mut openapi_unmatched_routes = BTreeSet::new();
    for (service, endpoints) in inventory {
        if endpoints.is_empty() || !registry.contains(&service) {
            continue;
        }

        let module = root.join("src/resources").join(format!("{service}.rs"));
        let source = fs::read_to_string(&module)
            .map_err(|error| format!("missing service module {service}: {error}"))?;
        let service_marker = format!("pub const SERVICE_NAME: &str = \"{service}\";");
        if !source.contains(&service_marker) {
            return Err(format!("{service} module has no matching SERVICE_NAME"));
        }
        if !resources.contains(&format!("pub mod {service};")) {
            return Err(format!("{service} module is not exported from resources"));
        }
        if !facade_binds_service(&facade, &service) {
            return Err(format!(
                "Frontal::{service}() is not bound to the {service} route inventory"
            ));
        }
        if !manifest.contains(&format!("{service} = []")) {
            return Err(format!("{service} has no Cargo feature"));
        }

        for endpoint in endpoints {
            endpoint_count += 1;
            validate_endpoint(&service, &endpoint)?;
            let method = sdk_method(&endpoint).ok_or_else(|| {
                format!(
                    "unsupported SDK method {} for {}",
                    endpoint.method, endpoint.path
                )
            })?;
            if !resources.contains(&format!("pub async fn {method}")) {
                return Err(format!(
                    "{service} endpoint {} {} has no service transport method {method}()",
                    endpoint.method, endpoint.path
                ));
            }
            let route = endpoint.path.split('?').next().unwrap_or(&endpoint.path);
            let operation = (
                http_method(&endpoint.method).to_owned(),
                normalize_path(route),
            );
            if openapi_operations.contains(&operation) {
                openapi_match_count += 1;
            } else {
                openapi_unmatched_routes.insert(UnmatchedRoute {
                    service: service.clone(),
                    method: endpoint.method.clone(),
                    path: endpoint.path.clone(),
                });
            }
        }
    }

    let report: ConformanceReport = read_json(&root.join("contracts/reports/conformance.json"))?;
    let expected_unmatched_routes = report
        .openapi_unmatched_routes
        .into_iter()
        .collect::<BTreeSet<_>>();
    if report.matched != endpoint_count as u64
        || report.unmatched != 0
        || report.openapi_matched != openapi_match_count as u64
        || report.openapi_unmatched != openapi_unmatched_routes.len() as u64
        || report.typescript_reference_commit != TYPESCRIPT_REFERENCE_COMMIT
        || expected_unmatched_routes != openapi_unmatched_routes
    {
        return Err(format!(
            "conformance report does not match the endpoint inventory and OpenAPI mappings ({openapi_match_count} routes matched, {} inventory routes explicitly absent from OpenAPI)",
            openapi_unmatched_routes.len()
        ));
    }

    println!(
        "validated {endpoint_count} SDK routes across {} services against the frozen TypeScript endpoint inventory ({openapi_match_count} also present in the committed OpenAPI snapshots)",
        registry.len()
    );
    Ok(())
}

fn validate_endpoint(service: &str, endpoint: &Endpoint) -> Result<(), String> {
    let path = endpoint.path.split('?').next().unwrap_or(&endpoint.path);
    if path.is_empty()
        || path.starts_with("//")
        || path.contains('\\')
        || path.contains('#')
        || path.chars().any(char::is_control)
        || path.split('/').any(|segment| matches!(segment, "." | ".."))
    {
        return Err(format!(
            "unsafe endpoint in {service}: {} {}",
            endpoint.method, endpoint.path
        ));
    }
    if endpoint.path.contains('?') && !matches!(endpoint.method.as_str(), "GET" | "POST") {
        return Err(format!(
            "query parameters are unsupported for {} {}",
            endpoint.method, endpoint.path
        ));
    }
    Ok(())
}

fn sdk_method(endpoint: &Endpoint) -> Option<&'static str> {
    let query = endpoint.path.contains('?');
    match endpoint.method.as_str() {
        "GET" if query => Some("get_with_query"),
        "GET" => Some("get"),
        "GETRAW" => Some("get_bytes"),
        "POST" if query => Some("post_with_query"),
        "POST" => Some("post"),
        "POSTFORMDATA" => Some("post_form"),
        "POSTRAW" => Some("post_bytes"),
        "PUT" => Some("put"),
        "PATCH" => Some("patch"),
        "DELETE" => Some("delete"),
        "STREAM" => Some("events"),
        _ => None,
    }
}

fn http_method(method: &str) -> &str {
    match method {
        "GETRAW" | "STREAM" => "GET",
        "POSTFORMDATA" | "POSTRAW" => "POST",
        other => other,
    }
}

fn is_http_method(method: &str) -> bool {
    matches!(
        method.to_ascii_lowercase().as_str(),
        "get" | "post" | "put" | "patch" | "delete" | "head" | "options"
    )
}

fn normalize_path(path: &str) -> String {
    let path = path.strip_prefix("/v1/").unwrap_or(path);
    let path = path.strip_prefix("/v1").unwrap_or(path);
    let mut normalized = String::new();
    for segment in path.split('/') {
        if !normalized.is_empty() {
            normalized.push('/');
        }
        if segment.starts_with('{') && segment.ends_with('}') {
            normalized.push_str("{param}");
        } else {
            normalized.push_str(segment);
        }
    }
    normalized
}

fn parse_service_registry(resources: &str) -> Result<BTreeSet<String>, String> {
    let registry = resources
        .split("pub const SDK_SERVICES: &[&str] = &[")
        .nth(1)
        .and_then(|tail| tail.split("];").next())
        .ok_or_else(|| "resources/mod.rs has no SDK_SERVICES registry".to_owned())?;
    Ok(registry
        .lines()
        .filter_map(|line| line.trim().trim_end_matches(',').trim().strip_prefix('"'))
        .filter_map(|value| value.strip_suffix('"'))
        .map(str::to_owned)
        .collect())
}

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask is a workspace member")
        .to_path_buf()
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let contents =
        fs::read_to_string(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    serde_json::from_str(&contents).map_err(|error| format!("parse {}: {error}", path.display()))
}
