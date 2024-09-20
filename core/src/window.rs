use std::{any::Any, sync::mpsc::Sender};

use tao::{event_loop::EventLoop, window::WindowBuilder};
use wry::WebViewBuilder;

pub fn create_window(event_loop: &EventLoop<()>, title: &str) -> tao::window::Window {
    WindowBuilder::new()
        .with_title(title)
        .build(event_loop)
        .unwrap()
}

pub fn create_webview(window: &tao::window::Window, url: &str, tx: Sender<String>) -> wry::WebView {
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

    builder
        .with_url(url)
        .with_devtools(true)
        .with_initialization_script("window.newtonium_ipc = { listeners: [], send: window.ipc.postMessage, onMessage: (fn) => window.newtonium_ipc.listeners.push(fn), _fire_recv: (data) => window.newtonium_ipc.listeners.forEach((fn) => fn(data)) }")
        .with_ipc_handler(|data| {
            println!("event::ipc::message::{}", data.body());
        })
        .build()
        .unwrap()
}
