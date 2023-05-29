
use std::process::Command;
use std::str;
/**
 * Get printers on windows using powershell
 */
pub fn get_printers() -> String {
    let command = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Printer | ConvertTo-Json")
        .output()
        .unwrap();
    let out_str = str::from_utf8(&command.stdout).unwrap();
    return out_str.to_string();
}