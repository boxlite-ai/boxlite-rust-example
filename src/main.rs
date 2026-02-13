use boxlite::{BoxCommand, BoxOptions, BoxliteRuntime, RootfsSpec};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = BoxliteRuntime::default_runtime();
    let options = BoxOptions {
        rootfs: RootfsSpec::Image("alpine:latest".into()),
        ..Default::default()
    };

    let litebox = runtime.create(options, None).await?;
    let mut execution = litebox
        .exec(BoxCommand::new("echo").arg("Hello from BoxLite!"))
        .await?;

    let mut stdout = execution.stdout().unwrap();
    while let Some(line) = stdout.next().await {
        println!("{}", line);
    }

    Ok(())
}
