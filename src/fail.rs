macro_rules! fail {
    ($n:ident $($a:ident)+ = $m:literal) => {
        pub fn $n($( $a: impl std::fmt::Display ),+) -> impl FnOnce() -> String {
            move || format!($m, $( $a ),+)
        }
    };
}

fail!(copy_file from to = "Failed to copy file {} to {}");
fail!(create_dir path = "Failed to create directory {}");
fail!(create_file path = "Failed to create file {}");
fail!(parse_cfg path = "Failed to parse configuration file {}");
fail!(parse_glob pat = "Failed to parse glob pattern {}");
fail!(upload name ver = "Failed to upload {} version {}");
fail!(read path = "Failed to read file {}");
fail!(remove_dir path = "Failed to remove directory {}");
fail!(remove_file path = "Failed to remove file {}");
fail!(set_dir path = "Failed to set working directory to {}");
fail!(traverse path = "Failed when traversing the source directory {}");
