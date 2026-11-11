//! DNS resolver cache flush.

use afina_platform::process;

use crate::{Error, Result};

pub(super) fn flush() -> Result<String> {
    let what = "ipconfig /flushdns";
    let out = process::run_hidden("ipconfig", &["/flushdns"]).map_err(|e| Error::io(what, e))?;
    if out.success { Ok("Cache de DNS limpo.".into()) } else { Err(Error::command(what, out.last_line())) }
}
