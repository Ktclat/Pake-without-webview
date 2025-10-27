#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    // 指向本地打包好的 WebView2
    std::env::set_var("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER", "./WebView2Runtime");

    app_lib::run();
}
