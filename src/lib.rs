#![allow(unused)]

pub mod dimension;
pub mod util;
pub mod platform;
pub mod window;

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;
    use windows::{
        core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
        Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
    };
    use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
    use windows::Win32::Graphics::Gdi::{BeginPaint, CreateSolidBrush, EndPaint, FillRect, PAINTSTRUCT, UpdateWindow};
    use windows::Win32::UI::ColorSystem::RGBCOLOR;
    use windows::Win32::UI::Controls::MARGINS;

    #[test]
    fn test_app() {
    }

    #[test]
    fn test_hwnd() -> Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None)?;
            debug_assert!(instance.0 != 0);

            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance.into(),
                lpszClassName: window_class,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let hwnd = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                window_class,
                s!("This is a sample window"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                None,
            );
            debug_assert!(hwnd.0 != 0);

            let mut message = MSG::default();

            while GetMessageA(&mut message, None, 0, 0).into() {
                DispatchMessageA(&message);
            }

            let style = GetWindowLongA(hwnd, GWL_EXSTYLE) | (WS_EX_LAYERED.0 as i32);
            SetWindowLongA(hwnd, GWL_EXSTYLE, style);
            SetLayeredWindowAttributes(hwnd, COLORREF(127), 0, LWA_ALPHA);

            Ok(())
        }
    }

    extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_CREATE => {
                    // println!("WM_CREATE");
                    let mut rect = Default::default();
                    GetWindowRect(window, &mut rect);
                    SetWindowPos(window, None, 0, 0, 0, 0, SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_DRAWFRAME);

                    let style = GetWindowLongA(window, GWL_STYLE);
                    let style: i32 = style & !WS_MAXIMIZEBOX.0 as i32 | WS_BORDER.0 as i32;
                    SetWindowLongA(window, GWL_STYLE, style);
                    DwmExtendFrameIntoClientArea(window, &MARGINS {
                        cxLeftWidth: 1,
                        cxRightWidth: 1,
                        cyBottomHeight: 1,
                        cyTopHeight: 1
                    });

                    UpdateWindow(window);
                    LRESULT(0)
                }
                WM_PAINT => {
                    // println!("WM_PAINT");
                    let mut ps = PAINTSTRUCT::default();
                    let hdc = BeginPaint(window, &mut ps);

                    let mut rect = RECT::default();
                    rect.left = 0;
                    rect.right = 128;
                    rect.top = 0;
                    rect.bottom = 64;

                    GetClientRect(window, &mut rect);
                    println!("{:?}", rect);
                    let color: RGBCOLOR = RGBCOLOR {
                        red: 63,
                        green: 127,
                        blue: 127
                    };
                    let color = 16777215;
                    let x = 0x8f << 16 | 0x8f << 8 | 0x00;
                    println!("{x}");
                    // println!("{color}");
                    let brush = CreateSolidBrush(COLORREF(x));
                    FillRect(hdc, &rect, brush);

                    EndPaint(window, &ps);

                    LRESULT(0)
                }
                WM_DESTROY => {
                    // println!("WM_DESTROY");
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(window, message, wparam, lparam),
            }
        }
    }
}
