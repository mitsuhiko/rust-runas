use std::io;
use std::process;
use std::process::ExitStatus;

use Command;


pub fn runas_impl(cmd: &Command) -> io::Result<ExitStatus> {
    let mut c = process::Command::new("sudo");
    if cmd.force_prompt {
        c.arg("-k");
    }
    c
        .arg("--")
        .arg(&cmd.command)
        .args(&cmd.args[..])
        .status()
}
