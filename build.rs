#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set("ProductName", "yaav");
        res.set("FileDescription", "yaav");
        // res.set_icon("assets/icon.ico")
        res.compile().expect("Failed to compile Windows resources");
    }
}