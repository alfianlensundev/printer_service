
mod windows;

pub fn get_printers() -> String {
    if cfg!(windows) {
        return windows::get_printers();
    }

    panic!("Unsupported OS");
}

pub fn get_printers_by_name(name: String) -> String {
    if cfg!(windows) {
        return windows::get_printers_by_name(name);
    }

    panic!("Unsupported OS");
}