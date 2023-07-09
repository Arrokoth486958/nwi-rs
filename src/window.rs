use crate::dimension::Size;

// use crate::platform;
#[cfg(target_os = "windows")]
use crate::platform::win32 as platform;

pub struct Window {
    inner: platform::Window
}