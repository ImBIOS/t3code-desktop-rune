//! Read-only bridge to the local T3 server via its `t3` CLI.
//!
//! Only `thread list --json` and `thread show --json` are used: the same
//! read-only GET surface the app needs (threads, transcript). Auth stays
//! inside the server/CLI; this crate never reads tokens or passwords.

use std::process::Command;

/// Candidate `t3` binaries, newest first.
fn candidates() -> Vec<std::path::PathBuf> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/imbios".into());
    let versions = std::path::Path::new(&home).join(".t3/runtime/versions");
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&versions) {
        let mut dirs: Vec<_> = rd.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.join("t3").exists()).collect();
        dirs.sort();
        for d in dirs.into_iter().rev() {
            out.push(d.join("t3"));
        }
    }
    out.push("t3".into());
    out
}

fn run_t3(args: &[&str]) -> Result<serde_json::Value, String> {
    let mut last = String::from("no t3 binary found");
    for bin in candidates() {
        match Command::new(&bin).args(args).output() {
            Ok(o) if o.status.success() => {
                return serde_json::from_slice(&o.stdout).map_err(|e| format!("bad JSON from t3: {e}"));
            }
            Ok(o) => last = String::from_utf8_lossy(&o.stderr).trim().chars().take(300).collect(),
            Err(e) => last = format!("{}: {e}", bin.display()),
        }
    }
    Err(last)
}

/// Read-only: list threads of a project (id or path), newest first.
pub fn thread_list(project: &str) -> Result<Vec<(String, String)>, String> {
    let v = run_t3(&["thread", "list", "--json", "--project", project])?;
    let arr = v.as_array().ok_or("unexpected thread list shape")?;
    Ok(arr
        .iter()
        .map(|t| {
            (
                t.get("threadId").and_then(|x| x.as_str()).unwrap_or("?").to_owned(),
                t.get("title").and_then(|x| x.as_str()).unwrap_or("(untitled)").to_owned(),
            )
        })
        .collect())
}

/// Read-only: latest messages of a thread (count capped by caller).
pub fn thread_messages(thread: &str, n: usize) -> Result<Vec<(String, String)>, String> {
    let v = run_t3(&["thread", "show", "--json", "--messages", &n.to_string(), thread])?;
    let arr = v.get("messages").and_then(|x| x.as_array()).cloned().unwrap_or_default();
    Ok(arr
        .iter()
        .map(|m| {
            let role = m.get("role").and_then(|x| x.as_str()).unwrap_or("?").to_owned();
            let text = m
                .get("text")
                .and_then(|x| x.as_str())
                .or_else(|| m.get("content").and_then(|x| x.as_str()))
                .unwrap_or("")
                .to_owned();
            (role, text)
        })
        .filter(|(_, t)| !t.is_empty())
        .collect())
}
