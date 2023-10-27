//! Get 请求测试

use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use reqwest::Url;
use s3_bench_rs::{GetTaskBuilder, StdError, Task, TaskBuiler};

const ENDPOINT: &str = "http://172.25.42.79:9000";
const KEY: &str = "ccc";
const SECRET: &str = "WXZFwxzf123";
const BUCKET: &str = "bucket0";
const OBJECT: &str = "test.md";
#[tokio::main]
async fn get() -> Result<String, Box<StdError>> {
    let get_task_builder =
        GetTaskBuilder::new(ENDPOINT.parse::<Url>().unwrap(), KEY, SECRET, "minio");
    let task = get_task_builder.spawn(BUCKET, OBJECT);
    let text = task.run().await?;
    Ok(text)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Async GetObject", move |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let _ret = get();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
