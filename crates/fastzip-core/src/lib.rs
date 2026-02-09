//! FastZip 核心库：跨平台快速解压缩

pub mod error;
pub mod extractor;
pub mod formats;
pub mod smart_dest;

pub use error::{FastZipError, Result};
pub use extractor::{extract_many, extract_one, ExtractOptions};
pub use formats::{detect_format, ArchiveFormat};
pub use smart_dest::{resolve_smart_dest, TopLevelEntries};
