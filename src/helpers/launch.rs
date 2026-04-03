// ============ IMPORTS ============
use std::process::Command as StdCommand;




// ============ CRATES ============
use crate::AppEntry;
use crate::ron::LauncherConfig;




// ============ FUNCTIONS ============
pub fn launch_app(exec: &str, cfg: &LauncherConfig, terminal: bool)
{
    let full_cmd = if terminal && !cfg.behaviour.terminal_command.is_empty()
    {
        format!("{} {}", cfg.behaviour.terminal_command.trim(), exec)
    }
    else
    {
        exec.to_string()
    };

    let parts: Vec<&str> = full_cmd.split_whitespace().collect();
    if let Some((prog, args)) = parts.split_first()
    {
        let _ = StdCommand::new(prog).args(args).spawn();
    }
}



pub fn filter_entries(entries: &[AppEntry], query: &str, cfg: &LauncherConfig) -> Vec<AppEntry>
{
    if query.is_empty() { return entries.to_vec(); }

    let q = if cfg.behaviour.case_sensitive { query.to_string() } else { query.to_lowercase() };

    let normalize = |s: &str| -> String
    {
        if cfg.behaviour.case_sensitive { s.to_string() } else { s.to_lowercase() }
    };

    let mut scored: Vec<(usize, AppEntry)> = entries.iter().filter_map(|e|
    {
        let name    = normalize(&e.name);
        let comment = normalize(&e.comment);
        let exec    = normalize(&e.exec);

        let name_match    = cfg.behaviour.search_name     && name.contains(&q);
        let comment_match = cfg.behaviour.search_comment  && comment.contains(&q);
        let exec_match    = cfg.behaviour.search_exec     && exec.contains(&q);
        let kw_match      = cfg.behaviour.search_keywords && e.keywords.iter().any(|k| normalize(k).contains(&q));

        if !(name_match || comment_match || exec_match || kw_match) { return None; }

        let score = if name.starts_with(&q)
        {
            0
        }
        else if name_match
        {
            1
        }
        else if kw_match
        {
            2
        }
        else if exec_match
        {
            3
        }
        else
        {
            4
        };

        Some((score, e.clone()))
    }).collect();

    scored.sort_by(|(sa, a), (sb, b)| sa.cmp(sb).then(a.name.to_lowercase().cmp(&b.name.to_lowercase())));
    scored.into_iter().map(|(_, e)| e).collect()
}
