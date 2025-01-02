use std::thread::sleep;
use std::time::Duration;

use libcli_rs::progress;
use libcli_rs::progress::{ProgressBar, ProgressTrait};

pub fn progress(msg: &str) -> anyhow::Result<()> {
    progress!(msg, sleep(Duration::from_millis(300)));
    Ok(())
}
