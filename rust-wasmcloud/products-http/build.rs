#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=./src");
    println!("cargo:rerun-if-changed=./Cargo.toml");

    wit_deps::lock!().await.unwrap();
    Ok(())
}
