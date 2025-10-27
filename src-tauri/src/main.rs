#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
std::env::set_var("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER", "./WebView2Runtime");
fn main() {
    app_lib::run()
}
