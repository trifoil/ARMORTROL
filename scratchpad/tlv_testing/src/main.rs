use tokio_linux_video::Device;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut devs = Device::list().await?;

    while let Some(path) = devs.fetch_next().await? {
        let dev = Device::open(&path).await?;

        let caps = dev.capabilities().await?;

        println!("path: {}, {caps}", path.display());
    }

    Ok(())
}