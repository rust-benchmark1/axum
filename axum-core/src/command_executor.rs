use std::ffi::CString;

//CWE-78: Execute command using libc::execl
// This function acts as a sink for OS command injection vulnerability testing
pub unsafe fn execute_command(executable: CString, args: CString) -> i32 {
    //SINK
    libc::execl(
        executable.as_ptr(),
        executable.as_ptr(),
        args.as_ptr(),
        std::ptr::null::<i8>()
    )
} 