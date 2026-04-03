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
        let _ = StdCommand::new(prog).args(args).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn();
    }
}



fn try_evaluate(expr: &str) -> Option<String>
{
    // Must contain at least one operator to avoid showing a result for every
    // plain word the user types.
    let has_operator = expr.contains(['+', '-', '*', '/', '%', '^', '(', ')']);
    if !has_operator { return None; }

    let mut ns = fasteval::EmptyNamespace;
    let result = fasteval::ez_eval(expr, &mut ns).ok()?;

    // Format: drop the decimal part when the result is a whole number.
    if result.fract() == 0.0 && result.abs() < 1e15
    {
        Some(format!("{}", result as i64))
    }
    else
    {
        // Up to 10 significant digits, strip trailing zeros.
        let s = format!("{:.10}", result);
        Some(s.trim_end_matches('0').trim_end_matches('.').to_string())
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
    let mut results: Vec<AppEntry> = scored.into_iter().map(|(_, e)| e).collect();

    // Prepend calculator result if enabled and the query evaluates cleanly.
    if cfg.behaviour.calc_enabled && let Some(value) = try_evaluate(query.trim())
    {
        let calc_entry = AppEntry
        {
            name:      format!("= {}", value),
            exec:      String::new(),
            comment:   query.trim().to_string(),
            icon:      String::new(),
            icon_path: None,
            keywords:  Vec::new(),
            terminal:  false,
        };
        results.insert(0, calc_entry);
    }

    results
}
