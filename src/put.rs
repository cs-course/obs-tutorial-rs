//! Put object task implemetation
use super::config::*;
use super::single::SingleTask;
use crate::{StdError, Task, TaskBuiler};
use async_trait::async_trait;
use reqwest::{Client, Url};
use rusty_s3::{actions::PutObject, Bucket, Credentials, S3Action};

pub struct PutTask(pub SingleTask);

impl PutTask {
    pub fn signed_url(&self) -> Url {
        let mut action = PutObject::new(&self.0.bucket, Some(&self.0.credentials), &self.0.object);
        action
            .query_mut()
            .insert("response-cache-control", "no-cache, no-store");
        action.sign(ONE_HOUR)
    }
}

#[async_trait]
impl Task for PutTask {
    type R = String;
    async fn run(self) -> Result<Self::R, Box<StdError>> {
        let signed_url = self.signed_url();
        let client = Client::new();
        let resp = client.put(signed_url).send().await?.error_for_status()?;
        Ok(resp.text().await?)
    }
}

pub struct PutTaskBuilder<const N: usize> {
    endpoint: Url,
    key: String,
    secret: String,
    region: String,
    tasks: [(String, String); N],
}

impl<const N: usize> PutTaskBuilder<N> {
    pub fn new<U, S>(
        endpoint: U,
        key: S,
        secret: S,
        region: S,
        tasks: [(String, String); N],
    ) -> Self
    where
        U: Into<Url>,
        S: Into<String>,
    {
        Self {
            endpoint: endpoint.into(),
            key: key.into(),
            secret: secret.into(),
            region: region.into(),
            tasks,
        }
    }
}

impl<const N: usize> TaskBuiler for PutTaskBuilder<N> {
    type R = String;
    type T = PutTask;
    type I = Vec<PutTask>;
    fn spawn(&self, bucket: &str, object: &str) -> Self::T {
        let bucket =
            Bucket::new(self.endpoint.clone(), true, bucket, self.region.as_str()).unwrap();
        let credentials = Credentials::new(self.key.clone(), self.secret.clone());
        PutTask(SingleTask::new(bucket, credentials, object))
    }

    fn spawn_tier(&self) -> Self::I {
        self.tasks
            .iter()
            .map(|(bucket, object)| self.spawn(bucket.as_str(), object.as_str()))
            .collect()
    }
}
