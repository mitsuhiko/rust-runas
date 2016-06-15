//! This library implements basic support for running a command in an elevated context.
//!
//! In particular this runs a command through "sudo" or other platform equivalents.
use std::io;
use std::mem;
use std::process::ExitStatus;
use std::ffi::{OsStr, OsString};
use std::os::raw::c_ushort;

/// A process builder for elevated execution
pub struct Command {
    command: OsString,
    args: Vec<OsString>,
    force_prompt: bool,
    hide: bool,
    from_gui: bool,
}

/// The `Command` type acts as a process builder for spawning programs that run in
/// an elevated context.
///
/// Example:
///
/// ```rust,no_run
/// use runas::Command;
/// let status = Command::new("cmd").status();
/// ```
impl Command {

    /// Creates a new command type for a given program.
    ///
    /// The default configuration is to spawn without arguments, to be visible and
    /// to not be launched from a GUI context.
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        Command {
            command: program.as_ref().to_os_string(),
            args: vec![],
            hide: false,
            from_gui: false,
            force_prompt: true,
        }
    }

    /// Add an argument to pass to the program.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.args.push(arg.as_ref().to_os_string());
        self
    }

    /// Add multiple arguments to pass to the program.

    pub fn args<S: AsRef<OsStr>>(&mut self, args: &[S]) -> &mut Command {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    /// Controls the visibility of the program on supported platforms.  The default is
    /// to launch the program visible.
    pub fn show(&mut self, val: bool) -> &mut Command {
        self.hide = !val;
        self
    }

    /// Controls the GUI context.  The default behavior is to assume that the program is
    /// launched from a command line (not from GUI).  This primarily controls how the
    /// elevation prompt is rendered.  On some platforms like Windows the elevation prompt
    /// is always a GUI element.
    pub fn from_gui(&mut self, val: bool) -> &mut Command {
        self.from_gui = val;
        self
    }

    /// Can disable the prompt forcing for supported platforms.  Mostly this allows sudo
    /// on unix platforms to not prompt for a password.
    pub fn force_prompt(&mut self, val: bool) -> &mut Command {
        self.force_prompt = val;
        self
    }

    /// Executes a command as a child process, waiting for it to finish and
    /// collecting its exit status.
    pub fn status(&mut self) -> io::Result<ExitStatus> {
        runas_impl(&self)
    }
}

#[cfg(windows)]
fn runas_impl(cmd: &Command) -> io::Result<ExitStatus> {
    use std::os::windows::ffi::OsStrExt;
    extern "C" {
        fn rust_win_runas(cmd: *const c_ushort, args: *const c_ushort, show: i32) -> u32;
    }

    let mut params = String::new();
    for arg in cmd.args.iter() {
        let arg = arg.to_string_lossy();
        params.push(' ');
        if arg.len() == 0 {
            params.push_str("\"\"");
        } else if arg.find(&[' ', '\t', '"'][..]).is_none() {
            params.push_str(&arg);
        } else {
            params.push('"');
            for c in arg.chars() {
                match c {
                    '\\' => params.push_str("\\\\"),
                    '"' => params.push_str("\\\""),
                    c => params.push(c)
                }
            }
            params.push('"');
        }
    }

    let file = OsStr::new(&cmd.command).encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let params = OsStr::new(&params).encode_wide().chain(Some(0)).collect::<Vec<_>>();

    unsafe {
        let rv = rust_win_runas(file.as_ptr(), params.as_ptr(), if cmd.hide { 0 } else { 1 });
        Ok(mem::transmute(rv))
    }
}

#[cfg(not(windows))]
fn runas_impl(cmd: &Command) -> io::Result<ExitStatus> {
    use std::process;
    let mut cmd = process::Command::new("sudo");
    if cmd.force_prompt {
        cmd.arg("-k");
    }
    cmd
        .arg("--")
        .arg(cmd.command)
        .args(cmd.args)
        .status()
}
