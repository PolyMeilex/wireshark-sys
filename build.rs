use std::env;

fn main() {
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    #[cfg(feature = "bindgen")]
    generate_bindings();

    pkg_config::probe_library("wireshark").unwrap();
}

#[cfg(feature = "bindgen")]
fn parse_version(version: &str) -> (u32, u32, u32) {
    let mut split = version.split('.');
    (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-DHAVE_PLUGINS")
        .generate_comments(false)
        .prepend_enum_name(false)
        .blocklist_type("gboolean")
        .raw_line("pub type gboolean = bool;")
        .raw_line("use glib_sys::*;")
        .blocklist_file(".*glib.*")
        .layout_tests(false);

    for item in [
        "value_string",
        "hf_register_info",
        "ftenum",
        "field_display_e",
        "tcp_dissect_pdus",
        "dissector_add_uint",
        "register_dissector",
        "proto_.*",
        "col_.*",
        "tvb_.*",
        "WIRESHARK_VERSION_.*",
        "COL_.*",
        "ENC_.*",
    ] {
        builder = builder.allowlist_item(item);
    }

    let libws = pkg_config::probe_library("wireshark").expect("wireshark headers not found");
    let (major, minor, micro) = parse_version(&libws.version);

    // Version 2 did not support those consts, but is has to be supported to make life of RHEL8 users easier
    if major == 2 {
        builder = builder.raw_line(format!("pub const WIRESHARK_VERSION_MAJOR: u32 = {major};"));
        builder = builder.raw_line(format!("pub const WIRESHARK_VERSION_MINOR: u32 = {minor};"));
        builder = builder.raw_line(format!("pub const WIRESHARK_VERSION_MICRO: u32 = {micro};"));
    }

    for path in libws.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.to_string_lossy()));
    }

    let bindings = builder
        .generate()
        .expect("should be able to generate bindings from wrapper.h");

    let out_path = std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("src").join("bindings.rs"))
        .expect("generated bindings should be written to file");
}
