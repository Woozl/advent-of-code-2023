use std::process::{Command, self};

// Get a temporary file name that is hopefully unique
#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    format!("./temp_{}_{thread_id}", process::id())
}

pub fn run_challenge(day: usize, part: usize) -> Result<(), Box<dyn std::error::Error>> {
  let cmd = Command::new("rustc")
    .args(["./challenges/day-1/part-1", "-o", &temp_file()])
    .output()?;

  if cmd.status.success() {
    
  }
  
  Ok(())
}