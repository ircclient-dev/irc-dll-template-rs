use std::ptr;
use std::sync::OnceLock;
use std::time::Duration;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{BOOL, HWND};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct LOADINFO {
    // m_version contains the mIRC version number (AdiIRC uses a mIRC compatibility version number) in the low and high words.
    m_version: u32,
    // mHwnd contains the window handle to the main AdiIRC/mIRC window.
    m_hwnd: HWND,
    // m_keep is set to TRUE by default, indicating that the client will keep the DLL loaded after the call.
    //   - You can set mKeep to FALSE to make AdiIRC/mIRC unload the DLL after the call (which is how older versions of mIRC worked).
    m_keep: BOOL,
    // m_unicode indicates that strings are Unicode as opposed to ANSI.
    //   - For backward compatibility reasons, the default format is ANSI, however Unicode is generally recommended for new DLLs.
    m_unicode: BOOL,
    // m_beta contains the AdiIRC/mIRC beta version number, for public betas.
    m_beta: u32,
    // m_bytes specifies the maximum number of bytes (not characters) allowed in the data and parms variables.
    m_bytes: u32,
}

// Implement Sync and Send for LOADINFO
unsafe impl Sync for LOADINFO {}
unsafe impl Send for LOADINFO {}

static LOADINFO: OnceLock<LOADINFO> = OnceLock::new();

#[no_mangle]
pub extern "system" fn hello(
    _m_wnd: HWND,
    _a_wnd: HWND,
    data: PCWSTR,
    _parms: PCWSTR,
    _show: BOOL,
    _nopause: BOOL,
) -> i32 {
    let result = w!("Hello, world!");

    // Obtain the LOADINFO that we stored during the LoadDll call
    let loadinfo = LOADINFO.get();
    match loadinfo {
        Some(_loadinfo) => {
            unsafe {
                // This is "Unsafe" as we're going to write to memory our process doesn't own (AdiIRC/mIRC owns it)

                // Write the result to the data variable
                // TODO: We should make sure that the data variable is large enough to hold the result (by checking LOADINFO.m_bytes)
                ptr::copy_nonoverlapping(result.as_ptr(), data.0 as *mut u16, result.len() + 1);

                // Fix for https://dev.adiirc.com/issues/5808 (AdiIRC doesn't stop reading after a null terminator)
                if data.len() > result.len() {
                    ptr::write_bytes(
                        data.0.offset(result.len() as isize + 1) as *mut u16,
                        0,
                        data.len() - result.len() - 1,
                    );
                }
            }
            3 // The return value can be:
              // - 0 means that mIRC should /halt processing
              // - 1 means that mIRC should continue processing
              // - 2 means that it has filled the data variable with a command which it wants mIRC to perform, and has filled parms with the parameters to use, if any, when performing the command.
              // - 3 means that the DLL has filled the data variable with the result that $dll() as an identifier should return.
        }
        None => {
            eprintln!("Error: LOADINFO not initialised");
            return 0;
        }
    }
}

#[no_mangle]
pub extern "system" fn LoadDll(info: *mut LOADINFO) {
    #[cfg(debug_assertions)]
    {
        // Start a VS Code debugging session
        let url = format!(
            "vscode://vadimcn.vscode-lldb/launch/config?{{'request':'attach','pid':{}}}",
            std::process::id()
        );
        std::process::Command::new("cmd.exe")
            .arg("/C")
            .arg("code")
            .arg("--open-url")
            .arg(url)
            .output()
            .unwrap();
        std::thread::sleep(Duration::from_millis(1000)); // Wait for debugger to attach
    }

    unsafe {
        // Set mKeep and mUnicode to TRUE by default
        (*info).m_keep = BOOL(1); // Keep the DLL loaded
        (*info).m_unicode = BOOL(1); // Use Unicode

        // Store (a copy of) LOADINFO for later use.
        // - It's not clear from documentation if this memory will be available outside of LoadDll, so we assume it won't be.
        let result = LOADINFO.set((*info).clone());
        if let Err(e) = result {
            eprintln!("Error: LOADINFO already initialised: {:?}", e);
        }
    }
}

#[no_mangle]
pub extern "system" fn UnloadDll(_m_timeout: i32) -> i32 {
    // The m_timeout value can be:
    // - 0: UnloadDll() is being called due to a DLL being unloaded with /dll -u.
    // - 1: UnloadDll() is being called due to a DLL not being used for ten minutes. The UnloadDll() routine can return 0 to keep the DLL loaded, or 1 to allow it to be unloaded.
    // - 2: UnloadDll() is being called due to a DLL being unloaded when mIRC exits.

    1 // Allow DLL to be unloaded. This is only honoured when m_timeout is 1.
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::{BOOL, HWND};

    #[test]
    fn test_hello() {
        let mut loadinfo = LOADINFO {
            m_version: 0,
            m_hwnd: HWND(std::ptr::null_mut()),
            m_keep: BOOL(0),
            m_unicode: BOOL(0),
            m_beta: 0,
            m_bytes: 0,
        };
        LoadDll(&mut loadinfo);

        let m_wnd: HWND = HWND(std::ptr::null_mut());
        let a_wnd: HWND = HWND(std::ptr::null_mut());
        let data_str = "The quick brown fox jumps over the lazy dog";
        let data_vec: Vec<u16> = data_str.encode_utf16().collect();
        let data = PCWSTR(data_vec.as_ptr());
        let parms = PCWSTR::null();
        let show: BOOL = BOOL(0);
        let nopause: BOOL = BOOL(0);

        let result = hello(m_wnd, a_wnd, data, parms, show, nopause);
        unsafe { println!("Result: {:?}", data.to_string()); }
        unsafe { assert_eq!(data.as_wide(), w!("Hello, world!").as_wide()) };
        assert_eq!(result, 3);
    }
}
