//! Helper and utility functions.

use {
  color_eyre::Result,
  subprocess::{Exec, NullFile},
};

/// Run a command using [`subprocess`] and discard the output.
pub fn shell_command(command: String) -> Result<()> {
  Exec::shell(command)
    .stdout(NullFile)
    .stderr(NullFile)
    .capture()?;

  Ok(())
}
