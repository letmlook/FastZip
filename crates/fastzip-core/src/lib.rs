//! FastZip 核心库：跨平台快速解压缩

pub mod error;
pub mod extractor;
pub mod formats;
pub mod path_utils;
pub mod smart_dest;

pub mod compress;

pub use compress::{compress_to_zip, compress_to_7z, CompressOptions};
pub use error::{FastZipError, Result};
pub use extractor::{extract_many, extract_one, ExtractOptions};
pub use formats::{detect_format, ArchiveFormat};
pub use path_utils::normalize_entry_path;
pub use smart_dest::{list_archive_top_level, resolve_smart_dest, TopLevelEntries};
