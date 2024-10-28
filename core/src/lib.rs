use std::{ ffi::CStr, os::raw::c_char };

use qt_core::QString;
use qt_qml::QQmlApplicationEngine;
use qt_gui::{ QGuiApplication, QIcon };

#[no_mangle]
pub extern "C" fn open_window(
    qml: *const c_char,
    icon: *const c_char,
    app_name: *const c_char
) -> i32 {
    let c_str = unsafe { CStr::from_ptr(qml) };
    let qml_str = c_str.to_str().unwrap_or("");

    let c_str = unsafe { CStr::from_ptr(icon) };
    let icon_str = c_str.to_str().unwrap_or("");

    let c_str = unsafe { CStr::from_ptr(app_name) };
    let app_name_str = c_str.to_str().unwrap_or("");

    QGuiApplication::init(|_| unsafe {
        let engine = QQmlApplicationEngine::new();
        engine.load_q_string(&QString::from_std_str(qml_str));

        QGuiApplication::set_window_icon(&QIcon::from_q_string(&QString::from_std_str(icon_str)));

        QGuiApplication::set_application_display_name(&QString::from_std_str(app_name_str));

        QGuiApplication::exec()
    })
}
