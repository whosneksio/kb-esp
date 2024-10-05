use hidapi::{HidApi, HidDevice};
use std::process;

#[cfg(target_os = "windows")]
pub const DEV_USAGE: u16 = 0xFF00;

#[cfg(target_os = "windows")]
pub const DEV_USAGE_PAGE: u16 = 0xFF90;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const DEV_USAGE: u16 = 0x6;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const DEV_USAGE_PAGE: u16 = 0x1;

pub fn get_device(vendor_id: u16, product_id: u16) -> Option<HidDevice> {
  let api = HidApi::new().expect("failed to initialize hid api");
  for dev in api.device_list() {
    if dev.vendor_id() == vendor_id
      && dev.product_id() == product_id
      && dev.usage() == DEV_USAGE
      && dev.usage_page() == DEV_USAGE_PAGE
    {
      if let Ok(device) = dev.open_device(&api) {
        return Some(device);
      }
    }
  }
  eprintln!("device not found");
  return None;
}

pub fn send_init_packet(device: &HidDevice) -> Result<(), String> {
  let mut req = [0u8; 65];
  req[0x00] = 0x00;
  req[0x01] = 0x04;
  req[0x02] = 0xF2;
  req[0x09] = 0x05;
  match device.send_feature_report(&req) {
    Ok(_) => Ok(()),
    Err(e) => Err(format!("failed to send feature report: {}", e)),
  }
}

pub fn send_colors(device: &HidDevice, colors: [u8; 65]) {
  match device.send_feature_report(&colors) {
    Ok(_) => (),
    Err(e) => {
      eprintln!("failed to send feature report: {}", e);
      process::exit(1);
    }
  }
}
