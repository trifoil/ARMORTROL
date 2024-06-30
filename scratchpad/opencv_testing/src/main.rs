// // src/main.rs

// use v4l::prelude::*;
// use v4l::video::Capture;
// use image::{ImageBuffer, Rgb};
// use minifb::{Key, Window, WindowOptions};

// fn main() {
//     // Open the camera device
//     let path = "/dev/video0";
//     let dev = Device::new(path).expect("Failed to open video device");

//     // Create a new stream
//     let mut stream = Stream::with_buffers(&dev, Type::VideoCapture, 4).expect("Failed to create stream");

//     // Get the format of the stream
//     let format = dev.format().expect("Failed to get format");
//     let width = format.width;
//     let height = format.height;
    
//     // Create a window
//     let mut window = Window::new(
//         "Camera Display",
//         width as usize,
//         height as usize,
//         WindowOptions::default(),
//     ).expect("Unable to open window");

//     // Capture frames
//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         let (buf, _) = stream.next().expect("Failed to capture frame");

//         // Create an image buffer from the frame
//         let img = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, buf).expect("Failed to create image buffer");

//         // Convert the image buffer to raw data
//         let raw: Vec<u32> = img.pixels().map(|p| {
//             let Rgb(data) = *p;
//             let (r, g, b) = (data[0], data[1], data[2]);
//             ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
//         }).collect();

//         // Update the window with the new frame
//         window.update_with_buffer(&raw, width as usize, height as usize).expect("Failed to update window");
//     }
// }


// src/main.rs

use std::fs::File;
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread::sleep;
use std::time::Duration;

use libc::{self, c_void, mmap, munmap, open, O_RDWR};
use libc::{close, ioctl, MAP_SHARED, PROT_READ, PROT_WRITE};

const VIDEO_DEVICE: &str = "/dev/video0";
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

#[repr(C)]
struct V4l2Format {
    type_: u32,
    width: u32,
    height: u32,
    pixel_format: u32,
    field: u32,
    bytes_per_line: u32,
    size_image: u32,
    colorspace: u32,
    priv_: u32,
}

#[repr(C)]
struct V4l2RequestBuffers {
    count: u32,
    type_: u32,
    memory: u32,
}

#[repr(C)]
struct V4l2Buffer {
    index: u32,
    type_: u32,
    bytesused: u32,
    flags: u32,
    field: u32,
    timestamp: libc::timeval,
    timecode: [u32; 8],
    sequence: u32,
    memory: u32,
    m: [u32; 2],
    length: u32,
    input: u32,
    reserved: [u32; 2],
}

const VIDIOC_QUERYCAP: u32 = 0x80685600;
const VIDIOC_S_FMT: u32 = 0xC0D05605;
const VIDIOC_REQBUFS: u32 = 0xC0145608;
const VIDIOC_QUERYBUF: u32 = 0xC0445609;
const VIDIOC_QBUF: u32 = 0xC044560F;
const VIDIOC_DQBUF: u32 = 0xC0445611;
const VIDIOC_STREAMON: u32 = 0xC0045612;
const VIDIOC_STREAMOFF: u32 = 0xC0045613;

fn main() -> io::Result<()> {
    // Open the video device
    let fd = unsafe { open(VIDEO_DEVICE.as_ptr() as *const i8, O_RDWR) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }

    // Set the video format
    let mut fmt = V4l2Format {
        type_: 2, // V4L2_BUF_TYPE_VIDEO_CAPTURE
        width: WIDTH,
        height: HEIGHT,
        pixel_format: 1448695129, // V4L2_PIX_FMT_MJPEG
        field: 0,
        bytes_per_line: 0,
        size_image: 0,
        colorspace: 0,
        priv_: 0,
    };
    if unsafe { ioctl(fd, VIDIOC_S_FMT, &mut fmt) } < 0 {
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Request buffer
    let mut req = V4l2RequestBuffers {
        count: 1,
        type_: 2, // V4L2_BUF_TYPE_VIDEO_CAPTURE
        memory: 1, // V4L2_MEMORY_MMAP
    };
    if unsafe { ioctl(fd, VIDIOC_REQBUFS, &mut req) } < 0 {
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Query buffer
    let mut buf = V4l2Buffer {
        index: 0,
        type_: 2, // V4L2_BUF_TYPE_VIDEO_CAPTURE
        bytesused: 0,
        flags: 0,
        field: 0,
        timestamp: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        timecode: [0; 8],
        sequence: 0,
        memory: 1, // V4L2_MEMORY_MMAP
        m: [0; 2],
        length: 0,
        input: 0,
        reserved: [0; 2],
    };
    if unsafe { ioctl(fd, VIDIOC_QUERYBUF, &mut buf) } < 0 {
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Memory map the buffer
    let buffer = unsafe {
        mmap(
            ptr::null_mut(),
            buf.length as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            buf.m[0] as i64,
        )
    } as *mut u8;
    if buffer == libc::MAP_FAILED {
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Queue buffer
    if unsafe { ioctl(fd, VIDIOC_QBUF, &mut buf) } < 0 {
        unsafe { munmap(buffer as *mut c_void, buf.length as usize) };
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Start streaming
    let type_ = 2; // V4L2_BUF_TYPE_VIDEO_CAPTURE
    if unsafe { ioctl(fd, VIDIOC_STREAMON, &type_) } < 0 {
        unsafe { munmap(buffer as *mut c_void, buf.length as usize) };
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Capture one frame
    if unsafe { ioctl(fd, VIDIOC_DQBUF, &mut buf) } < 0 {
        unsafe { munmap(buffer as *mut c_void, buf.length as usize) };
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Save the frame to a file
    let mut file = File::create("frame.jpg")?;
    let data = unsafe { slice::from_raw_parts(buffer, buf.bytesused as usize) };
    file.write_all(data)?;

    // Clean up
    unsafe { munmap(buffer as *mut c_void, buf.length as usize) };
    unsafe { close(fd) };

    println!("Frame captured and saved to frame.jpg");

    Ok(())
}
