//! Put 请求测试

use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use reqwest::Url;
use s3_bench_rs::{PutTaskBuilder, StdError, Task, TaskBuiler};

const ENDPOINT: &str = "http://172.25.42.79:9000";
const KEY: &str = "ccc";
const SECRET: &str = "WXZFwxzf123";
const BUCKET: &str = "bucket0";
const OBJECT: &str = "test.md";

#[tokio::main]
async fn put() -> Result<String, Box<StdError>> {
    let tasks: [(String, String); 2] = [
        ("bucket1".into(), "test0.txt".into()),
        ("bucket1".into(), "test1.txt".into()),
    ];
    let put_task_builder = PutTaskBuilder::new(
        ENDPOINT.parse::<Url>().unwrap(),
        KEY,
        SECRET,
        "minio",
        tasks,
    );
    let task = put_task_builder.spawn(BUCKET, OBJECT);
    let resp = task.run().await?;
    Ok(resp)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Async PutObject", move |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let _ret = put();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
