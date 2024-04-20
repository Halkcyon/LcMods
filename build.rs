use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

fn main() {
    println!("cargo::rerun-if-changed=BepInExPack");

    let mods_zip = PathBuf::from("src/mods.zip");
    let file = BufWriter::new(File::create(mods_zip).expect("unable to create mods.zip"));
    let mut zip = ZipWriter::new(file);

    let file_opts = FileOptions::default()
        .compression_method(zip::CompressionMethod::Zstd)
        .compression_level(Some(22));
    for entry in WalkDir::new("BepInExPack").into_iter().skip(1) {
        let entry = entry.expect("unable to read file?");
        let path = entry.path();
        let name = path
            .to_str()
            .unwrap()
            .strip_prefix("BepInExPack\\")
            .unwrap();

        if entry.file_type().is_dir() {
            zip.add_directory(name, file_opts).unwrap();
        } else {
            zip.start_file(name, file_opts).unwrap();
            let file = fs::read(path).unwrap();
            zip.write_all(&file).unwrap();
        }
    }
}
