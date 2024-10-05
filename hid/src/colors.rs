use crate::hid;
use hidapi::HidDevice;

const NUMBER_OF_KEYS: usize = 71;

pub fn apply_color(device: &HidDevice, color: &str) {
  match parse_color(color) {
    Ok(color) => {
      let color = u32_to_rgb(color);
      let colors: Vec<String> = vec![rgb_to_hex(color.0, color.1, color.2); NUMBER_OF_KEYS];
      loop {
        send_colors(&device, colors.clone());
        std::thread::sleep(std::time::Duration::from_millis(400));
      }
    }
    Err(err) => {
      return
    }
  }
}

fn get_color_chunks(colors: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
  colors
    .chunks(chunk_size)
    .map(|chunk| chunk.to_vec())
    .collect()
}

pub fn is_valid_hex_color(s: &str) -> bool {
  let s = s.trim_start_matches('#');
  if s.len() != 3 && s.len() != 6 {
    return false;
  }
  s.chars().all(|c| c.is_digit(16))
}

pub fn parse_color(color_str: &str) -> Result<u32, String> {
  if !is_valid_hex_color(color_str) {
    return Err("invalid hex color".to_string());
  }
  let mut color_str = color_str.trim_start_matches('#').to_string();
  if color_str.len() == 3 {
    color_str = color_str
      .chars()
      .flat_map(|c| std::iter::repeat(c).take(2))
      .collect();
  }
  u32::from_str_radix(&color_str, 16).map_err(|_| "failed to parse hex color".to_string())
}



fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
  format!("{:02x}{:02x}{:02x}", r, g, b)
}

fn send_colors(device: &HidDevice, colors: Vec<String>) {
  match hid::send_init_packet(&device) {
    Ok(_) => {
      let color_chunks = get_color_chunks(colors, 16);
      for (_i, chunk) in color_chunks.iter().enumerate() {
        let mut req = [0u8; 65];
        req[0x00] = 0x00;
        for (i, color) in chunk.iter().enumerate() {
          let color_value = parse_color(color).unwrap();
          req[(i * 4) + 1] = 0x81;
          req[(i * 4) + 2] = ((color_value >> 16) & 0x000000FF) as u8;
          req[(i * 4) + 3] = ((color_value >> 8) & 0x000000FF) as u8;
          req[(i * 4) + 4] = (color_value & 0x000000FF) as u8;
        }
        hid::send_colors(&device, req);
      }
    }
    Err(e) => {
      std::process::exit(1);
    }
  }
}

pub fn u32_to_rgb(color: u32) -> (u8, u8, u8) {
  let r = ((color >> 16) & 0xff) as u8;
  let g = ((color >> 8) & 0xff) as u8;
  let b = (color & 0xff) as u8;
  (r, g, b)
}
