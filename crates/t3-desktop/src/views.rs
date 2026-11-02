//! Views: app shell, sidebar, transcript, composer, diff, palette, settings.

use rune::{Cx, El, TextEdit, col, el, row, text};
use rune_kit::shell::{Sidebar, SidebarRow, SidebarSection, SplitView, Toolbar};
use rune_kit::{Button, Kit, Markdown, TextArea};

use crate::model::{App, Message, Route, PARITY_T3_VERSION};
use crate::t3rpc;

fn sidebar(s: &App, k: &Kit) -> El {
    let mut sb = Sidebar::new()
        .label("Threads")
        .header(SidebarRow::new("New chat").detail("⌘N").build(k).key("new"))
        .header(SidebarRow::new("Command palette").detail("⌘K").build(k).key("palette"))
        .footer(SidebarRow::new("Settings").build(k).key("settings"));
    let rows: Vec<El> = s
        .threads
        .iter()
        .map(|t| {
            let mut r = SidebarRow::new(t.title.clone())
                .detail(t.time.clone())
                .selected(matches!(&s.route, Route::Session(id) if id == &t.id))
                .on_select({
                    let id = t.id.clone();
                    move |s: &mut App, _| {
                        s.route = Route::Session(id.clone());
                    }
                });
            if t.unread > 0 {
                r = r.badge(t.unread.to_string());
            }
            r.build(k).key(t.id.clone())
        })
        .collect();
    for name in ["Pinned", "Today", "Older"] {
        let mine: Vec<El> = if name == "Today" { rows.clone() } else { Vec::new() };
        sb = sb.section(SidebarSection::new(name).count(mine.len()).rows(mine));
    }
    sb.build(k)
}

fn transcript(s: &App, k: &Kit) -> El {
    let mut list = col().key("messages").w_full().gap(16.0);
    for (i, m) in s.messages.iter().enumerate() {
        list = list.child(message_row(m, k).key(format!("m{i}")));
    }
    rune::el().key("scroll").w_full().child(list)
}

fn message_row(m: &Message, k: &Kit) -> El {
    let who = text(m.role.to_owned()).font(k.ty.small).color(k.c.text_faint);
    let body = Markdown::new(m.body.clone()).streaming(m.streaming).selectable(true).build(k);
    col().w_full().gap(4.0).child(who).child(body)
}

fn composer(s: &App, k: &Kit) -> El {
    let input = TextArea::new(&s.composer)
        .placeholder("Message T3… (read-only build: sending disabled)")
        .auto_grow(1, 6)
        .on_edit(|s: &mut App| &mut s.composer)
        .on_submit(|s: &mut App, _| {
            if !s.composer.text.trim().is_empty() {
                let body = std::mem::replace(&mut s.composer.text, String::new());
                s.messages.push(Message { role: "user", body, streaming: false });
            }
        })
        .build(k)
        .key("composer");
    let send = Button::new("Send (local echo)")
        .on_press(|s: &mut App| {
            if !s.composer.text.trim().is_empty() {
                let body = std::mem::replace(&mut s.composer.text, String::new());
                s.messages.push(Message { role: "user", body, streaming: false });
            }
        })
        .build(k)
        .key("send");
    col()
        .key("dock")
        .w_full()
        .gap(8.0)
        .child(input)
        .child(row().w_full().gap(8.0).child(send).child(
            text("Read-only: lists/reads via `t3 thread list|show`. No prompts are ever sent.".to_owned())
                .font(k.ty.small)
                .color(k.c.text_faint),
        ))
}

fn home(s: &App, k: &Kit) -> El {
    let head = text("T3 Code (Rune)".to_owned()).font(k.ty.body_medium).color(k.c.text_base);
    let sub = text(format!("Nightly parity target: {PARITY_T3_VERSION}")).font(k.ty.small).color(k.c.text_faint);
    let status = text(s.rpc_status.clone()).font(k.ty.small).color(k.c.text_muted).wrap();
    let refresh = Button::new("Refresh threads (read-only)")
        .on_press(|s: &mut App| match t3rpc::thread_list(".") {
            Ok(list) => {
                s.rpc_status = format!("{} thread(s) via read-only CLI.", list.len());
                s.rpc_threads = list.clone();
                s.threads = list
                    .into_iter()
                    .map(|(id, title)| crate::model::Thread { id, title, time: String::new(), unread: 0 })
                    .collect();
            }
            Err(e) => s.rpc_status = format!("Refresh failed (read-only): {e}"),
        })
        .build(k)
        .key("refresh");
    let mut body = col().key("home").w_full().padding(24.0).gap(12.0).child(head).child(sub).child(refresh).child(status);
    for (id, title) in s.rpc_threads.clone() {
        let line = text(format!("{title}  ({id})")).font(k.ty.small).color(k.c.text_muted).wrap();
        body = body.child(line.key(format!("rpc-{id}")));
    }
    body
}

