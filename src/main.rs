use post_gis::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Just try to create a connection pool
    post_gis::get_connection_pool().await?;
    Ok(())
}
