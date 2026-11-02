//! App state: routes, threads, messages, composer, palette, settings.

use rune::{Key, KeyEvent, TextEdit, Tx};
use rune_kit::shell::{PaneState, is_sidebar_toggle};

/// Parity target this build tracks (see `t3 --version`).
pub const PARITY_T3_VERSION: &str = "0.1.7-nightly.20261102.4061";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Route {
    Home,
    Session(String),
    Diff,
    Settings(String),
}

#[derive(Clone, Debug)]
pub struct Thread {
    pub id: String,
    pub title: String,
    pub time: String,
    pub unread: u32,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub role: &'static str,
    pub body: String,
    pub streaming: bool,
}

pub struct App {
    pub sidebar: PaneState,
    pub route: Route,
    pub threads: Vec<Thread>,
    pub messages: Vec<Message>,
    pub composer: TextEdit,
    pub palette_open: bool,
    pub palette_query: TextEdit,
    pub rpc_status: String,
    pub rpc_threads: Vec<(String, String)>,
}

pub fn init() -> App {
    App {
        sidebar: PaneState::new(260.0).range(180.0, 420.0),
        route: Route::Home,
        threads: vec![
            Thread { id: "t-new-chat".into(), title: "New chat".into(), time: "now".into(), unread: 0 },
            Thread { id: "t-rune-ports".into(), title: "Port the settings pane".into(), time: "12m".into(), unread: 2 },
            Thread { id: "t-scroll".into(), title: "Fix scroll jank in the list".into(), time: "1h".into(), unread: 0 },
        ],
        messages: vec![
            Message { role: "user", body: "Recreate the desktop 1:1 in Rune on Linux.".into(), streaming: false },
            Message {
                role: "assistant",
                body: "Plan:\n\n- Sidebar + thread list from read-only RPC\n- Streaming transcript as Markdown\n- Composer with model picker\n\n```rust\nlet app = rune::App::new(spec).run();\n```\n".into(),
                streaming: false,
            },
        ],
        composer: TextEdit::new(""),
        palette_open: false,
        palette_query: TextEdit::new(""),
        rpc_status: "Not connected — press Refresh (read-only `t3 thread list`).".into(),
        rpc_threads: Vec::new(),
    }
}

pub fn on_key(s: &mut App, ev: &KeyEvent, _: &mut Tx) -> bool {
    if is_sidebar_toggle(ev) {
        s.sidebar.toggle();
        return true;
    }
    if ev.is_command('k') {
        s.palette_open = !s.palette_open;
        return true;
    }
    if ev.is_command('n') {
        s.route = Route::Home;
        s.messages.clear();
        return true;
    }
    if ev.key == Key::Escape && s.palette_open {
        s.palette_open = false;
        return true;
    }
    false
}
