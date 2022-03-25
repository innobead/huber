use std::thread::sleep;
use std::time::Duration;

use libcli_rs::progress::{ProgressBar, ProgressTrait};

use crate::result::Result;

pub fn progress(msg: &str) -> Result<()> {
    progress!(msg, sleep(Duration::from_millis(300)));
    Ok(())
}
