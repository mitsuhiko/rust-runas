use std::io;
use std::process;
use std::process::ExitStatus;

use crate::Command;

pub fn runas_impl(cmd: &Command) -> io::Result<ExitStatus> {
    let mut executor = "";
    match which::which("sudo") {
        Ok(_) => executor = "sudo",
        Err(_) => {}
    }
    // Detect if doas is installed
    match which::which("doas") {
        Ok(_) => {
            // Prefer using sudo
            if executor == "" {
                executor = "doas"
            }
        }
        Err(_) => {}
    }
    if executor != "sudo" && executor != "doas" {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Commands sudo or doas not found!",
        ));
    }
    let mut c = process::Command::new(executor);
    if cmd.force_prompt && executor == "sudo" {
        // Forces passwork re-prompting
        c.arg("-k");
    }
    c.arg("--").arg(&cmd.command).args(&cmd.args[..]).status()
}
