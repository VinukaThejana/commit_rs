use commit_rs::config::log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::setup();

    anyhow::Ok(())
}
