#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;

    let version = client
        .container()
        .from("rust:1.71.1")
        .with_exec(vec!["cargo", "version"])
        .stdout()
        .await?;

    println!("Hello from Dagger and {}", version.trim());

    Ok(())
}
