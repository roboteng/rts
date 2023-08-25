use dagger_sdk::HostDirectoryOpts;

const PROJECT: &str = "/project";
const PROJECT_TARGET: &str = "/project/target";

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

    let build_cache = client.cache_volume("target").id().await?;
    let cargo_registry = client.cache_volume("cargo").id().await?;
    let cargo_bin_registry = client.cache_volume("global bin").id().await?;

    let container = client
        .container()
        .from("rust:1.71.1")
        .with_mounted_directory(PROJECT, host_source_dir.id().await?)
        .with_mounted_cache(PROJECT_TARGET, build_cache);

    let cargo_home = container.env_variable("CARGO_HOME").await?;

    let container = container.with_mounted_cache(format!("{cargo_home}/registry"), cargo_registry);
    let container = container.with_mounted_cache(format!("{cargo_home}/bin"), cargo_bin_registry);

    let container = container
        .with_workdir(PROJECT)
        .with_exec(vec!["cargo", "check"]);

    let container = container
        .with_workdir(PROJECT)
        .with_exec(vec!["cargo", "test"]);

    let container =
        container
            .with_workdir(PROJECT)
            .with_exec(vec!["rustup", "component", "add", "clippy"]);

    let container = container
        .with_workdir(PROJECT)
        .with_exec(vec!["cargo", "clippy", "--all", "--", "-D", "warnings"]);

    container
        .with_workdir(PROJECT)
        .with_exec(vec!["cargo", "build", "-p", "sample"])
        .stderr()
        .await?;

    Ok(())
}
