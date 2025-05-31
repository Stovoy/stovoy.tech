use once_cell::sync::Lazy;

include!(concat!(env!("OUT_DIR"), "/file_map.rs"));

pub static SOURCE_FILES: Lazy<std::collections::HashMap<&'static str, &'static str>> =
    Lazy::new(|| build_file_map());
