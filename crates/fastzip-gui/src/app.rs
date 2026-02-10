//! FastZip 主界面 - Leptos + rust-ui 风格

use leptos::*;
use tauri::invoke;

#[component]
pub fn App() -> impl IntoView {
    let (tab, set_tab) = signal(false); // false = 解压, true = 压缩
    let (archive_path, set_archive_path) = signal(String::new());
    let (dest_path, set_dest_path) = signal(String::new());
    let (smart_extract, set_smart_extract) = signal(true);
    let (password, set_password) = signal(String::new());
    let (preview_format, set_preview_format) = signal(String::new());
    let (preview_entries, set_preview_entries) = signal(Vec::<String>::new());

    let (compress_sources, set_compress_sources) = signal(Vec::<String>::new());
    let (compress_dest, set_compress_dest) = signal(String::new());
    let (compress_recursive, set_compress_recursive) = signal(true);
    let (compress_format_zip, set_compress_format_zip) = signal(true);

    let (status, set_status) = signal(String::new());
    let (error, set_error) = signal(String::new());
    let (running, set_running) = signal(false);

    let on_pick_file = move |_| {
        spawn_local(async move {
            if let Ok(Some(p)) = invoke::<_, Option<String>>("pick_file", ()).await {
                set_archive_path.set(p.clone());
                set_error.set(String::new());
                if let Ok(Ok((fmt, entries))) =
                    invoke::<_, Result<(String, Vec<String>), String>>("list_archive", (p,)).await
                {
                    set_preview_format.set(fmt);
                    set_preview_entries.set(entries);
                }
            }
        });
    };

    let on_pick_folder = move |_| {
        spawn_local(async move {
            if let Ok(Some(p)) = invoke::<_, Option<String>>("pick_folder", ()).await {
                set_dest_path.set(p);
            }
        });
    };

    let on_pick_files = move |_| {
        spawn_local(async move {
            if let Ok(Some(files)) = invoke::<_, Option<Vec<String>>>("pick_files", ()).await {
                set_compress_sources.update(|v| v.extend(files));
            }
        });
    };

    let on_save_file = move |_| {
        spawn_local(async move {
            if let Ok(Some(p)) = invoke::<_, Option<String>>("save_file", ()).await {
                set_compress_dest.set(p.clone());
                set_compress_format_zip
                    .set(p.to_lowercase().ends_with(".zip"));
            }
        });
    };

    let remove_source = move |i: usize| {
        set_compress_sources.update(|v| {
            v.remove(i);
        });
    };

    let on_extract = move |_| {
        let archive = archive_path.get();
        let dest = dest_path.get();
        if archive.is_empty() || dest.is_empty() {
            set_error.set("请选择压缩包和目标目录".to_string());
            return;
        }
        set_running.set(true);
        set_error.set(String::new());
        set_status.set("正在解压...".to_string());
        let pw = if password.get().is_empty() {
            None
        } else {
            Some(password.get())
        };
        spawn_local(async move {
            let result: Result<String, String> = invoke(
                "extract",
                (archive, dest, smart_extract.get(), pw),
            )
            .await;
            set_running.set(false);
            match result {
                Ok(path) => {
                    set_status.set(format!("已解压到: {}", path));
                    set_error.set(String::new());
                }
                Err(e) => {
                    set_error.set(e);
                    set_status.set(String::new());
                }
            }
        });
    };

    let on_compress = move |_| {
        let sources = compress_sources.get();
        let dest = compress_dest.get();
        if sources.is_empty() || dest.is_empty() {
            set_error.set("请添加要压缩的文件并指定输出路径".to_string());
            return;
        }
        set_running.set(true);
        set_error.set(String::new());
        set_status.set("正在压缩...".to_string());
        spawn_local(async move {
            let result: Result<(), String> = invoke(
                "compress",
                (sources, dest, compress_format_zip.get(), compress_recursive.get()),
            )
            .await;
            set_running.set(false);
            match result {
                Ok(()) => {
                    set_status.set("压缩完成".to_string());
                    set_error.set(String::new());
                }
                Err(e) => {
                    set_error.set(e);
                    set_status.set(String::new());
                }
            }
        });
    };

    view! {
        <div class="app">
            <header class="header">
                <button
                    class=move || if !tab.get() { "tab active" } else { "tab" }
                    on:click=move |_| set_tab.set(false)
                >
                    "解压"
                </button>
                <button
                    class=move || if tab.get() { "tab active" } else { "tab" }
                    on:click=move |_| set_tab.set(true)
                >
                    "压缩"
                </button>
            </header>

            <main class="main">
                <Show
                    when=move || !tab.get()
                    fallback=|| view! {
                        <div class="panel">
                                <h2>"压缩文件/目录"</h2>
                                <button class="btn secondary" on:click=on_pick_files>"添加文件或目录…"</button>
                                <Show when=move || !compress_sources.get().is_empty() fallback=|| ()>
                                    {move || view! {
                                        <div class="scroll-entries sources">
                                            {compress_sources.get().iter().enumerate().map(|(i, p)| view! {
                                                <div class="row entry-row">
                                                    <span class="path">{p.clone()}</span>
                                                    <button class="btn small" on:click=move |_| remove_source(i)>"移除"</button>
                                                </div>
                                            }).collect_view()}
                                        </div>
                                    }}
                                </Show>
                                <div class="row">
                                    <label>"输出路径:"</label>
                                    <input
                                        type="text"
                                        prop:value=move || compress_dest.get()
                                        readonly=true
                                        placeholder=".zip 或 .7z 文件路径"
                                    />
                                    <button class="btn secondary" on:click=on_save_file>"另存为…"</button>
                                </div>
                                <div class="row">
                                    <label class="checkbox">
                                        <input
                                            type="checkbox"
                                            prop:checked=move || compress_format_zip.get()
                                            on:change=move |ev| set_compress_format_zip.set(event_target_checked(&ev))
                                        />
                                        "ZIP 格式（否则 7z）"
                                    </label>
                                    <label class="checkbox">
                                        <input
                                            type="checkbox"
                                            prop:checked=move || compress_recursive.get()
                                            on:change=move |ev| set_compress_recursive.set(event_target_checked(&ev))
                                        />
                                        "递归子目录"
                                    </label>
                                </div>
                                <div class="row actions">
                                    <button
                                        class="btn primary"
                                        disabled=move || running.get() || compress_sources.get().is_empty() || compress_dest.get().is_empty()
                                        on:click=on_compress
                                    >
                                        {move || if running.get() { "压缩中…" } else { "压缩" }}
                                    </button>
                                </div>
                            </div>
                    }
                >
                    <div class="panel">
                        <h2>"解压压缩包"</h2>
                                <div class="row">
                                    <label>"压缩包:"</label>
                                    <input
                                        type="text"
                                        prop:value=move || archive_path.get()
                                        placeholder="路径或点击浏览"
                                        readonly=true
                                    />
                                    <button class="btn secondary" on:click=on_pick_file>"浏览…"</button>
                                </div>
                                <div class="row">
                                    <label>"目标目录:"</label>
                                    <input
                                        type="text"
                                        prop:value=move || dest_path.get()
                                        placeholder="解压到此目录"
                                        readonly=true
                                    />
                                    <button class="btn secondary" on:click=on_pick_folder>"浏览…"</button>
                                </div>
                                <div class="row">
                                    <label class="checkbox">
                                        <input
                                            type="checkbox"
                                            prop:checked=move || smart_extract.get()
                                            on:change=move |ev| set_smart_extract.set(event_target_checked(&ev))
                                        />
                                        "智能解压（根据内容自动选择子目录）"
                                    </label>
                                </div>
                                <div class="row">
                                    <label>"密码:"</label>
                                    <input
                                        type="password"
                                        prop:value=move || password.get()
                                        on:input=move |ev| set_password.set(event_target_value(&ev))
                                        placeholder="可选"
                                    />
                                </div>
                                <Show
                                    when=move || !preview_format.get().is_empty()
                                    fallback=|| ()
                                >
                                    {move || {
                                        let fmt = preview_format.get();
                                        let entries = preview_entries.get();
                                        view! {
                                            <div class="preview">
                                                <p class="small">"格式: " {fmt}</p>
                                                <div class="scroll-entries">
                                                    {entries.iter().take(50).map(|n| view! {
                                                        <div class="entry">{n.clone()}</div>
                                                    }).collect_view()}
                                                    <Show when=move || entries.len() > 50 fallback=|| ()>
                                                        <div class="entry">"..."</div>
                                                    </Show>
                                                </div>
                                            </div>
                                        }
                                    }}
                                </Show>
                                <div class="row actions">
                                    <button
                                        class="btn primary"
                                        disabled=move || running.get() || archive_path.get().is_empty() || dest_path.get().is_empty()
                                        on:click=on_extract
                                    >
                                        {move || if running.get() { "解压中…" } else { "解压" }}
                                    </button>
                                </div>
                            </div>
                </Show>
            </main>

            <footer class="footer">
                {move || {
                    let s = status.get();
                    let e = error.get();
                    view! {
                        <>
                            <Show when=move || running.get() fallback=|| ()>
                                <span class="status running">"⏳ " {s}</span>
                            </Show>
                            <Show when=move || !running.get() && !s.is_empty() fallback=|| ()>
                                <span class="status ok">{s}</span>
                            </Show>
                            <Show when=move || !e.is_empty() fallback=|| ()>
                                <span class="status err">{e}</span>
                            </Show>
                        </>
                    }
                }}
            </footer>
        </div>
    }
}
