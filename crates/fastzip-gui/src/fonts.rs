//! 中文字体加载，解决界面中文乱码

use std::path::Path;

use egui::{FontData, FontDefinitions};

/// 返回当前平台可能的中文字体路径（按优先级）
fn chinese_font_paths() -> Vec<std::path::PathBuf> {
    #[cfg(windows)]
    {
        let system_root = std::env::var("SYSTEMROOT").unwrap_or_else(|_| "C:\\Windows".to_string());
        let fonts = format!("{}\\Fonts", system_root);
        vec![
            std::path::PathBuf::from(&fonts).join("msyh.ttc"),   // 微软雅黑
            std::path::PathBuf::from(&fonts).join("msyhbd.ttc"),  // 微软雅黑 Bold
            std::path::PathBuf::from(&fonts).join("simsun.ttc"),  // 宋体
            std::path::PathBuf::from(&fonts).join("simhei.ttf"),   // 黑体
        ]
    }

    #[cfg(target_os = "macos")]
    {
        vec![
            std::path::PathBuf::from("/System/Library/Fonts/PingFang.ttc"),
            std::path::PathBuf::from("/System/Library/Fonts/STHeiti Light.ttc"),
            std::path::PathBuf::from("/System/Library/Fonts/Supplemental/Arial Unicode.ttf"),
        ]
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        vec![
            std::path::PathBuf::from("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"),
            std::path::PathBuf::from("/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc"),
            std::path::PathBuf::from("/usr/share/fonts/wqy-microhei/wqy-microhei.ttc"),
        ]
    }

    #[cfg(not(any(windows, target_os = "macos", unix)))]
    {
        vec![]
    }
}

/// 尝试从路径加载字体字节；TTC 使用 index 0
fn load_font_bytes(path: &Path) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}

/// 配置 egui 使用中文字体，避免中文显示为方框
/// 将系统中文字体加入默认字体的 fallback 列表首位，使 CJK 字符优先用该字体渲染
pub fn setup_chinese_fonts(ctx: &egui::Context) {
    let paths = chinese_font_paths();
    for path in paths {
        if path.exists() {
            if let Some(bytes) = load_font_bytes(&path) {
                let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
                let font_data = FontData::from_static(leaked).tweak(egui::FontTweak {
                    scale: 1.0,
                    ..Default::default()
                });
                const FONT_NAME: &str = "chinese_fallback";
                let mut fonts = FontDefinitions::default();
                fonts.font_data.insert(FONT_NAME.to_owned(), font_data);
                // 放在列表首位，优先用中文字体渲染（对 CJK 有效；西文常用字体也含拉丁字母）
                for (_family, list) in fonts.families.iter_mut() {
                    list.insert(0, FONT_NAME.to_owned());
                }
                ctx.set_fonts(fonts);
                return;
            }
        }
    }
}
