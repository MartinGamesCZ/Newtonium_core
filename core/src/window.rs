use std::any::Any;

use tao::{event_loop::EventLoop, window::WindowBuilder};
use wry::WebViewBuilder;

pub fn create_window(event_loop: &EventLoop<()>, title: &str) -> tao::window::Window {
    WindowBuilder::new()
        .with_title(title)
        .build(event_loop)
        .unwrap()
}

pub fn create_webview(window: &tao::window::Window, url: &str) -> wry::WebView {
    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let builder = WebViewBuilder::new(&window);

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let builder = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        WebViewBuilder::new_gtk(vbox)
    };

    builder.with_url(url).build().unwrap()
}
