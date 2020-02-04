use std::ffi::c_void;
use std::mem::{size_of, transmute};
use std::ptr::null_mut;
use std::rc::Rc;

use gtk::prelude::*;
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::HWND,
    },
    um::{
        shellapi::{Shell_NotifyIconA, NIF_ICON, NIF_MESSAGE, NIM_ADD, NOTIFYICONDATAA},
        winuser::{
            CallWindowProcA, GetAsyncKeyState, GetWindowLongPtrA, LoadIconW, RegisterHotKey,
            SetWindowLongPtrA, UnregisterHotKey, GWL_WNDPROC, IDI_APPLICATION, WM_APP, WM_HOTKEY,
            WM_LBUTTONDBLCLK, WNDPROC,
        },
    },
};

use crate::view::*;

use super::MAIN_VIEW;

pub static mut VIEW_HWND: Option<HWND> = None;
pub static mut GTK_PROC: WNDPROC = None;

const WM_TRAY: UINT = WM_APP + 1;

/// 为了创建托盘图标以及快捷键等, 需要获取到窗体的 HWND, gtk-sys 没有暴露对应的方法
/// 只能自己加 extern "C" 来导入
#[link(name = "gdk-3")]
extern "C" {
    pub fn gdk_win32_window_get_handle(window: *mut c_void) -> HWND;
}

pub fn register_hotkey(mod_code: u32, key_code: u32, id: i32) -> bool {
    unsafe { RegisterHotKey(VIEW_HWND.unwrap(), id, mod_code, key_code) == 1 }
}

pub fn unregister_hotkey(id: i32) -> bool {
    unsafe { UnregisterHotKey(VIEW_HWND.unwrap(), id) == 1 }
}

pub fn init_tray(view: Rc<View>) {
    unsafe {
        let display = view.window.get_window().unwrap().as_ptr();
        let hwnd = gdk_win32_window_get_handle(display as *mut c_void);
        VIEW_HWND = Some(hwnd);

        let mut nid = NOTIFYICONDATAA::default();
        nid.cbSize = size_of::<NOTIFYICONDATAA>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        nid.uFlags = NIF_MESSAGE | NIF_ICON;
        nid.uCallbackMessage = WM_APP + 1;
        nid.hIcon = LoadIconW(null_mut(), IDI_APPLICATION);

        Shell_NotifyIconA(NIM_ADD, &mut nid);
    };
}

pub fn init(_view: Rc<View>) {
    // 由于需要接受热键的消息, 需要处理 WM_HOTKEY 消息, gtk::timeout_add 和 PeekMessage 的方式似乎会漏掉很多消息.
    // 而 gtk::idle_add 会导致 CPU 占用率显著上升, 并且也会漏掉消息.
    // 替换了原本 gtk 上的 WNDPROC, 用自定义的对其进行包裹, 处理 WM_HOTKEY 和 WM_TRAY 消息.
    unsafe {
        let old_proc = GetWindowLongPtrA(VIEW_HWND.unwrap(), GWL_WNDPROC);
        GTK_PROC = Some(transmute(old_proc));
        SetWindowLongPtrA(VIEW_HWND.unwrap(), GWL_WNDPROC, proc_wrapper as isize);
    }
}

pub fn is_key_release(codes: &[u32]) -> bool {
    let mut release = false;
    for &code in codes {
        let r = unsafe { GetAsyncKeyState(code as i32) };
        let r = (r >> 8) == 0;
        release |= r;
    }

    release
}

unsafe extern "system" fn proc_wrapper(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAY => match lparam as u32 {
            WM_LBUTTONDBLCLK => {
                MAIN_VIEW.as_ref().map(|view| view.show());
            }
            _ => {}
        },
        WM_HOTKEY => match wparam {
            1 => {
                MAIN_VIEW.as_ref().map(|view| view.toggle());
            }
            2 => {
                MAIN_VIEW.as_ref().map(|view| view.unmute());
            }
            _ => {}
        },
        _ => {
            if let Some(ref _proc) = GTK_PROC {
                return CallWindowProcA(GTK_PROC, hwnd, msg, wparam, lparam);
            }
        }
    }

    1
}
