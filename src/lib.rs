// #[cfg_attr(target_os = "android", ndk_glue::main(logger(level="info", tag="cargo-android-sample")))]
pub fn main() {
    println!("Hello world");
    println!("OS Info: {}", os_info::get());
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_test() {
        assert_eq!(1 + 1, 2);
    }
}
