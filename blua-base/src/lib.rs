use std::path::Path;

#[cfg(feature = "crypto")]
pub mod crypto;
pub mod date;
pub mod json;
#[cfg(feature = "regexp")]
pub mod regexp;

pub fn write_definitions(path: &Path) -> std::io::Result<()> {
    Ok(())
}
