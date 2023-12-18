#[derive(Debug)]
pub enum LoadError {
    NotFound,
    Lua(mlua::Error),
}

impl LoadError {
    pub fn is_not_found(&self) -> bool {
        matches!(self, LoadError::NotFound)
    }
}

impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for LoadError {}

impl From<mlua::Error> for LoadError {
    fn from(value: mlua::Error) -> Self {
        LoadError::Lua(value)
    }
}
