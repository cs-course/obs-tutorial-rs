//! Delete object task implementation

use super::config::*;
use super::single::SingleTask;
use crate::{StdError, Task, TaskBuiler};
use async_trait::async_trait;
use reqwest::{Client, Url};
use rusty_s3::{actions::DeleteObject, Bucket, Credentials, S3Action};

pub struct DeleteTask(pub SingleTask);

impl DeleteTask {
    pub fn signed_url(&self) -> Url {
        let mut action = DeleteObject::new(&self.0.bucket, Some(&self.0.credentials), &self.0.object);
        action
            .query_mut()
            .insert("response-cache-control", "no-cache, no-store");
        action.sign(ONE_HOUR)
    }
}

#[async_trait]
impl Task for DeleteTask {
    type R = ();
    async fn run(self) -> Result<Self::R, Box<StdError>> {
        let signed_url = self.signed_url();
        let client = Client::new();
        let resp = client.delete(signed_url).send().await?.error_for_status()?;
        let _text = resp.text().await?;
        Ok(())
    }
}

pub struct DeleteTaskBuilder<'a> {
    endpoint: Url,
    key: &'a str,
    secret: &'a str,
    region: &'a str,
    pool: Vec<(&'a str, &'a str)>,
}

impl<'a> DeleteTaskBuilder<'a> {
    pub fn new<U, S>(endpoint: U, key: &'a S, secret: &'a S, region: &'a S) -> Self
    where
        U: Into<Url>,
        S: AsRef<str>,
    {
        Self {
            endpoint: endpoint.into(),
            key: key.as_ref(),
            secret: secret.as_ref(),
            region: region.as_ref(),
            pool: Vec::new(),
        }
    }

    pub fn append_task<S: AsRef<str>>(&mut self, bucket: &'a S, object: &'a S) {
        self.pool.push((bucket.as_ref(), object.as_ref()));
    }
}

impl<'a> TaskBuiler for DeleteTaskBuilder<'a> {
    type R = ();
    type T = DeleteTask;
    type I = Vec<DeleteTask>;
    fn spawn(&self, bucket: &str, object: &str) -> Self::T {
        let bucket =
            Bucket::new(self.endpoint.clone(), true, bucket, self.region).unwrap();
        let credentials = Credentials::new(self.key.clone(), self.secret.clone());
        DeleteTask(SingleTask::new(bucket, credentials, object))
    }

    fn spawn_tier(&self) -> Self::I {
        self.pool
            .iter()
            .map(|(bucket, object)| self.spawn(bucket, object))
            .collect()
    }
}