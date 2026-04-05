use crate::{
    AppEntry,
    helpers::filter::filter_entries,
    ron::{LauncherConfig, SearchBehaviourConfig},
};

// ── helpers ──────────────────────────────────────────────────────────────────

fn entry(name: &str, exec: &str, comment: &str, keywords: &[&str]) -> AppEntry
{
    AppEntry {
        name: name.to_string(),
        exec: exec.to_string(),
        comment: comment.to_string(),
        icon: String::new(),
        icon_path: None,
        keywords: keywords.iter().map(|s| s.to_string()).collect(),
        terminal: false,
        name_lc: String::new(),
        exec_lc: String::new(),
        comment_lc: String::new(),
        keywords_lc: Vec::new(),
    }
    .with_normalized()
}

fn cfg_all_on() -> LauncherConfig
{
    let mut c = LauncherConfig::default();
    c.behaviour = SearchBehaviourConfig {
        search_name: true,
        search_comment: true,
        search_exec: true,
        search_keywords: true,
        case_sensitive: false,
        close_on_launch: false,
        terminal_command: String::new(),
        ..Default::default()
    };
    c
}

fn cfg_name_only() -> LauncherConfig
{
    let mut c = cfg_all_on();
    c.behaviour.search_comment = false;
    c.behaviour.search_exec = false;
    c.behaviour.search_keywords = false;
    c
}

// ── empty query ──────────────────────────────────────────────────────────────

#[test]
fn empty_query_returns_all_entries()
{
    let entries = vec![entry("Firefox", "firefox", "", &[]), entry("Vim", "vim", "", &[])];
    let cfg = cfg_all_on();
    let result = filter_entries(&entries, "", &cfg, &Default::default());
    assert_eq!(result.len(), 2);
}

#[test]
fn empty_entries_returns_empty()
{
    let result = filter_entries(&[], "firefox", &cfg_all_on(), &Default::default());
    assert!(result.is_empty());
}

// ── name matching ────────────────────────────────────────────────────────────

#[test]
fn name_prefix_match_scores_highest()
{
    let entries =
        vec![entry("X Firefox Extra", "x", "", &[]), entry("Firefox", "firefox", "", &[])];
    let result = filter_entries(&entries, "Firefox", &cfg_all_on(), &Default::default());
    assert_eq!(result[0].name, "Firefox");
}

#[test]
fn name_contains_match_works()
{
    let entries = vec![entry("My Firefox Browser", "mfb", "", &[])];
    let result = filter_entries(&entries, "Firefox", &cfg_all_on(), &Default::default());
    assert_eq!(result.len(), 1);
}

#[test]
fn no_match_returns_empty()
{
    let entries = vec![entry("Firefox", "firefox", "", &[])];
    let result = filter_entries(&entries, "vim", &cfg_all_on(), &Default::default());
    assert!(result.is_empty());
}

// ── comment matching ─────────────────────────────────────────────────────────

#[test]
fn comment_match_when_enabled()
{
    let entries = vec![entry("App", "app", "A fast web browser", &[])];
    let result = filter_entries(&entries, "web browser", &cfg_all_on(), &Default::default());
    assert_eq!(result.len(), 1);
}

#[test]
fn comment_not_searched_when_disabled()
{
    let mut cfg = cfg_all_on();
    cfg.behaviour.search_comment = false;
    let entries = vec![entry("App", "app", "web browser", &[])];
    let result = filter_entries(&entries, "browser", &cfg, &Default::default());
    assert!(result.is_empty());
}

// ── exec matching ─────────────────────────────────────────────────────────────

#[test]
fn exec_match_when_enabled()
{
    let entries = vec![entry("Text Editor", "gedit", "", &[])];
    let result = filter_entries(&entries, "gedit", &cfg_all_on(), &Default::default());
    assert_eq!(result.len(), 1);
}

#[test]
fn exec_not_searched_when_disabled()
{
    let mut cfg = cfg_all_on();
    cfg.behaviour.search_exec = false;
    let entries = vec![entry("Text Editor", "gedit", "", &[])];
    let result = filter_entries(&entries, "gedit", &cfg, &Default::default());
    assert!(result.is_empty());
}

// ── keyword matching ─────────────────────────────────────────────────────────

#[test]
fn keyword_match_when_enabled()
{
    let entries = vec![entry("App", "app", "", &["internet", "web"])];
    let result = filter_entries(&entries, "internet", &cfg_all_on(), &Default::default());
    assert_eq!(result.len(), 1);
}

