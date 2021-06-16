use reqwest::Url;
use s3_bench_rs::{PutTaskBuilder, StdError, Task, TaskBuiler};

#[tokio::main]
async fn main() -> Result<(), Box<StdError>> {
    let tasks: [(String, String); 2] = [
        ("bucket1".into(), "test0.txt".into()),
        ("bucket1".into(), "test1.txt".into()),
    ];
    let put_task_builder = PutTaskBuilder::new(
        "http://172.22.110.215:9000".parse::<Url>().unwrap(),
        "ccc",
        "WXZFwxzf123",
        "minio",
        tasks,
    );
    let task = put_task_builder.spawn("bucket0", "test.md");
    let resp = task.run().await?;
    println!("{}", resp);
    let mut task_iter = put_task_builder.spawn_tier();
    while let Some(task) = task_iter.pop() {
        let resp = task.run().await?;
        println!("{}", resp);
    }
    Ok(())
}
