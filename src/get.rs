//! Get object task implementation
use super::config::*;
use super::single::SingleTask;
use crate::{StdError, Task, TaskBuiler};
use async_trait::async_trait;
use reqwest::{Client, Url};
use rusty_s3::{actions::GetObject, Bucket, Credentials, S3Action};

pub struct GetTask(pub SingleTask);

impl GetTask {
    pub fn signed_url(&self) -> Url {
        let mut action = GetObject::new(&self.0.bucket, Some(&self.0.credentials), &self.0.object);
        action
            .query_mut()
            .insert("response-cache-control", "no-cache, no-store");
        action.sign(ONE_HOUR)
    }
}

#[async_trait]
impl Task for GetTask {
    type R = String;
    async fn run(self) -> Result<Self::R, Box<StdError>> {
        let signed_url = self.signed_url();
        let client = Client::new();
        let resp = client.get(signed_url).send().await?.error_for_status()?;
        Ok(resp.text().await?)
    }
}

pub struct GetTaskBuilder {
    endpoint: Url,
    key: String,
    secret: String,
    region: String,
    pool: Vec<(String, String)>,
}

impl GetTaskBuilder {
    pub fn new<U, S>(endpoint: U, key: S, secret: S, region: S) -> Self
    where
        U: Into<Url>,
        S: Into<String>,
    {
        Self {
            endpoint: endpoint.into(),
            key: key.into(),
            secret: secret.into(),
            region: region.into(),
            pool: Vec::new(),
        }
    }

    pub fn append_task<S: Into<String>>(&mut self, bucket: S, object: S) {
        self.pool.push((bucket.into(), object.into()));
    }
}

impl TaskBuiler for GetTaskBuilder {
    type R = String;
    type T = GetTask;
    type I = Vec<GetTask>;
    fn spawn(&self, bucket: &str, object: &str) -> Self::T {
        let bucket = Bucket::new(
            self.endpoint.clone(),
            true,
            bucket.into(),
            self.region.clone(),
        )
        .unwrap();
        let credentials = Credentials::new(self.key.clone(), self.secret.clone());
        GetTask(SingleTask::new(bucket, credentials, object))
    }

    fn spawn_tier(&self) -> Self::I {
        self.pool
            .iter()
            .map(|(bucket, object)| self.spawn(bucket.as_str(), object.as_str()))
            .collect()
    }
}
