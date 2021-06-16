use reqwest::Url;
use s3_bench_rs::{GetTaskBuilder, StdError, Task, TaskBuiler};

#[tokio::main]
async fn main() -> Result<(), Box<StdError>> {
    let mut get_task_builer = GetTaskBuilder::new(
        "http://172.22.110.215:9000".parse::<Url>().unwrap(),
        "ccc",
        "WXZFwxzf123",
        "minio",
    );
    let task = get_task_builer.spawn("bucket0", "test.md");
    let text = task.run().await?;
    println!("{}", text);
    get_task_builer.append_task("bucket1", "test0.txt");
    get_task_builer.append_task("bucket1", "test1.txt");
    let mut tasks = get_task_builer.spawn_tier();
    while let Some(task) = tasks.pop() {
        let text = task.run().await?;
        println!("{}", text);
    }
    Ok(())
}
