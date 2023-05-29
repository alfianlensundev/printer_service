
mod windows;

pub fn get_printers() -> String {
    if cfg!(windows) {
        return windows::get_printers();
    }

    panic!("Unsupported OS");
}