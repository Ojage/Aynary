mod api;
mod app;
mod ui;
mod dbus_service;
mod clipboard_monitor;

use app::App;
use adw::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Initialize GTK
    adw::init().expect("Failed to initialize Adwaita");

    let mut app = App::new();
    app.setup();
    
    // Create a channel for DBus to communicate with the app
    let (sender, receiver) = glib::MainContext::channel(glib::Priority::default());
    
    // Store app in Arc for DBus service
    let app_arc = Arc::new(Mutex::new(app));
    
    // Start DBus service in a separate thread
    let sender_clone = sender.clone();
    thread::spawn(move || {
        let service = dbus_service::DictionaryService::new(sender_clone);
        if let Err(e) = service.start() {
            eprintln!("DBus service error: {}", e);
        }
    });

    // Note: Clipboard monitoring is disabled for now as it requires more complex
    // implementation. Users can use keyboard shortcuts or browser extension instead.
    // To enable, uncomment below and implement proper clipboard change detection:
    //
    // thread::spawn(move || {
    //     let rt = tokio::runtime::Runtime::new().unwrap();
    //     rt.block_on(async {
    //         let mut monitor = clipboard_monitor::ClipboardMonitor::new();
    //         if let Err(e) = monitor.start().await {
    //             eprintln!("Clipboard monitor error: {}", e);
    //         }
    //     });
    // });

    // Handle DBus commands on the main thread
    let app_main = Arc::clone(&app_arc);
    receiver.attach(None, move |cmd| {
        let mut app_guard = app_main.lock().unwrap();
        match cmd {
            dbus_service::DbusCommand::LookupWord(word) => {
                let _ = app_guard.lookup_word(&word);
            }
            dbus_service::DbusCommand::ShowWindow => {
                app_guard.show_window();
            }
            dbus_service::DbusCommand::LookupAndShow(word) => {
                let _ = app_guard.lookup_and_show(&word);
            }
        }
        glib::Continue(true)
    });

    // Run the GTK application
    {
        let app_guard = app_arc.lock().unwrap();
        app_guard.application().run();
    }
}

