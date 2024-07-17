fn main() {
    pkg_config::Config::new()
        // .atleast_version("1.26.1") // FIXME find an appropriate version
        .probe("libudev")
        .expect("Failed to link to libudev");
}
