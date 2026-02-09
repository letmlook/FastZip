//! 主界面逻辑与状态

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

use eframe::egui;
use fastzip_core::{compress_to_7z, compress_to_zip, extract_one, CompressOptions, ExtractOptions};

use crate::archive_preview;

/// 界面模式
#[derive(Default, PartialEq)]
enum Tab {
    #[default]
    Extract,
    Compress,
}

/// 任务结果
enum TaskResult {
    Extract(Result<PathBuf, String>),
    Compress(Result<(), String>),
}

pub struct FastZipApp {
    tab: Tab,

    // 解压
    archive_path: String,
    dest_path: String,
    smart_extract: bool,
    password: String,
    preview_entries: Vec<String>,
    preview_format: String,

    // 压缩
    compress_sources: Vec<PathBuf>,
    compress_dest: String,
    compress_recursive: bool,
    compress_format_zip: bool,

    status: String,
    error: String,
    running: bool,
    rx: Option<mpsc::Receiver<TaskResult>>,
}

impl Default for FastZipApp {
    fn default() -> Self {
        Self {
            tab: Tab::Extract,
            archive_path: String::new(),
            dest_path: String::new(),
            smart_extract: true,
            password: String::new(),
            preview_entries: Vec::new(),
            preview_format: String::new(),
            compress_sources: Vec::new(),
            compress_dest: String::new(),
            compress_recursive: true,
            compress_format_zip: true,
            status: String::new(),
            error: String::new(),
            running: false,
            rx: None,
        }
    }
}

impl FastZipApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn poll_result(&mut self) {
        if !self.running {
            return;
        }
        let rx = match &self.rx {
            Some(r) => r,
            None => return,
        };
        match rx.try_recv() {
            Ok(TaskResult::Extract(Ok(dest))) => {
                self.status = format!("已解压到: {}", dest.display());
                self.error.clear();
                self.running = false;
                self.rx = None;
            }
            Ok(TaskResult::Extract(Err(e))) => {
                self.error = e;
                self.status.clear();
                self.running = false;
                self.rx = None;
            }
            Ok(TaskResult::Compress(Ok(()))) => {
                self.status = "压缩完成".to_string();
                self.error.clear();
                self.running = false;
                self.rx = None;
            }
            Ok(TaskResult::Compress(Err(e))) => {
                self.error = e;
                self.status.clear();
                self.running = false;
                self.rx = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.running = false;
                self.rx = None;
            }
        }
    }

    fn start_extract(&mut self) {
        if self.archive_path.is_empty() || self.dest_path.is_empty() {
            self.error = "请选择压缩包和目标目录".to_string();
            return;
        }
        let archive = PathBuf::from(&self.archive_path);
        let dest = PathBuf::from(&self.dest_path);
        if !archive.exists() {
            self.error = "压缩包不存在".to_string();
            return;
        }
        let smart = self.smart_extract;
        let password = if self.password.is_empty() {
            None
        } else {
            Some(self.password.clone())
        };
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        self.running = true;
        self.error.clear();
        self.status = "正在解压...".to_string();
        thread::spawn(move || {
            let opts = ExtractOptions {
                dest: Some(dest.clone()),
                smart,
                overwrite: false,
                password,
            };
            let result = extract_one(&archive, &opts).map_err(|e| e.to_string());
            let _ = tx.send(TaskResult::Extract(result));
        });
    }

    fn start_compress(&mut self) {
        if self.compress_sources.is_empty() || self.compress_dest.is_empty() {
            self.error = "请添加要压缩的文件并指定输出路径".to_string();
            return;
        }
        let dest = PathBuf::from(&self.compress_dest);
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        self.running = true;
        self.error.clear();
        self.status = "正在压缩...".to_string();
        let sources = self.compress_sources.clone();
        let recursive = self.compress_recursive;
        let is_zip = self.compress_format_zip;
        thread::spawn(move || {
            let result = if is_zip {
                compress_to_zip(&sources, &dest, &CompressOptions { recursive, password: None, fast: true })
                    .map_err(|e| e.to_string())
            } else {
                if sources.len() > 1 {
                    let _ = tx.send(TaskResult::Compress(Err(
                        "7z 仅支持单一路径".to_string(),
                    )));
                    return;
                }
                compress_to_7z(&sources[0], &dest).map_err(|e| e.to_string())
            };
            let _ = tx.send(TaskResult::Compress(result));
        });
    }

    fn load_preview(&mut self) {
        if self.archive_path.is_empty() {
            return;
        }
        let path = Path::new(&self.archive_path);
        match archive_preview::list_top_level(path) {
            Ok((format_name, entries)) => {
                self.preview_format = format_name;
                self.preview_entries = entries;
                self.error.clear();
            }
            Err(e) => {
                self.error = format!("{}", e);
                self.preview_entries.clear();
                self.preview_format.clear();
            }
        }
    }
}

