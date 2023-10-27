use reqwest::Url;
use s3_bench_rs::{DeleteTaskBuilder, StdError, Task, TaskBuiler};

#[tokio::main]
async fn main() -> Result<(), Box<StdError>> {
    let key = "ccc".to_string();
    let secret = "WXZFwxzf123".to_string();
    let region = "minio".to_string();
    let mut get_task_builer = DeleteTaskBuilder::new(
        "http://172.25.41.98:9000".parse::<Url>().unwrap(),
        &key,
        &secret,
        &region,
    );
    let task = get_task_builer.spawn("bucket0", "test.md");
    task.run().await?;
    let data = [
        ("bucket1".to_string(), "test0.txt".to_string()),
        ("bucket1".to_string(), "test1.txt".to_string()),
    ];
    for d in data.iter() {
        get_task_builer.append_task(&d.0, &d.1);
    }
    let mut tasks = get_task_builer.spawn_tier();
    while let Some(task) = tasks.pop() {
        task.run().await?;
    }
    Ok(())
}
