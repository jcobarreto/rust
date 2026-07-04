use std::time::Duration;
use std::thread::sleep;

pub fn clear_screen() {
  clearscreen::clear().expect("Failed to clear the screen");
}

pub fn wait_for(time: u64) {
  sleep(Duration::from_secs(time));
}
