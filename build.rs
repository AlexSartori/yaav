#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set("ProductName", "yaav");
        res.set("FileDescription", "yaav");
        res.compile().expect("Failed to compile Windows resources");
    }
    // res.set_icon("assets/icon.ico")
}