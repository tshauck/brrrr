// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("brrrr")?;

    cmd.arg("help");
    cmd.assert().success();

    Ok(())
}
