// Enumerating devices:

// use tokio_linux_video::Device;

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let mut devs = Device::list().await?;

//     while let Some(path) = devs.fetch_next().await? {
//         let dev = Device::open(&path).await?;

//         let caps = dev.capabilities().await?;

//         println!("path: {}, {caps}", path.display());
//     }

//     Ok(())
// }

// Getting capabilities and controls:

// use tokio_linux_video::Device;

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let dev = Device::open("/dev/video0").await?;

//     let caps = dev.capabilities().await?;

//     println!("Capabilities: {caps}");

//     println!("Controls:");
//     let mut controls = dev.controls(None);

//     while let Some(ctrl) = controls.fetch_next().await? {
//         println!("  {ctrl}");

//         if let Some(mut items) = dev.control_items(&ctrl) {
//             while let Some(item) = items.fetch_next().await? {
//                 println!("    {item}");
//             }
//         }
//     }

//     Ok(())
// }

// Getting supported formats:

// use tokio_linux_video::{types::BufferType, Device};

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let dev = Device::open("/dev/video0").await?;

//     let caps = dev.capabilities().await?;

//     for type_ in BufferType::ALL {
//         if type_.is_supported(caps.capabilities()) {
//             println!("{type_} formats:");
//             let mut fmts = dev.formats(type_);

//             if let Some(fmt) = fmts.fetch_next().await? {
//                 println!("  {fmt}");

//                 if type_.content().is_video() {
//                     let mut sizes = dev.sizes(fmt.pixel_format());

//                     while let Some(size) = sizes.fetch_next().await? {
//                         println!("    {size}");

//                         for size in size.sizes() {
//                             println!("      {size}");
//                             let mut intervals = dev.intervals(fmt.pixel_format(), size.width(), size.height());

//                             while let Some(interval) = intervals.fetch_next().await? {
//                                 println!("        {interval}");
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Ok(())
// }

// Using controls:

// use tokio_linux_video::{types::*, Device};

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let dev = Device::open("/dev/video0").await?;

//     // Get control from device by identifier
//     let contrast_ctrl = dev.control(CtrlId::Contrast).await?;

//     // Create a value for control
//     let mut contrast = Value::from(&contrast_ctrl);

//     // Get control value from device
//     dev.get_control(&mut contrast).await?;

//     // Get reference to value data
//     let contrast_value = contrast.try_ref::<i32>().unwrap();

//     println!("Current contrast: {contrast_value:?}");

//     // Set new value by reference
//     *contrast.try_mut::<i32>().unwrap() = contrast_value + 10;

//     println!("Updated contrast: {:?}", contrast.try_ref::<i32>().unwrap());

//     // Set new control value to device
//     dev.set_control(&contrast).await?;

//     Ok(())
// }

// Capture video data:

// use tokio_linux_video::{types::*, Device};

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let dev = Device::open("/dev/video0").await?;

//     // Get current format
//     let mut fmt = dev.format(BufferType::VideoCapture).await?;
//     println!("  {fmt}");

//     // Start video capture stream
//     let stream = dev.stream::<In, Mmap>(ContentType::Video, 4)?;

//     let mut i = 0;
//     while let Ok(buffer) = stream.next().await {
//         let buffer = buffer.lock();
//         println!("#{i} {buffer}");

//         // Get reference to frame buffer contents
//         let _data: &[u8] = buffer.as_ref();

//         i += 1;
//         if i > 30 {
//             break;
//         }
//     }

//     Ok(())
// }

// Output video data:

use tokio_linux_video::{types::*, Device};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0").await?;

    // Get current format
    let mut fmt = dev.format(BufferType::VideoOutput).await?;
    println!("  {fmt}");

    // Start video output stream
    let stream = dev.stream::<Out, Mmap>(ContentType::Video, 4)?;

    let mut i = 0;
    while let Ok(mut buffer) = stream.next().await {
        let mut buffer = buffer.lock();
        println!("#{i} {buffer}");

        // Get reference to frame buffer contents
        let _data: &mut [u8] = buffer.as_mut();

        i += 1;
        if i > 30 {
            break;
        }
    }

    Ok(())
}
