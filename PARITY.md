# Parity with T3 Code Desktop Nightly

Target: `0.1.7-nightly.20261102.4061` (channel `nightly`, `t3 --version`).
Upstream source of the Nightly: the `t3 update --channel nightly` feed
(registry.npmjs.org-backed; client bundle ships inside the installed version
dir). Original repo: `pingdotgg/t3code` (this machine uses the `ImBIOS/t3code`
fork as its checkout).

Client inventory method: `~/.t3/runtime/versions/<ver>/client/assets/*.js`
filenames + `t3 thread list|show --json` (read-only). The bundles are minified
without sourcemaps, so route/grep-level fidelity is tracked by screen, not by
import.

## Screen matrix

| # | T3 Desktop screen | rune build | Notes |
|---|---|---|---|
| 1 | App shell: sidebar, workspace header, toasts, dialogs | PARTIAL — SplitView+Sidebar+Toolbar, no toasts/dialogs yet | `rune-kit::shell`, OpenCode theme |
| 2 | Chat thread: composer, timeline, turn items | PARTIAL — Markdown transcript + local-echo composer | streaming deltas, tool pills, queued edits missing |
| 3 | Composer: mode/host/workspace/effort pickers, model picker | MISSING — plain TextArea | pickers disabled-live pattern exists in rune `opencode` example |
| 4 | Terminal drawer + panel terminals | MISSING | no pty/ANSI component in rune-kit (biggest gap) |
| 5 | Diff / code review | STUB — `diff` fence tint only | file/hunk headers, split view, staging missing |
| 6 | File explorer + file preview zoo | MISSING | PDF/Office/notebook WASM viewers out of scope for v1 |
| 7 | Browser preview panel | MISSING | no webview in rune; out of scope for v1 |
| 8 | Device panels (iOS/Android) | MISSING | out of scope for v1 |
| 9 | Pull requests | MISSING | diff fetch + checkout commands later |
| 10 | Projects / worktrees / scripts | MISSING | read-only thread list only |
| 11 | Settings (22 pages) | STUB — one page | appearance page first |
| 12 | Appearance / theme editor | PARTIAL — system dark/light via OpenCode theme | editor UI missing |
| 13 | Keybindings | MISSING — hardcoded ⌘K/⌘N/sidebar toggle | settings page later |
| 14 | Usage | MISSING | `getUsageSummary` RPC later |
| 15 | Connect / pairing / accounts | MISSING | out of scope for v1 |
| 16 | Diagnostics / maintenance | MISSING | later |

## Data dependencies (read-only first)

- `t3 thread list --json [--project]` → sidebar (DONE via `t3rpc::thread_list`)
- `t3 thread show --json --messages N <id>` → transcript (DONE via `t3rpc::thread_messages`)
- Live WS/SSE sync, mutations (`send`, create/fork/rename/archive), attachments,
  terminal attach, preview RPCs — all LATER, behind explicit flags.

## Rune contributions needed (upstream-first)

1. Window-scoped screenshot + recording (single app window) — issue to file on
   `anomalyco/rune`; headless `--snap`/`--video` covers only rune's own frames.
2. Terminal emulation component (pty/ANSI) — largest M3 gap; track upstream.
3. Diff view components (file/hunk, split/inline) — may start here, move up.
4. Command palette wiring + global search.
5. Linux shell maturity (IME, a11y, tray, portal chooser) — partly covered by
   `ImBIOS/rune#29` (qa/linux-issues).
