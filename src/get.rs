//! Get object task implementation
use rusty_s3::{Bucket, Credentials, S3Action, actions::GetObject};
use reqwest::{Client, Url};
use crate::{Task, TaskBuiler, StdError};
use async_trait::async_trait;
use super::config::*;

pub struct GetTask {
    pub(crate) bucket: Bucket,
    pub(crate) credentials: Credentials,
    pub(crate) object: String
}

impl GetTask {
    pub fn new<S: Into<String>>(
        bucket: Bucket,
        credentials: Credentials,
        object: S
    ) -> Self {
        Self {
            bucket,
            credentials,
            object: object.into()
        }
    }

    pub fn signed_url(&self) -> Url {
        let mut action = GetObject::new(&self.bucket, Some(&self.credentials), &self.object);
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
    pool: Vec<(String, String)>
}

impl GetTaskBuilder {
    pub fn new<U, S> (
        endpoint: U,
        key: S,
        secret: S,
        region: S,
    ) -> Self
    where
        U: Into<Url>,
        S: Into<String>
    {
        Self {
            endpoint: endpoint.into(),
            key: key.into(),
            secret: secret.into(),
            region: region.into(),
            pool: Vec::new()
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
        let bucket = Bucket::new(self.endpoint.clone(), true, bucket, self.region.as_str()).unwrap();
        let credentials = Credentials::new(self.key.clone(), self.secret.clone());
        GetTask::new(
            bucket,
            credentials,
            object
        )
    }

    fn spawn_tier(&self) -> Self::I {
        self.pool.iter().map(|(bucket, object)| self.spawn(bucket.as_str(), object.as_str())).collect()
    }
}