use std::ffi::OsStr;
use std::io;
use std::mem;
use std::os::raw::c_ushort;
use std::os::windows::ffi::OsStrExt;
use std::process::ExitStatus;

use windows_sys::Win32::System::Com::{
    CoInitializeEx, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
};
use windows_sys::Win32::System::Threading::GetExitCodeProcess;
use windows_sys::Win32::System::Threading::WaitForSingleObject;
use windows_sys::Win32::System::Threading::INFINITE;
use windows_sys::Win32::UI::Shell::{ShellExecuteExW, SHELLEXECUTEINFOW};
use windows_sys::Win32::UI::WindowsAndMessaging::{SW_HIDE, SW_SHOW};

use crate::Command;

unsafe fn win_runas(cmd: *const c_ushort, args: *const c_ushort, show: bool) -> u32 {
    let code = 0;
    let sei: SHELLEXECUTEINFOW = mem::zeroed();
    let verb = "runas\0".encode_utf16().collect::<Vec<u16>>();
    CoInitializeEx(NULL, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);

    sei.cbSize = mem::size_of::<SHELLEXECUTEINFOW>();
    sei.lpVerb = verb.as_ptr();
    sei.lpFile = cmd;
    sei.lpParameters = args;
    sei.nShow = if show { SW_NORMAL } else { SW_HIDE };

    if !ShellExecuteExW(&sei) || sei.hProcess == ptr::null() {
        return -1;
    }

    WaitForSingleObject(sei.hProcess, INFINITE);

    if GetExitCodeProcess(sei.hProcess, &code) == 0 {
        -1
    } else {
        code
    }
}

pub fn runas_impl(cmd: &Command) -> io::Result<ExitStatus> {
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
                    c => params.push(c),
                }
            }
            params.push('"');
        }
    }

    let file = OsStr::new(&cmd.command)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let params = OsStr::new(&params)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();

    unsafe {
        let show = if cmd.hide { 0 } else { 1 };
        Ok(mem::transmute(win_runas(
            file.as_ptr(),
            params.as_ptr(),
            show,
        )))
    }
}