impl eframe::App for FastZipApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_result();

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Extract, "解压");
                ui.selectable_value(&mut self.tab, Tab::Compress, "压缩");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Extract => self.ui_extract(ui),
                Tab::Compress => self.ui_compress(ui),
            }
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.running {
                    ui.spinner();
                    ui.label(&self.status);
                } else if !self.status.is_empty() {
                    ui.label(egui::RichText::new(&self.status).color(egui::Color32::GREEN));
                }
                if !self.error.is_empty() {
                    ui.label(egui::RichText::new(&self.error).color(egui::Color32::RED));
                }
            });
        });
    }
}

impl FastZipApp {
    fn ui_extract(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(12.0);
            ui.heading("解压压缩包");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("压缩包:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.archive_path)
                        .desired_width(320.0)
                        .hint_text("路径或点击浏览"),
                );
                if ui.button("浏览…").clicked() {
                    if let Some(p) = rfd::FileDialog::new().pick_file() {
                        self.archive_path = p.to_string_lossy().to_string();
                        self.load_preview();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("目标目录:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.dest_path)
                        .desired_width(320.0)
                        .hint_text("解压到此目录"),
                );
                if ui.button("浏览…").clicked() {
                    if let Some(p) = rfd::FileDialog::new().pick_folder() {
                        self.dest_path = p.to_string_lossy().to_string();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.smart_extract, "智能解压（根据内容自动选择子目录）");
            });

            ui.horizontal(|ui| {
                ui.label("密码:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.password)
                        .desired_width(180.0)
                        .password(true)
                        .hint_text("可选"),
                );
            });

            if !self.preview_format.is_empty() {
                ui.add_space(6.0);
                ui.label(egui::RichText::new(format!("格式: {}", self.preview_format)).small());
                if !self.preview_entries.is_empty() {
                    egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                        for name in self.preview_entries.iter().take(50) {
                            ui.label(egui::RichText::new(name).small());
                        }
                        if self.preview_entries.len() > 50 {
                            ui.label(egui::RichText::new("...").small());
                        }
                    });
                }
            }

            ui.add_space(12.0);
            let can_extract = !self.running && !self.archive_path.is_empty() && !self.dest_path.is_empty();
            if ui
                .add_enabled(can_extract, egui::Button::new("解压"))
                .clicked()
            {
                self.start_extract();
            }
        });
    }

    fn ui_compress(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(12.0);
            ui.heading("压缩文件/目录");
            ui.add_space(8.0);

            if ui.button("添加文件或目录…").clicked() {
                let files = rfd::FileDialog::new().pick_files();
                if let Some(f) = files {
                    self.compress_sources.extend(f);
                }
            }

            if !self.compress_sources.is_empty() {
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    let mut to_remove = None;
                    for (i, p) in self.compress_sources.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(p.display().to_string());
                            if ui.small_button("移除").clicked() {
                                to_remove = Some(i);
                            }
                        });
                    }
                    if let Some(i) = to_remove {
                        self.compress_sources.remove(i);
                    }
                });
            }

            ui.horizontal(|ui| {
                ui.label("输出路径:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.compress_dest)
                        .desired_width(280.0)
                        .hint_text(".zip 或 .7z 文件路径"),
                );
                if ui.button("另存为…").clicked() {
                    if let Some(p) = rfd::FileDialog::new()
                        .add_filter("ZIP", &["zip"])
                        .add_filter("7z", &["7z"])
                        .save_file()
                    {
                        self.compress_dest = p.to_string_lossy().to_string();
                        self.compress_format_zip = p
                            .extension()
                            .map(|e| e.eq_ignore_ascii_case("zip"))
                            .unwrap_or(true);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.compress_format_zip, "ZIP 格式（否则 7z）");
                ui.checkbox(&mut self.compress_recursive, "递归子目录");
            });

            ui.add_space(12.0);
            let can_compress = !self.running
                && !self.compress_sources.is_empty()
                && !self.compress_dest.is_empty();
            if ui
                .add_enabled(can_compress, egui::Button::new("压缩"))
                .clicked()
            {
                self.start_compress();
            }
        });
    }
}
