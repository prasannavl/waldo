extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::os::raw;

pub fn console_set_vterm() {
    unsafe {
        let handle = kernel32::GetConsoleWindow();
        kernel32::SetConsoleMode(handle as *mut raw::c_void, 0x0200);
    }
}

pub fn display_off() {
    unsafe {
        let hwnd = -1 as isize as winapi::HWND;
        let v = 2 as winapi::LPARAM;
        user32::SendNotifyMessageW(hwnd, winapi::WM_SYSCOMMAND, winapi::SC_MONITORPOWER, v);
    }
}
