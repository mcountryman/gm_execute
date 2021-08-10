use mlua::{ExternalResult, Lua};
use std::process::{Command, Stdio};

/// Executes supplied command on shell and returns a string matching either `stdout` if
/// command completed with a 0 status code or `stderr` if non-zero.
///
/// # Arguments
/// * `lua` - The lua state (not used but, required for lua callbacks)
/// * `command` - The command to execute.
pub fn os_execute(lua: &Lua, command: String) -> mlua::Result<String> {
  os_execute_imp(lua, command)
}

#[inline]
#[cfg(unix)]
fn os_execute_imp(_: &Lua, command: String) -> mlua::Result<String> {
  let child = Command::new("sh")
    .arg("-c")
    .arg(command)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .to_lua_err()?;

  let output = child.wait_with_output().to_lua_err()?;
  let output = if output.status.success() {
    output.stdout
  } else {
    output.stderr
  };

  Ok(String::from_utf8_lossy(&output).to_string())
}

#[inline]
#[cfg(windows)]
fn os_execute_imp(_: &Lua, command: String) -> mlua::Result<String> {
  let child = Command::new("cmd.exe")
    .arg("/c")
    .arg(command)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .to_lua_err()?;

  let output = child.wait_with_output().to_lua_err()?;
  let output = if output.status.success() {
    output.stdout
  } else {
    output.stderr
  };

  Ok(String::from_utf8_lossy(&output).to_string())
}

#[inline]
#[cfg(all(not(unix), not(windows)))]
fn os_execute_imp(_: &Lua, _: String) -> mlua::Result<String> {
  Err(anyhow::anyhow!("Execute not supported on this system.")).to_lua_err()
}

#[cfg(test)]
mod test {
  use super::os_execute;
  use mlua::Lua;

  #[test]
  fn test_execute_captures_stdout() {
    let lua = Lua::new();
    let stdout = os_execute(&lua, "echo test".to_string()).unwrap();
    let stdout = stdout.trim_end();

    assert_eq!(stdout, "test");
  }

  #[test]
  fn test_execute_captures_stderr() {
    let lua = Lua::new();
    let stderr = os_execute(&lua, "echo test 1>&2 && exit 1".to_string()).unwrap();
    let stderr = stderr.trim_end();

    assert_eq!(stderr, "test");
  }
}
