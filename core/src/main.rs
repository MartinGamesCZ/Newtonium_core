use ipc::register_ipc_command;
use std::io::{BufRead, Stdin};
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
    let event_loop = EventLoop::new();

    let window = create_window(&event_loop, "Newtonium Window");
    let builder = create_webview(&window, "https://example.com");

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        for line in reader.lines() {
            let command = line.unwrap_or_default();
            if !command.is_empty() {
                tx.send(command).unwrap();
            }
        }
    });

    println!("event::ready");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(command) = rx.try_recv() {
            let splitted = command.split("::");

            let cmd = splitted.clone().next().unwrap();
            let mut args: Vec<String> = Vec::new();

            splitted.for_each(|item| {
                args.push(item.to_string());
            });

            match cmd {
                "window_title_set" => {
                    window.set_title(&args[1]);

                    println!("confirm::ok");
                }
                "window_url_set" => {
                    let _ = builder.load_url(&args[1]);

                    println!("confirm::ok");
                }
                "exit" => {
                    *control_flow = ControlFlow::Exit;

                    println!("confirm::ok");
                }
                _ => {
                    println!("confirm::not_found");
                }
            }
        }

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
