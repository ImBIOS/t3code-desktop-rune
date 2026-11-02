# Live QA — windowed run on Linux (Xvfb + Lavapipe)

Date: 2026-11-02. Build: this repo @ scaffold + palette-overlay fix.
Parity target: T3 Code `0.1.7-nightly.20261102.4061`.
Display: `Xvfb :99 -screen 0 1440x960x24`, no window manager,
`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json` (Lavapipe).
Driver: `python-xlib` + XTest clicks, `scrot` screenshots,
`ffmpeg -f x11grab` recording. No rune capture feature was needed.

## Tour (all read-only; nothing is ever sent)

| Shot | Action | Result |
|---|---|---|
| `01-home.png` | launch | home, fixture threads, "Not connected" |
| `02-connected.png` | click Refresh | "1 thread(s) via read-only CLI", sidebar shows New thread |
| `03-palette.png` | click Command palette row | centered palette overlay, dimmed backdrop, 5 commands |
| `04-palette-closed.png` | Escape | panel gone (see note) |
| `05-thread.png` | click thread row | session view, fixture Markdown transcript + code block, composer |
| `06-transcript.png` | click Load transcript | empty real transcript replaces fixtures (that thread has 0 messages — correct) |
| `07-settings.png` | click Settings | appearance stub page |
| `tour.mp4` | full pass, 12 fps x11grab, 408 frames / 34 s | frames extracted at n=30/130/210/300/380 and read: home → connected (+cursor) → palette → thread+transcript+composer; no foreign windows, no torn frames |

## Findings

1. Refresh needs a *registered* project path: `t3 thread list --project <any path>`
   fails for unregistered dirs. The app tries `.`, then known rune checkouts,
   first success wins. OK for now; a project picker comes later.
2. Palette first rendered *below* the shell (pushed offscreen) — fixed by
   mounting it as a `rune_kit::root` overlay (scrim + centered panel).
3. After Escape-closing the palette, one screenshot showed the scrim area as a
   light-gray band that cleared on the next frame — likely a slow first repaint
   on software GL, not a stuck state (subsequent shots clean). Watch item.
4. Composer typing via XTest keycodes with explicit X focus set did not insert
   text (field focused, caret visible). Unresolved: needs a focused retry with
   keysym-verified delivery, or the app's `on_key`/IME path inspected.
5. Headless X keeps dying between agent turns here; the display + app now run
   as systemd user units (`t3rune-xvfb.service`, `t3rune-app.service`) so QA
   survives. A foreign AppImage ("Natively") once autostarted onto :99 and a WM
   appeared mid-tour — killed; keep this display free of other apps.
