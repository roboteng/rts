use dagger_sdk::HostDirectoryOpts;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;

    let host_source_dir = client.host().directory_opts(
        ".",
        HostDirectoryOpts {
            exclude: Some(vec!["target".into()]),
            include: None,
        },
    );

    let container = client
        .container()
        .from("rust:1.71.1")
        .with_mounted_directory("/src", host_source_dir.id().await?);

    let check = container
        .with_workdir("/src")
        .with_exec(vec!["cargo", "check"])
        .stdout()
        .await?;

    println!("Hello from Dagger and {}", check.trim());

    Ok(())
}
