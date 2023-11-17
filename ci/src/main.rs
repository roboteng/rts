use dagger_sdk::HostDirectoryOpts;

const PROJECT: &str = "/project";
const PROJECT_TARGET: &str = "/project/target";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;

    let host_source_dir = client.host().directory_opts(
        ".",
        HostDirectoryOpts {
            exclude: Some(vec!["target", ".obsidian"]),
            include: None,
        },
    );

    let build_cache = client.cache_volume("target").id().await?;
    let cargo_registry = client.cache_volume("cargo").id().await?;

    let container = client
        .container()
        .from("rust:1.74.0")
        .with_mounted_directory(PROJECT, host_source_dir.id().await?)
        .with_mounted_cache(PROJECT_TARGET, build_cache);

    let cargo_home = container.env_variable("CARGO_HOME").await?;

    container
        .with_mounted_cache(format!("{cargo_home}/registry"), cargo_registry)
        .with_workdir(PROJECT)
        .with_exec("apt-get update".split(' ').collect())
        .with_exec(
            "apt-get install -y --no-install-recommends libasound2-dev libudev-dev"
                .split(' ')
                .collect(),
        )
        .with_exec(vec!["cargo", "check"])
        .with_exec(vec!["cargo", "test"])
        .with_exec(vec!["rustup", "component", "add", "clippy"])
        .with_exec(vec!["cargo", "clippy", "--all", "--", "-D", "warnings"])
        .stderr()
        .await?;

    Ok(())
}