fn session(s: &App, k: &Kit, id: &str) -> El {
    let title = s.threads.iter().find(|t| t.id == id).map(|t| t.title.clone()).unwrap_or_else(|| id.to_owned());
    let head = text(title).font(k.ty.body_medium).color(k.c.text_base);
    let load = Button::new("Load transcript (read-only)")
        .on_press({
            let id = id.to_owned();
            move |s: &mut App| match t3rpc::thread_messages(&id, 20) {
                Ok(msgs) => {
                    s.rpc_status = format!("{} message(s) loaded read-only.", msgs.len());
                    s.messages = msgs
                        .into_iter()
                        .map(|(role, body)| Message {
                            role: if role.starts_with("assist") { "assistant" } else { "user" },
                            body,
                            streaming: false,
                        })
                        .collect();
                }
                Err(e) => s.rpc_status = format!("Transcript load failed (read-only): {e}"),
            }
        })
        .build(k)
        .key("load-transcript");
    col().key("session").w_full().padding(24.0).gap(12.0).child(head).child(load).child(transcript(s, k)).child(composer(s, k))
}

fn diff_view(_s: &App, k: &Kit) -> El {
    let head = text("Diff (placeholder — file/hunk split view lands next)".to_owned())
        .font(k.ty.body_medium)
        .color(k.c.text_base);
    let sample = Markdown::new("```diff\n+ added line\n- removed line\n context\n```\n".to_owned()).build(k);
    col().key("diff").w_full().padding(24.0).gap(12.0).child(head).child(sample)
}

fn settings_view(_s: &App, k: &Kit, page: &str) -> El {
    let head = text(format!("Settings — {page}")).font(k.ty.body_medium).color(k.c.text_base);
    let note = text("Appearance, keybindings, providers and connections live here (mirrors /settings/*).".to_owned())
        .font(k.ty.body)
        .color(k.c.text_muted)
        .wrap();
    col().key("settings").w_full().padding(24.0).gap(12.0).child(head).child(note)
}

fn palette(s: &App, k: &Kit) -> El {
    if !s.palette_open {
        return el().key("no-palette");
    }
    let q = TextArea::new(&s.palette_query)
        .placeholder("Type a command…")
        .on_edit(|s: &mut App| &mut s.palette_query)
        .build(k)
        .key("palette-input");
    let cmds = ["Go: New chat (⌘N)", "Go: Settings", "Go: Diff", "Refresh threads (read-only)", "Toggle sidebar"];
    let mut list = col().w_full().gap(4.0);
    for (i, c) in cmds.iter().enumerate() {
        list = list.child(text((*c).to_owned()).font(k.ty.body).color(k.c.text_base).key(format!("cmd{i}")));
    }
    col()
        .key("palette")
        .w_full()
        .padding(16.0)
        .gap(8.0)
        .child(text("Command palette".to_owned()).font(k.ty.ui_medium).color(k.c.text_base))
        .child(q)
        .child(list)
}

pub fn root(s: &App, k: &'static Kit, cx: &mut Cx<App>) -> El {
    let main: El = match &s.route {
        Route::Home => home(s, k),
        Route::Session(id) => session(s, k, id),
        Route::Diff => diff_view(s, k),
        Route::Settings(page) => settings_view(s, k, page),
    };
    let title = match &s.route {
        Route::Home => "T3 Code",
        Route::Session(_) => "Thread",
        Route::Diff => "Diff",
        Route::Settings(_) => "Settings",
    };
    let toolbar = Toolbar::new()
        .title(title)
        .trailing(text(format!("parity {PARITY_T3_VERSION}")).font(k.ty.small).color(k.c.text_faint).key("parity"));
    let shell = SplitView::new(main)
        .sidebar(&s.sidebar, |s: &mut App| &mut s.sidebar, sidebar(s, k))
        .toolbar(toolbar)
        .titlebar_inset(cx.titlebar_inset())
        .build(k);
    if s.palette_open { col().w_full().h_full().child(shell).child(palette(s, k)) } else { shell }
}

#[allow(dead_code)]
fn _keep_imports(_: &TextEdit) {
    let _ = TextEdit::new("");
}
