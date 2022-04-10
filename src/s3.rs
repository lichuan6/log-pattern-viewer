use anyhow::Result;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use tokio::io::AsyncReadExt;

const BUCKET: &str = "nwlogs";
const REPORT_PATH: &str = "log-patterns-reports";

/// read report file from s3 bucket file
pub async fn read_report_file(
    s3: &S3Client,
    namespace: &str,
    app: &str,
    year: i32,
    month: i32,
) -> Result<String> {
    let key = report_file_key(namespace, app, year, month);
    let buf = get_object(s3, BUCKET, &key).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

/// read report file from s3 bucket file
pub async fn read_report_file_from_key(s3: &S3Client, key: &str) -> Result<String> {
    let buf = get_object(s3, BUCKET, key).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

/// Get object contents under bucket and use a key
pub async fn get_object(s3: &S3Client, bucket: &str, key: &str) -> Result<Vec<u8>> {
    let body = s3
        .get_object(GetObjectRequest {
            bucket: bucket.into(),
            key: key.into(),
            ..Default::default()
        })
        .await?
        .body
        .unwrap();

    let mut r = body.into_async_read();
    let mut buf = Vec::new();
    r.read_to_end(&mut buf).await?;

    Ok(buf)
}

/// Build report file key in s3 bucket
pub fn report_file_key(namespace: &str, app: &str, year: i32, month: i32) -> String {
    format!("{REPORT_PATH}/{namespace}/{app}/{year}/{month:0>2}/report.json")
}
