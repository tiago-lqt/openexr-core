mod attributes;
mod decode;
mod errors;
mod file;
mod version;

mod wrappers;

pub use attributes::*;
pub use decode::*;
pub use errors::*;
pub use file::*;
pub use version::*;

pub use wrappers::*;

pub fn read_image_pixels(path: &str) -> anyhow::Result<()> {
    let file = open_file(path)?;

    let parts = file.parts_count()?;

    println!("Exr file '{}' has '{}' parts", path, parts);

    if parts > 1 {
        panic!("Multipart EXR not supported yet");
    }

    let part_name = file.get_part_name(0)?;
    println!("Part 0 name: '{}'", part_name);

    let display_window = file.get_display_window(0);
    println!(
        "Part 0 has display window min: '{:?}', max: '{:?}'",
        display_window.min, display_window.max
    );

    let data_window = file.get_data_window(0);
    println!(
        "Part 0 has data window min: '{:?}', max: '{:?}'",
        data_window.min, data_window.max
    );

    let storage_type = file.get_part_storage(0)?;
    println!("Part 0 has storage type: '{:?}'", storage_type);

    match storage_type {
        StorageType::Scanline => read_scanline_storage(file),
        StorageType::Tiled => read_tiled_storage(file),
        StorageType::DeepScanline => read_deep_scanline_storage(file),
        StorageType::DeepTiled => read_deep_tiled_storage(file),
    }
}

fn read_scanline_storage(file: ExrFile) -> anyhow::Result<()> {
    let chunks = file.get_chunk_count(0)?;
    println!("Part 0 has '{}' chunks", chunks);

    let scalines = file.get_scanlines_per_chunk(0)?;
    println!("Part 0 has '{}' lines per chunk", scalines);

    todo!()
}

fn read_deep_tiled_storage(_file: ExrFile) -> anyhow::Result<()> {
    todo!()
}

fn read_deep_scanline_storage(_file: ExrFile) -> anyhow::Result<()> {
    todo!()
}

fn read_tiled_storage(_file: ExrFile) -> anyhow::Result<()> {
    todo!()
}
