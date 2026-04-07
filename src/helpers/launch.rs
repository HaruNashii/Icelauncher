// ============ IMPORTS ============
use std::process::Command as StdCommand;
use std::os::unix::process::CommandExt;




// ============ CRATES ============
use crate::helpers::desktop::tokenize;
use crate::ron::LauncherConfig;




// ============ FUNCTIONS ============
pub fn launch_app(exec: &str, config: &LauncherConfig, terminal: bool)
{
	let command = build_launch_command(exec, config, terminal);
	spawn_detached(&command);
}


fn build_launch_command(exec: &str, config: &LauncherConfig, terminal: bool) -> String
{
	let terminal_cmd = &config.behaviour.terminal_command;
	if terminal && !terminal_cmd.is_empty() {
		format!("{} {}", terminal_cmd.trim(), exec)
	} else {
		exec.to_string()
	}
}


fn spawn_detached(command: &str)
{
	let tokens = tokenize(command);
	let Some((program, args)) = tokens.split_first() else { return };

	// process_group(0) puts the child in its own process group so it is
	// fully detached from icelauncher's session and won't receive SIGHUP
	// when the launcher exits.
	let _ = StdCommand::new(program)
		.args(args)
		.stdin(std::process::Stdio::null())
		.stdout(std::process::Stdio::null())
		.stderr(std::process::Stdio::null())
		.process_group(0)
		.spawn();
}
