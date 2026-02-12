//! FastZip GUI - Modern Interface

use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::Serialize;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_with_args(cmd: &str, args: JsValue) -> JsValue;
}

async fn invoke<T: Serialize, R: for<'de> serde::Deserialize<'de>>(cmd: &str, args: T) -> Result<R, String> {
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result_js = invoke_with_args(cmd, args_js).await;
    serde_wasm_bindgen::from_value(result_js).map_err(|e| e.to_string())
}

async fn invoke_no_args<R: for<'de> serde::Deserialize<'de>>(cmd: &str) -> Result<R, String> {
    let result_js = invoke_without_args(cmd).await;
    serde_wasm_bindgen::from_value(result_js).map_err(|e| e.to_string())
}

#[component]
pub fn App() -> impl IntoView {
    let (tab, set_tab) = create_signal(false);
    let (archive_path, set_archive_path) = create_signal(String::new());
    let (dest_path, set_dest_path) = create_signal(String::new());
    let (smart_extract, set_smart_extract) = create_signal(true);
    let (password, set_password) = create_signal(String::new());
    let (preview_format, set_preview_format) = create_signal(String::new());
    let (preview_entries, set_preview_entries) = create_signal(Vec::<String>::new());
    
    let (compress_sources, set_compress_sources) = create_signal(Vec::<String>::new());
    let (compress_dest, set_compress_dest) = create_signal(String::new());
    let (compress_recursive, set_compress_recursive) = create_signal(true);
    let (compress_format_zip, set_compress_format_zip) = create_signal(true);
    
    let (status, set_status) = create_signal(String::new());
    let (status_type, set_status_type) = create_signal("info");
    let (running, set_running) = create_signal(false);

    let set_status_with_type = move |msg: String, stype: &'static str| {
        set_status.set(msg);
        set_status_type.set(stype);
    };

    let on_pick_file = move |_| {
        spawn_local(async move {
            match invoke_no_args::<Option<String>>("pick_file").await {
                Ok(Some(p)) => {
                    set_archive_path.set(p.clone());
                    set_status_with_type(String::new(), "info");
                    match invoke::<_, Result<(String, Vec<String>), String>>("list_archive", (p,)).await {
                        Ok(Ok((fmt, entries))) => {
                            set_preview_format.set(fmt);
                            set_preview_entries.set(entries);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    };

    let on_pick_folder = move |_| {
        spawn_local(async move {
            if let Ok(Some(p)) = invoke_no_args::<Option<String>>("pick_folder").await {
                set_dest_path.set(p);
            }
        });
    };

    let on_pick_files = move |_| {
        spawn_local(async move {
            if let Ok(Some(files)) = invoke_no_args::<Option<Vec<String>>>("pick_files").await {
                set_compress_sources.update(|v| v.extend(files));
            }
        });
    };

    let on_save_file = move |_| {
        spawn_local(async move {
            if let Ok(Some(p)) = invoke_no_args::<Option<String>>("save_file").await {
                set_compress_dest.set(p.clone());
                set_compress_format_zip.set(p.to_lowercase().ends_with(".zip"));
            }
        });
    };

    let on_extract = move |_| {
        let archive = archive_path.get();
        let dest = dest_path.get();
        if archive.is_empty() || dest.is_empty() {
            set_status_with_type("请选择压缩包和目标目录".to_string(), "error");
            return;
        }
        set_running.set(true);
        set_status_with_type("正在解压...".to_string(), "running");
        let pw = if password.get().is_empty() { None } else { Some(password.get()) };
        
        spawn_local(async move {
            let result: Result<String, String> = invoke("extract", (archive, dest, smart_extract.get(), pw)).await;
            set_running.set(false);
            match result {
                Ok(path) => set_status_with_type(format!("已解压到: {}", path), "success"),
                Err(e) => set_status_with_type(e, "error"),
            }
        });
    };

    let on_compress = move |_| {
        let sources = compress_sources.get();
        let dest = compress_dest.get();
        if sources.is_empty() || dest.is_empty() {
            set_status_with_type("请添加要压缩的文件并指定输出路径".to_string(), "error");
            return;
        }
        set_running.set(true);
        set_status_with_type("正在压缩...".to_string(), "running");
        
        spawn_local(async move {
            let result: Result<(), String> = invoke("compress", 
                (sources, dest, compress_format_zip.get(), compress_recursive.get())).await;
            set_running.set(false);
            match result {
                Ok(()) => set_status_with_type("压缩完成".to_string(), "success"),
                Err(e) => set_status_with_type(e, "error"),
            }
        });
    };

    view! {
        <div class="app">
            <header class="header">
                <div class="logo-section">
                    <div class="logo-icon">"FZ"</div>
                    <div class="logo-text">"FastZip"</div>
                </div>
                <div class="tab-switcher">
                    <button
                        class=move || if !tab.get() { "tab active" } else { "tab" }
                        on:click=move |_| set_tab.set(false)
                    >
                        <span>"📦"</span>
                        <span>"解压"</span>
                    </button>
                    <button
                        class=move || if tab.get() { "tab active" } else { "tab" }
                        on:click=move |_| set_tab.set(true)
                    >
                        <span>"🗜️"</span>
                        <span>"压缩"</span>
                    </button>
                </div>
            </header>

            <main class="main">
                {move || if !tab.get() {
                    view! {
                        <ExtractView
                            archive_path=archive_path
                            dest_path=dest_path
                            smart_extract=smart_extract
                            set_smart_extract=set_smart_extract
                            password=password
                            set_password=set_password
                            preview_format=preview_format
                            preview_entries=preview_entries
                            on_pick_file=on_pick_file
                            on_pick_folder=on_pick_folder
                            on_extract=on_extract
                            running=running
                        />
                    }.into_view()
                } else {
                    view! {
                        <CompressView
                            sources=compress_sources
                            set_sources=set_compress_sources
                            dest=compress_dest
                            recursive=compress_recursive
                            set_recursive=set_compress_recursive
                            format_zip=compress_format_zip
                            set_format_zip=set_compress_format_zip
                            on_pick_files=on_pick_files
                            on_save_file=on_save_file
                            on_compress=on_compress
                            running=running
                        />
                    }.into_view()
                }}
            </main>

            <footer class="footer">
                <div class="status-bar">
                    {move || {
                        let stype = status_type.get();
                        let s = status.get();
                        let (icon, class) = match stype {
                            "success" => ("✓", "status-success"),
                            "error" => ("✗", "status-error"),
                            "running" => ("⏳", "status-running"),
                            _ => ("ℹ", "status-info"),
                        };
                        view! {
                            <span class=class>{icon} " " {s}</span>
                        }
                    }}
                </div>
                {move || if running.get() {
                    view! {
                        <div class="progress-bar active">
                            <div class="progress-fill" style="width: 100%"></div>
                        </div>
                    }.into_view()
                } else { ().into_view() }}
            </footer>
        </div>
    }
}

#[component]
fn ExtractView(
    archive_path: ReadSignal<String>,
    dest_path: ReadSignal<String>,
    smart_extract: ReadSignal<bool>,
    set_smart_extract: WriteSignal<bool>,
    password: ReadSignal<String>,
    set_password: WriteSignal<String>,
    preview_format: ReadSignal<String>,
    preview_entries: ReadSignal<Vec<String>>,
    on_pick_file: impl Fn(leptos::ev::MouseEvent) + 'static + Clone,
    on_pick_folder: impl Fn(leptos::ev::MouseEvent) + 'static + Clone,
    on_extract: impl Fn(leptos::ev::MouseEvent) + 'static + Clone,
    running: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-header">
                <span class="card-icon">"📂"</span>
                <span class="card-title">"选择压缩包"</span>
            </div>
            
            <div class="drop-zone" on:click=on_pick_file.clone()>
                <div class="drop-zone-icon">"📦"</div>
                <div class="drop-zone-text">"点击选择或拖拽文件到此处"</div>
                <div class="drop-zone-hint">"支持 ZIP, 7z, TAR, RAR 等格式"</div>
            </div>

            {move || {
                let path = archive_path.get();
                if !path.is_empty() {
                    view! {
                        <div class="input-group" style="margin-top: 16px">
                            <label class="input-label">"已选择"</label>
                            <div class="input-wrapper">
                                <input type="text" class="input-field" prop:value=path readonly=true />
                                <button class="btn btn-secondary" on:click=on_pick_file.clone()>"浏览"</button>
                            </div>
                        </div>
                    }.into_view()
                } else { ().into_view() }
            }}
        </div>

        <div class="card">
            <div class="card-header">
                <span class="card-icon">"⚙️"</span>
                <span class="card-title">"解压选项"</span>
            </div>

            <div class="input-group">
                <label class="input-label">"目标目录"</label>
                <div class="input-wrapper">
                    <input type="text" class="input-field" prop:value=dest_path.get() placeholder="选择解压目标文件夹" readonly=true />
                    <button class="btn btn-secondary" on:click=on_pick_folder.clone()>"📁 浏览"</button>
                </div>
            </div>

            <div class="options-grid">
                <label class="option-item">
                    <input type="checkbox" class="option-checkbox" prop:checked=smart_extract.get()
                        on:change=move |ev| { if let Ok(c) = event_target_checked(&ev) { set_smart_extract.set(c); } } />
                    <div>
                        <div class="option-label">"智能解压"</div>
                        <div class="option-hint">"根据内容自动选择子目录"</div>
                    </div>
                </label>
            </div>

            <div class="input-group">
                <label class="input-label">"密码保护（可选）"</label>
                <input type="password" class="input-field" prop:value=password.get()
                    on:input=move |ev| { if let Ok(v) = event_target_value(&ev) { set_password.set(v); } }
                    placeholder="如果压缩包有密码，请在此输入" />
            </div>

            {move || {
                let fmt = preview_format.get();
                let entries = preview_entries.get();
                if !fmt.is_empty() {
                    let has_more = entries.len() > 50;
                    view! {
                        <div class="preview-section">
                            <div class="preview-header">
                                <span class="preview-badge">{fmt}</span>
                                <span style="color: var(--text-muted); font-size: 12px">{entries.len()} " 个项目"</span>
                            </div>
                            <div class="preview-content">
                                {entries.into_iter().take(50).map(|n| view! {
                                    <div class="preview-item"><span>"📄"</span><span>{n}</span></div>
                                }).collect_view()}
                                {if has_more { view! { <div class="preview-item" style="font-style: italic; opacity: 0.6">"... 还有更多项目"</div> }.into_view() } else { ().into_view() }}
                            </div>
                        </div>
                    }.into_view()
                } else { ().into_view() }
            }}

            <div class="action-bar">
                <button class="btn btn-primary"
                    disabled=move || running.get() || archive_path.get().is_empty() || dest_path.get().is_empty()
                    on:click=on_extract>
                    {move || if running.get() { "⏳ 解压中..." } else { "🚀 开始解压" }}
                </button>
            </div>
        </div>
    }
}

#[component]
fn CompressView(
    sources: ReadSignal<Vec<String>>,
    set_sources: WriteSignal<Vec<String>>,
    dest: ReadSignal<String>,
    recursive: ReadSignal<bool>,
    set_recursive: WriteSignal<bool>,
    format_zip: ReadSignal<bool>,
    set_format_zip: WriteSignal<bool>,
    on_pick_files: impl Fn(leptos::ev::MouseEvent) + 'static + Clone,
    on_save_file: impl Fn(leptos::ev::MouseEvent) + 'static + Clone,
    on_compress: impl Fn(leptos::ev::MouseEvent) + 'static + Clone,
    running: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-header">
                <span class="card-icon">"📋"</span>
                <span class="card-title">"选择文件"</span>
            </div>
            
            <button class="btn btn-secondary" style="width: 100%; margin-bottom: 16px" on:click=on_pick_files.clone()>
                "➕ 添加文件或目录"
            </button>

            {move || {
                let files = sources.get();
                if !files.is_empty() {
                    view! {
                        <div class="file-list">
                            {files.into_iter().enumerate().map(|(i, p)| view! {
                                <div class="file-item">
                                    <span class="file-icon">"📄"</span>
                                    <span class="file-name">{p}</span>
                                    <button class="file-remove" on:click=move |_| set_sources.update(|v| { v.remove(i); })>"✕"</button>
                                </div>
                            }).collect_view()}
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="empty-state">
                            <div class="empty-state-icon">"🗂️"</div>
                            <div class="empty-state-text">"还没有选择文件"</div>
                        </div>
                    }.into_view()
                }
            }}
        </div>

        <div class="card">
            <div class="card-header">
                <span class="card-icon">"⚙️"</span>
                <span class="card-title">"压缩选项"</span>
            </div>

            <div class="input-group">
                <label class="input-label">"输出文件"</label>
                <div class="input-wrapper">
                    <input type="text" class="input-field" prop:value=dest.get() placeholder="选择保存位置" readonly=true />
                    <button class="btn btn-secondary" on:click=on_save_file.clone()>"💾 保存到"</button>
                </div>
            </div>

            <div class="options-grid">
                <label class="option-item">
                    <input type="checkbox" class="option-checkbox" prop:checked=format_zip.get()
                        on:change=move |ev| { if let Ok(c) = event_target_checked(&ev) { set_format_zip.set(c); } } />
                    <div>
                        <div class="option-label">"ZIP 格式"</div>
                        <div class="option-hint">"通用兼容性好"</div>
                    </div>
                </label>
                
                <label class="option-item">
                    <input type="checkbox" class="option-checkbox" prop:checked=recursive.get()
                        on:change=move |ev| { if let Ok(c) = event_target_checked(&ev) { set_recursive.set(c); } } />
                    <div>
                        <div class="option-label">"包含子目录"</div>
                        <div class="option-hint">"递归压缩所有文件"</div>
                    </div>
                </label>
            </div>

            <div class="action-bar">
                <button class="btn btn-primary"
                    disabled=move || running.get() || sources.get().is_empty() || dest.get().is_empty()
                    on:click=on_compress>
                    {move || if running.get() { "⏳ 压缩中..." } else { "🗜️ 开始压缩" }}
                </button>
            </div>
        </div>
    }
}

fn event_target_checked(ev: &web_sys::Event) -> Result<bool, JsValue> {
    let target = ev.target().ok_or("no target")?;
    let input: web_sys::HtmlInputElement = target.dyn_into()?;
    Ok(input.checked())
}

fn event_target_value(ev: &web_sys::Event) -> Result<String, JsValue> {
    let target = ev.target().ok_or("no target")?;
    let input: web_sys::HtmlInputElement = target.dyn_into()?;
    Ok(input.value())
}
