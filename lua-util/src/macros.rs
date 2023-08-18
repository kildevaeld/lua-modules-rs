#[macro_export]
macro_rules! definition {
    ($($const: ident($name: literal) = $source: literal)+) => {
        $(
            pub const $const: &[u8] = include_bytes!($source);
        )+

        pub fn write_definition(path: &std::path::Path) -> std::io::Result<()> {
            $(
                let file_path = path.join(format!("{}.lua", $name));
                std::fs::write(file_path, $const)?;
            )+

            Ok(())
        }
    };
}
