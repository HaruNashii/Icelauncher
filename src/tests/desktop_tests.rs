use std::{io::Write, path::PathBuf};

use tempfile::NamedTempFile;

use crate::helpers::desktop::{parse_desktop_file, sanitize_exec};

// ── sanitize_exec ────────────────────────────────────────────────────────────

#[test]
fn sanitize_exec_strips_percent_placeholders()
{
    assert_eq!(sanitize_exec("firefox %u"), "firefox");
    assert_eq!(sanitize_exec("code %F"), "code");
    assert_eq!(sanitize_exec("gimp %f %U"), "gimp");
}

#[test]
fn sanitize_exec_keeps_normal_args()
{
    assert_eq!(sanitize_exec("kitty --hold"), "kitty --hold");
}

#[test]
fn sanitize_exec_no_placeholders_unchanged()
{
    assert_eq!(sanitize_exec("alacritty"), "alacritty");
}

#[test]
fn sanitize_exec_mixed_placeholders_and_flags()
{
    assert_eq!(sanitize_exec("app --flag %u --other"), "app --flag --other");
}

#[test]
fn sanitize_exec_only_placeholders_gives_empty()
{
    assert_eq!(sanitize_exec("%u %f %F"), "");
}

#[test]
fn sanitize_exec_empty_input()
{
    assert_eq!(sanitize_exec(""), "");
}

#[test]
fn sanitize_exec_extra_whitespace_normalized()
{
    // split_whitespace collapses internal spaces
    assert_eq!(sanitize_exec("app   arg"), "app arg");
}

#[test]
fn sanitize_exec_preserves_quoted_argument_as_single_token()
{
    assert_eq!(sanitize_exec("app --title \"My App\""), "app --title My App");
}

#[test]
fn sanitize_exec_single_quotes_work()
{
    assert_eq!(sanitize_exec("app --title 'My App'"), "app --title My App");
}

#[test]
fn sanitize_exec_percent_inside_quotes_stripped()
{
    assert_eq!(sanitize_exec("app \"%u\""), "app");
}

// ── parse_desktop_file ───────────────────────────────────────────────────────

fn write_desktop(content: &str) -> NamedTempFile
{
    let mut f = NamedTempFile::new().unwrap();
    write!(f, "{}", content).unwrap();
    f
}

fn parse(content: &str) -> Option<crate::AppEntry>
{
    let f = write_desktop(content);
    parse_desktop_file(&PathBuf::from(f.path()))
}

#[test]
fn parse_minimal_valid_entry()
{
    let entry = parse("[Desktop Entry]\nName=Test\nExec=test\nType=Application\n").unwrap();
    assert_eq!(entry.name, "Test");
    assert_eq!(entry.exec, "test");
}

#[test]
fn parse_full_entry_all_fields()
{
    let entry = parse(
        "[Desktop Entry]\nName=Firefox\nExec=firefox %u\nComment=Web Browser\nIcon=firefox\nKeywords=web;browser;internet;\nType=Application\n"
    ).unwrap();
    assert_eq!(entry.name, "Firefox");
    assert_eq!(entry.exec, "firefox"); // %u stripped
    assert_eq!(entry.comment, "Web Browser");
    assert_eq!(entry.icon, "firefox");
    assert_eq!(entry.keywords, vec!["web", "browser", "internet"]);
    assert!(!entry.terminal);
}

#[test]
fn parse_terminal_true_sets_flag()
{
    let entry =
        parse("[Desktop Entry]\nName=Htop\nExec=htop\nType=Application\nTerminal=true\n").unwrap();
    assert!(entry.terminal);
}

#[test]
fn parse_terminal_false_does_not_set_flag()
{
    let entry =
        parse("[Desktop Entry]\nName=App\nExec=app\nType=Application\nTerminal=false\n").unwrap();
    assert!(!entry.terminal);
}

#[test]
fn parse_no_display_true_returns_none()
{
    let result =
        parse("[Desktop Entry]\nName=Hidden\nExec=hidden\nType=Application\nNoDisplay=true\n");
    assert!(result.is_none());
}

#[test]
fn parse_missing_name_returns_none()
{
    let result = parse("[Desktop Entry]\nExec=test\nType=Application\n");
    assert!(result.is_none());
}

#[test]
fn parse_missing_exec_returns_none()
{
    let result = parse("[Desktop Entry]\nName=Test\nType=Application\n");
    assert!(result.is_none());
}

#[test]
fn parse_non_application_type_returns_none()
{
    let result = parse("[Desktop Entry]\nName=Link\nExec=link\nType=Link\n");
    assert!(result.is_none());
}

#[test]
fn parse_directory_type_returns_none()
{
    let result = parse("[Desktop Entry]\nName=Dir\nExec=dir\nType=Directory\n");
    assert!(result.is_none());
}

#[test]
fn parse_ignores_keys_outside_desktop_entry_section()
{
    // Fields in [Other Section] must be ignored
    let entry = parse(
        "[Other Section]\nName=Fake\nExec=fake\n\n[Desktop Entry]\nName=Real\nExec=real\nType=Application\n"
    ).unwrap();
    assert_eq!(entry.name, "Real");
    assert_eq!(entry.exec, "real");
}

#[test]
fn parse_only_first_name_is_used()
{
    // Duplicate Name= lines: only the first wins
    let entry =
        parse("[Desktop Entry]\nName=First\nName=Second\nExec=app\nType=Application\n").unwrap();
    assert_eq!(entry.name, "First");
}

#[test]
fn parse_keywords_lowercased_and_filtered()
{
    let entry =
        parse("[Desktop Entry]\nName=App\nExec=app\nType=Application\nKeywords=FOO;Bar;;baz;\n")
            .unwrap();
    // Empty segments filtered, all lowercased
    assert_eq!(entry.keywords, vec!["foo", "bar", "baz"]);
}

#[test]
fn parse_empty_keywords_gives_empty_vec()
{
    let entry =
        parse("[Desktop Entry]\nName=App\nExec=app\nType=Application\nKeywords=\n").unwrap();
    assert!(entry.keywords.is_empty());
}

#[test]
fn parse_icon_path_initially_none()
{
    let entry =
        parse("[Desktop Entry]\nName=App\nExec=app\nIcon=myicon\nType=Application\n").unwrap();
    // icon_path is resolved later; parse sets it to None
    assert!(entry.icon_path.is_none());
    assert_eq!(entry.icon, "myicon");
}

#[test]
fn parse_comment_empty_when_absent()
{
    let entry = parse("[Desktop Entry]\nName=App\nExec=app\nType=Application\n").unwrap();
    assert_eq!(entry.comment, "");
}

#[test]
fn parse_nonexistent_file_returns_none()
{
    let result = parse_desktop_file(&PathBuf::from("/nonexistent/path/fake.desktop"));
    assert!(result.is_none());
}

#[test]
fn parse_exec_percent_stripped()
{
    let entry =
        parse("[Desktop Entry]\nName=App\nExec=myapp %F --flag\nType=Application\n").unwrap();
    assert_eq!(entry.exec, "myapp --flag");
}
