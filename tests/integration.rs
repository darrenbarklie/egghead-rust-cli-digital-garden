use assert_cmd::Command;
use assert_cmd::assert::Assert;
use color_eyre::eyre::Result;

#[test]
/// Make sure help runs. This indicates the binary works.
fn test_help() -> Result<()> {
  let mut cmd: Command = Command::cargo_bin("garden")?;
  let assert: Assert = cmd.arg("--help").assert();
  assert.success().stderr("");
  Ok(())
}

#[test]
/// Make sure we have a write command by running `garden write --help`
fn test_write_help() -> Result<()> {
  let mut cmd: Command = Command::cargo_bin("garden")?;
  let assert: Assert = cmd.arg("write").arg("--help").assert();
  assert.success().stderr("");
  Ok(())
}

#[test]
#[ignore]
/// Execute the write command, saving a file out
fn test_write() {
  let mut cmd: Command = Command::cargo_bin("garden").unwrap();
  let assert: Assert = cmd.arg("write").assert();
  assert.success();
}