
mod windows;

fn main() {
    get_printers();
}

pub fn get_printers() -> String {
    if cfg!(windows) {
        return windows::get_printers();
    }

    panic!("Unsupported OS");
}