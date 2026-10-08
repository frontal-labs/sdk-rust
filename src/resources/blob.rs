//! Service scoped access for the `blob` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `blob` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct BlobService;

/// Typed access to the `blob` service.
pub type Blob = ServiceClient<BlobService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "blob";

impl ServiceClient<BlobService> {
    /// Upload an object using the Blob service's multipart form endpoint.
    pub async fn upload_object(
        &self,
        bucket: &str,
        key: &str,
        form: reqwest::multipart::Form,
    ) -> Result<()> {
        let path = format!(
            "blob/object/{}/{}",
            encode_path_segment(bucket),
            encode_path_segment(key)
        );
        self.post_form_empty(path, form).await
    }

    /// Download an object as bytes.
    pub async fn download_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>> {
        let path = format!(
            "blob/object/{}/{}",
            encode_path_segment(bucket),
            encode_path_segment(key)
        );
        let endpoint = self.validate("GET", &path, &[])?;
        self.client.get_bytes(endpoint).await
    }

    /// Delete an object from a bucket.
    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<()> {
        let path = format!(
            "blob/object/{}/{}",
            encode_path_segment(bucket),
            encode_path_segment(key)
        );
        self.delete_empty(path).await
    }

    /// List objects in a bucket using the request shape supported by the API.
    pub async fn list_objects<B, T>(&self, bucket: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let path = format!("blob/object/list/{}", encode_path_segment(bucket));
        self.post(path, request).await
    }

    /// Create a signed URL for a bucket object.
    pub async fn create_signed_url<B, T>(&self, bucket: &str, key: &str, request: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let path = format!(
            "blob/object/sign/{}/{}",
            encode_path_segment(bucket),
            encode_path_segment(key)
        );
        self.post(path, request).await
    }

    /// Copy an object between buckets or keys.
    pub async fn copy_object<B>(&self, request: &B) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        self.post_empty("blob/object/copy", request).await
    }

    /// Move or rename an object.
    pub async fn move_object<B>(&self, request: &B) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        self.post_empty("blob/object/move", request).await
    }

    /// Fetch object metadata.
    pub async fn object_metadata<T>(&self, bucket: &str, key: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = format!(
            "blob/object/info/{}/{}",
            encode_path_segment(bucket),
            encode_path_segment(key)
        );
        self.get(path).await
    }
}
