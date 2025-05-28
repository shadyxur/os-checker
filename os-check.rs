use std::process::Command;
use std::str;

fn main() {
    println!("Attempting to determine OS and version...");

    // Try to run 'uname' command for Unix-like systems
    match Command::new("uname").args(["-s", "-r"]).output() {
        Ok(output) => {
            if output.status.success() {
                let os_output = match str::from_utf8(&output.stdout) {
                    Ok(s) => s.trim(),
                    Err(e) => {
                        eprintln!("Failed to decode output: {}", e);
                        ""
                    }
                };
                if os_output.contains("Linux") {
                    println!("OS: Linux, Version: {}", os_output);
                } else if os_output.contains("Darwin") {
                    // For macOS, get additional version info
                    if let Ok(version) = Command::new("sw_vers").output() {
                        if let Ok(ver_str) = str::from_utf8(&version.stdout) {
                            println!("OS: macOS\nVersion info:\n{}", ver_str);
                        }
                    }
                } else {
                    println!("Detected Unix-like OS: {}", os_output);
                }
            } else {
                println!("Unix-like OS check failed. Switching to Windows detection...");
                check_windows_command();
            }
        }
        Err(_) => {
            println!("Unix-like OS check failed. Switching to Windows detection...");
            check_windows_command();
        }
    }
}

fn check_windows_command() {
    println!("Checking for Windows system...");

    let os_info = Command::new("cmd")
        .args(["/C", "wmic os get Caption,Version,BuildNumber /value"])
        .output();

    match os_info {
        Ok(output) => {
            if output.status.success() {
                match str::from_utf8(&output.stdout) {
                    Ok(info) => {
                        let info = info.trim().replace("\r\r\n", "\n");
                        if !info.is_empty() {
                            println!("Windows System Information:");
                            println!("{}", info);
                        } else {
                            eprintln!("No output from system command");
                        }
                    }
                    Err(e) => eprintln!("Failed to decode output: {}", e),
                }
            } else {
                eprintln!("Command executed but failed");
            }
        }
        Err(e) => {
            eprintln!("Failed to execute Windows command: {}", e);
            println!("Could not determine OS version");
        }
    }
}
