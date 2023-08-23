use dagger_sdk::HostDirectoryOpts;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;

    let host_source_dir = client.host().directory_opts(
        ".",
        HostDirectoryOpts {
            exclude: Some(vec!["target"]),
            include: None,
        },
    );

    let cache = client.cache_volume("deps").id().await?;

    let container = client
        .container()
        .from("rust:1.71.1")
        .with_mounted_directory("/src", host_source_dir.id().await?)
        .with_mounted_cache("/target", cache);

    container
        .with_workdir("/src")
        .with_exec(vec!["cargo", "check"])
        .stdout()
        .await?;

    let runner = container
        .with_workdir("/src")
        .with_exec(vec!["cargo", "test"]);

    let runner =
        runner
            .with_workdir("/src")
            .with_exec(vec!["rustup", "component", "add", "clippy"]);

    let runner = runner
        .with_workdir("/src")
        .with_exec(vec!["cargo", "clippy"]);

    runner
        .with_workdir("/src")
        .with_exec(vec!["cargo", "build", "-p", "sample"])
        .stdout()
        .await?;

    println!("Everything is A-Ok!");
    Ok(())
}