#[test]
fn keyword_not_searched_when_disabled()
{
    let mut cfg = cfg_all_on();
    cfg.behaviour.search_keywords = false;
    let entries = vec![entry("App", "app", "", &["internet"])];
    let result = filter_entries(&entries, "internet", &cfg, &Default::default());
    assert!(result.is_empty());
}

// ── case sensitivity ─────────────────────────────────────────────────────────

#[test]
fn case_insensitive_matches_uppercase_query()
{
    let entries = vec![entry("firefox", "firefox", "", &[])];
    let result = filter_entries(&entries, "FIREFOX", &cfg_all_on(), &Default::default());
    assert_eq!(result.len(), 1);
}

#[test]
fn case_sensitive_no_match_on_wrong_case()
{
    let mut cfg = cfg_all_on();
    cfg.behaviour.case_sensitive = true;
    let entries = vec![entry("firefox", "firefox", "", &[])];
    let result = filter_entries(&entries, "FIREFOX", &cfg, &Default::default());
    assert!(result.is_empty());
}

#[test]
fn case_sensitive_matches_exact_case()
{
    let mut cfg = cfg_all_on();
    cfg.behaviour.case_sensitive = true;
    let entries = vec![entry("Firefox", "firefox", "", &[])];
    let result = filter_entries(&entries, "Firefox", &cfg, &Default::default());
    assert_eq!(result.len(), 1);
}

// ── scoring order ─────────────────────────────────────────────────────────────

#[test]
fn score_0_prefix_beats_score_1_contains()
{
    let entries = vec![
        entry("My Browser Firefox", "a", "", &[]), // name contains → score 1
        entry("Firefox", "b", "", &[]),            // name prefix   → score 0
    ];
    let result = filter_entries(&entries, "firefox", &cfg_all_on(), &Default::default());
    assert_eq!(result[0].name, "Firefox");
}

#[test]
fn score_1_name_beats_score_2_keyword()
{
    let entries = vec![
        entry("App", "app", "", &["firefox"]), // kw match → score 2
        entry("Firefox Extra", "x", "", &[]),  // name contains → score 1
    ];
    let result = filter_entries(&entries, "firefox", &cfg_all_on(), &Default::default());
    assert_eq!(result[0].name, "Firefox Extra");
}

#[test]
fn score_2_keyword_beats_score_3_exec()
{
    let entries = vec![
        entry("App A", "firefox-bin", "", &[]), // exec match → score 3
        entry("App B", "b", "", &["firefox"]),  // kw match  → score 2
    ];
    let result = filter_entries(&entries, "firefox", &cfg_all_on(), &Default::default());
    assert_eq!(result[0].name, "App B");
}

#[test]
fn score_3_exec_beats_score_4_comment()
{
    let entries = vec![
        entry("App A", "x", "uses firefox internally", &[]), // comment → score 4
        entry("App B", "firefox", "", &[]),                  // exec    → score 3
    ];
    let result = filter_entries(&entries, "firefox", &cfg_all_on(), &Default::default());
    assert_eq!(result[0].name, "App B");
}

#[test]
fn same_score_sorted_alphabetically()
{
    let entries = vec![
        entry("Zebra Browser", "zebra", "", &[]),
        entry("Alpha Browser", "alpha", "", &[]),
        entry("Mango Browser", "mango", "", &[]),
    ];
    let result = filter_entries(&entries, "browser", &cfg_name_only(), &Default::default());
    assert_eq!(result[0].name, "Alpha Browser");
    assert_eq!(result[1].name, "Mango Browser");
    assert_eq!(result[2].name, "Zebra Browser");
}

// ── all search fields disabled ────────────────────────────────────────────────

#[test]
fn all_search_fields_disabled_returns_empty()
{
    let mut cfg = cfg_all_on();
    cfg.behaviour.search_name = false;
    cfg.behaviour.search_comment = false;
    cfg.behaviour.search_exec = false;
    cfg.behaviour.search_keywords = false;

    let entries = vec![entry("Firefox", "firefox", "browser", &["web"])];
    let result = filter_entries(&entries, "firefox", &cfg, &Default::default());
    assert!(result.is_empty());
}

// ── multiple results ──────────────────────────────────────────────────────────

#[test]
fn multiple_matching_entries_all_returned()
{
    let entries = vec![
        entry("Firefox", "firefox", "", &[]),
        entry("Chromium", "chromium", "", &[]),
        entry("Vim", "vim", "", &[]),
    ];
    let result = filter_entries(&entries, "i", &cfg_name_only(), &Default::default());
    // Firefox and Chromium and Vim all contain 'i'
    assert_eq!(result.len(), 3);
}
