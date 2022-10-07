use cornucopia::{CodegenSettings, Error};

fn main() -> Result<(), Error> {
    let queries_path = "queries";
    let schema_file = "schema.sql";
    let destination = "src/cornucopia.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");
    cornucopia::generate_managed(
        queries_path,
        vec![schema_file.into()],
        Some(destination),
        false,
        settings,
    )?;

    Ok(())
}
