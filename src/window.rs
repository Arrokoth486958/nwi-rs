struct Window {
    #[cfg(target_os = "windows")]
    inner: crate::platform::win32::Window,
    #[cfg(target_os = "linux")]
    inner: crate::platform::x11::Window,
    #[cfg(target_os = "macos")]
    inner: crate::platform::macos::Window,
}