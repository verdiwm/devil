fn main() {
    pkg_config::Config::new()
        .atleast_version("221")
        .probe("libudev")
        .expect("Failed to link to libudev");

    // pkg_config::probe_library("libudev").expect("Failed to link libudev");
}
