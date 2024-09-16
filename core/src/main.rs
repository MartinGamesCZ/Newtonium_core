use ipc::register_ipc_command;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::unix::EventLoopBuilderExtUnix,
    window::WindowBuilder,
};
use window::{create_webview, create_window};

mod ipc;
mod window;

#[tokio::main]
async fn main() {
    register_ipc_command("window_open", |args: Vec<String>| {
        println!("confirm::ok");
        tokio::task::spawn_blocking(move || {
            let event_loop = EventLoopBuilder::new().with_any_thread(true).build();

            let win = create_window(&event_loop, args[1].as_str());
            let _builder = create_webview(&win, args[2].as_str());

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                if let Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } = event
                {
                    *control_flow = ControlFlow::Exit
                }
            });
        });
    });
}
