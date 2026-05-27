use std::env;

use cbindgen::Language;

fn main() {
    bundle(Language::C, "h");
    bundle(Language::Cxx, "hxx");
    bundle(Language::Cython, "pxd");
}

fn bundle(language: Language, extension: &str) {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(language)
        .generate()
        .map_or_else(
            |error| match error {
                cbindgen::Error::ParseSyntaxError { .. } => {}
                e => panic!("{:?}", e),
            },
            |bindings| {
                bindings.write_to_file(format!("headers/libduckity_c.{extension}"));
            },
        )
}
