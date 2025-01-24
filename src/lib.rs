use std::ptr;
use windows::Win32::Foundation::{BOOL, HWND};
use windows::core::{PCWSTR, w};

#[repr(C)]
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

    unsafe {
        // This is "Unsafe" as we're going to write to memory our process doesn't own (AdiIRC/mIRC owns it)

        // Fix for https://dev.adiirc.com/issues/5808 (AdiIRC doesn't stop reading after a null terminator)
        if data.len() > result.len() {
            ptr::write_bytes(data.0.offset(result.len() as isize + 1) as *mut u16, 0, data.len() - result.len() - 1);
        }

        // Write the result to the data variable
        ptr::copy_nonoverlapping(result.as_ptr(), data.0 as *mut u16, result.len() + 1);
    }
    3 // The return value can be:
    // - 0 means that mIRC should /halt processing
    // - 1 means that mIRC should continue processing
    // - 2 means that it has filled the data variable with a command which it wants mIRC to perform, and has filled parms with the parameters to use, if any, when performing the command.
    // - 3 means that the DLL has filled the data variable with the result that $dll() as an identifier should return.
}

#[no_mangle]
pub extern "system" fn LoadDll(info: *mut LOADINFO) {
    unsafe {
        (*info).m_keep = BOOL(1);    // Keep the DLL loaded
        (*info).m_unicode = BOOL(1); // Use Unicode
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