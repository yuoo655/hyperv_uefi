#![no_std]
#![no_main]
extern crate uefi;
extern crate alloc;

use alloc::vec::Vec;
use log::info;
use uefi::boot::{self, LoadImageSource};
use uefi::prelude::*;
use uefi::proto::device_path::build::{self, DevicePathBuilder};
use uefi::proto::device_path::{DevicePath, DeviceSubType, DeviceType, LoadedImageDevicePath};
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::BootPolicy;
use uefi::proto::device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly};


/// Get the device path of the linux app. This is the same as the
/// currently-loaded image's device path, but with the file path part changed.
fn get_linux_app_device_path(storage: &mut Vec<u8>) -> &DevicePath {
    let loaded_image_device_path =
        boot::open_protocol_exclusive::<LoadedImageDevicePath>(boot::image_handle())
            .expect("failed to open LoadedImageDevicePath protocol");

    let mut builder = DevicePathBuilder::with_vec(storage);
    for node in loaded_image_device_path.node_iter() {
        if node.full_type() == (DeviceType::MEDIA, DeviceSubType::MEDIA_FILE_PATH) {
            break;
        }
        builder = builder.push(&node).unwrap();
    }
    builder = builder
        .push(&build::media::FilePath {
            path_name: cstr16!(r"efi\boot\bzImage.efi"),
        })
        .unwrap();
    builder.finalize().unwrap()
}

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();

    let mut storage = Vec::new();
    let linux_image_path = get_linux_app_device_path(&mut storage);


    let load_source = LoadImageSource::FromDevicePath {
        device_path: linux_image_path,
        boot_policy: BootPolicy::ExactMatch,
    };
    let linux_image_handle = boot::load_image(boot::image_handle(), load_source).expect("failed to load linux");

    let mut linux_loaded_image = boot::open_protocol_exclusive::<LoadedImage>(linux_image_handle)
        .expect("failed to open LoadedImage protocol");
    let load_options = cstr16!(r"console=tty0 console=ttyS0 video=hyperv_fb:1920x1080");
    unsafe {
        linux_loaded_image.set_load_options(
            load_options.as_ptr().cast(),
            load_options.num_bytes() as u32,
        );
    }
    
    info!("launching the kernel");
    boot::start_image(linux_image_handle).expect("failed to start linux");

    boot::stall(10_000_000);

    Status::SUCCESS
}

