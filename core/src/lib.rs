use std::{ ffi::CStr, os::raw::c_char };
use tao::{
    event::{ Event, WindowEvent },
    event_loop::{ ControlFlow, EventLoop, EventLoopBuilder },
};
use window::{ create_webview, create_window };

#[cfg(
    not(any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android"))
)]
use tao::platform::unix::EventLoopBuilderExtUnix;

mod window;

#[no_mangle]
pub extern "C" fn open_window(
    title: *const c_char,
    url: *const c_char,
    instance_secret: *const c_char
) -> () {
    let title = unsafe { CStr::from_ptr(title) };
    let url = unsafe { CStr::from_ptr(url) };
    let instance_secret = unsafe { CStr::from_ptr(instance_secret) };

    #[cfg(
        not(
            any(
                target_os = "windows",
                target_os = "macos",
                target_os = "ios",
                target_os = "android"
            )
        )
    )]
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();

    #[cfg(
        any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android")
    )]
    let event_loop = EventLoop::new();

    let window = create_window(&event_loop, title.to_str().unwrap_or("Newtonium"));
    let _builder = create_webview(
        &window,
        url.to_str().unwrap_or("https://example.com"),
        instance_secret.to_str().unwrap_or("")
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } =>
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                }
            _ => (),
        }
    });
}
