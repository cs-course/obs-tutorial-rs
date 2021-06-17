//! Benchmark for AWS S3

mod config;
mod get;
mod put;
mod delete;
mod single;

use async_trait::async_trait;
pub use get::{GetTask, GetTaskBuilder};
pub use put::{PutTask, PutTaskBuilder};
pub use delete::{DeleteTask, DeleteTaskBuilder};
pub type StdError = dyn std::error::Error;

#[async_trait]
pub trait Task {
    type R: Sized;
    /// 夺取任务的所有权去运行，运行完毕后释放内存空间
    async fn run(self) -> Result<Self::R, Box<StdError>>;
}

/// 任务构建器
pub trait TaskBuiler {
    type R: Sized;
    type T: Task<R = Self::R>;
    type I: IntoIterator<Item = Self::T, IntoIter = std::vec::IntoIter<Self::T>>;
    fn spawn(&self, bucket: &str, object: &str) -> Self::T;
    fn spawn_tier(&self) -> Self::I;
}
