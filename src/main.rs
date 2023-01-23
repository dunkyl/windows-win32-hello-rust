#![windows_subsystem = "windows"]

use windows::core::*;
use windows::Win32::{
    Foundation::*, Graphics::Gdi::HBRUSH, System::LibraryLoader::*, UI::WindowsAndMessaging::*,
};

extern "system" fn callback(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wp, lp) }
}

pub fn get_instance_handle() -> HINSTANCE {
    unsafe { GetModuleHandleA(PCSTR(std::ptr::null())).unwrap() }
}

fn main() {
    unsafe {
        let hinstance = get_instance_handle();

        RegisterClassExW(&WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(callback),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: HICON(0),
            hCursor: HCURSOR(0),
            hbrBackground: HBRUSH(0),
            lpszMenuName: std::mem::transmute::<_, PCWSTR>(0usize),
            lpszClassName: w!["my window class"],
            hIconSm: HICON(0),
        });

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!["my window class"],
            w!["my window"],
            WS_OVERLAPPEDWINDOW | WS_MINIMIZEBOX | WS_MAXIMIZEBOX,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            400,
            400,
            HWND(0),
            HMENU(0),
            hinstance,
            None,
        );

        ShowWindow(hwnd, SW_SHOW);

        loop {
            // event loop
            let mut msg = MSG::default();
            if GetMessageW(&mut msg, hwnd, 0, 0) == BOOL(0) {
                break;
            }
            match msg {
                MSG {
                    // close button
                    message: WM_NCLBUTTONDOWN,
                    wParam: WPARAM(20),
                    ..
                } => break,
                _ => {}
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
