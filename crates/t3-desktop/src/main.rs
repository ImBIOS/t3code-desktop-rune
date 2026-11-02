//! T3 Code Desktop recreated in Rune for Linux.
//!
//! A 1:1 rebuild of the T3 Code Desktop Nightly client (sidebar, thread list,
//! streaming transcript, composer, diff placeholder, command palette, settings)
//! against the local T3 server's read-only CLI surface. It starts from the
//! `rune-kit` shell (SplitView/Sidebar/Toolbar), the OpenCode theme, Markdown
//! transcript rendering, and a TextArea composer.
//!
//! Parity target: T3 Code `0.1.7-nightly.20261102.4061` (`t3 --version`).
//! Rune rev: the `t3/desktop-linux` branch of the rune fork until the needed
//! pieces land upstream (see PARITY.md).

mod model;
mod t3rpc;
mod views;

use model::{App, Route, init};
use rune::{AppSpec, Cx, El};
use rune_kit::shell::PaneState;
use views::root;

fn view(s: &App, cx: &mut Cx<App>) -> El {
    let dark = cx.system_dark();
    let k = rune_theme_opencode::kit(dark);
    root(s, k, cx)
}

fn main() {
    let spec = AppSpec {
        scenes: &[
            ("home", |s: &mut App| {
                s.route = Route::Home;
            }),
            ("session", |s: &mut App| {
                s.route = Route::Session("demo-thread".into());
            }),
            ("diff", |s: &mut App| {
                s.route = Route::Diff;
            }),
            ("settings", |s: &mut App| {
                s.route = Route::Settings("appearance".into());
            }),
            ("palette", |s: &mut App| {
                s.palette_open = true;
            }),
        ],
        fonts: rune_theme_opencode::FONTS,
        families: rune_theme_opencode::FAMILIES,
        on_key: Some(model::on_key),
        ..AppSpec::new("T3 Code", (1280.0, 820.0), init, view)
    };
    rune::App::new(spec)
        .chrome(rune::Chrome {
            full_size_content: true,
            traffic_lights: Some((16.0, 17.0)),
            drag_regions: true,
        })
        .run();
}

#[allow(dead_code)]
fn _keep_pane_import() {
    let _ = PaneState::new(260.0);
}
