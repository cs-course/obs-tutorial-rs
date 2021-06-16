//! Put 请求测试

use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use reqwest::Url;
use s3_bench_rs::{PutTaskBuilder, StdError, Task, TaskBuiler};

#[tokio::main]
async fn put() -> Result<String, Box<StdError>> {
    let tasks: [(String, String); 2] = [
        ("bucket1".into(), "test0.txt".into()),
        ("bucket1".into(), "test1.txt".into()),
    ];
    let put_task_builder = PutTaskBuilder::new(
        "http://172.25.38.164:9000".parse::<Url>().unwrap(),
        "ccc",
        "WXZFwxzf123",
        "minio",
        tasks,
    );
    let task = put_task_builder.spawn("bucket0", "test.md");
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
