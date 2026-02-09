//! 各压缩格式适配器

mod detect;
mod single;
mod zip_format;
mod sevenz_format;
mod tar_format;

#[cfg(feature = "unrar")]
mod rar_format;

pub use detect::{detect_format, ArchiveFormat};
pub use single::extract_single_compressed;
pub use zip_format::ZipExtractor;
pub use sevenz_format::SevenZExtractor;
pub use tar_format::TarExtractor;

#[cfg(feature = "unrar")]
pub use rar_format::RarExtractor;
