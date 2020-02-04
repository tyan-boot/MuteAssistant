use std::rc::Rc;

use crate::view::View;

pub static mut MAIN_VIEW: Option<Rc<View>> = None;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(target_os = "windows")]
pub use win32::*;