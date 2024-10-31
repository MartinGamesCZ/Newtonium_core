use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{ Arc, Mutex };
use gdk::keys::constants::l;
use once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::{ Application, ApplicationWindow };
use std::sync::mpsc;
use tungstenite::{ connect, Message };

static GLOBAL_CTX: Lazy<Arc<Mutex<Option<mpsc::Sender<String>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

#[no_mangle]
pub extern "C" fn create_application(id: *const c_char) -> Application {
    let id = (unsafe { CStr::from_ptr(id) }).to_str().unwrap();

    let application = Application::builder().application_id(id).build();

    application
}

#[no_mangle]
pub extern "C" fn create_window(application: Application, title: *const c_char) {
    let title = (unsafe { CStr::from_ptr(title) }).to_str().unwrap();

    println!("Creating window with title: {}", title);

    let global_ctx = Arc::clone(&GLOBAL_CTX);

    application.connect_activate(move |app| {
        let (tx, rx) = mpsc::channel::<String>();
        let (mut socket, response) = connect("ws://localhost:8080").expect("Can't connect");

        {
            let mut ctx = global_ctx.lock().unwrap();
            *ctx = Some(tx.clone());
        }

        let window = ApplicationWindow::builder()
            .application(app)
            .title(title)
            .default_width(350)
            .build();

        window.show_all();

        socket.send(Message::Text("init~".into())).unwrap();

        let (gui_tx, gui_rx) = gtk::glib::MainContext::channel(gtk::glib::Priority::default());

        std::thread::spawn(move || {
            loop {
                let msg = socket.read();

                if let Ok(msg) = msg {
                    let msg = msg.to_text().unwrap();

                    gui_tx.send(msg.to_string()).unwrap();
                }
            }
        });

        gui_rx.attach(None, move |msg| {
            window.set_title(&msg);
            gtk::glib::ControlFlow::Continue
        });
    });

    application.run_with_args::<String>(&[]);
}

#[no_mangle]
pub extern "C" fn modify_component(data: *const c_char) {
    let data = (unsafe { CStr::from_ptr(data) }).to_str().unwrap();

    println!("Modifying component with data: {}", data);

    let tx_option = {
        let ctx = GLOBAL_CTX.lock().unwrap();
        ctx.clone()
    }; // `ctx` is dropped here

    if let Some(tx) = tx_option {
        tx.send(data.to_string()).unwrap();
    } else {
        println!("No sender available");
    }
}
