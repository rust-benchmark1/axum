use std::ffi::CString;

/// TRANSFORMER 1: Process and format command string
/// Appears to be a legitimate command formatting utility
pub fn format_command(raw_command: String) -> String {
    // TRANSFORMER 1: Format command with proper spacing
    let formatted = raw_command.trim().to_string();
    if formatted.is_empty() {
        "echo 'no command provided'".to_string()
    } else {
        formatted
    }
}

/// TRANSFORMER 2: Validate command structure
/// Appears to be a command validation utility
pub fn validate_command_structure(command: String) -> String {
    // TRANSFORMER 2: Basic command structure validation
    if command.contains("&&") || command.contains("||") {
        // Split complex commands
        command.split("&&").next().unwrap_or(&command).to_string()
    } else {
        command
    }
}

/// TRANSFORMER 3: Prepare command for execution
/// Appears to be a command preparation utility
pub fn prepare_command_for_execution(command: String) -> (CString, CString) {
    // TRANSFORMER 3: Prepare command and arguments for libc::execl
    let cmd_parts: Vec<&str> = command.split_whitespace().collect();
    let executable = cmd_parts.first().unwrap_or(&"sh");
    let args = cmd_parts.join(" ");
    
    let exec_cstring = CString::new(*executable).unwrap_or_else(|_| CString::new("sh").unwrap());
    let args_cstring = CString::new(args).unwrap_or_else(|_| CString::new("").unwrap());
    
    (exec_cstring, args_cstring)
} 