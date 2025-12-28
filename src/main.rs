mod api;
mod app;
mod ui;
mod dbus_service;
mod clipboard_monitor;

use app::App;
use adw::prelude::*;
use gtk4::{CssProvider, StyleContext};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    // Initialize GTK
    adw::init().expect("Failed to initialize Adwaita");
    load_css();

    let mut app = App::new();
    app.setup();
    
    // Create a channel for DBus to communicate with the app
    let (sender, receiver) = mpsc::channel::<dbus_service::DbusCommand>();
    
    // Store app in Arc for DBus service
    let app_arc = Arc::new(Mutex::new(app));
    
    // Start DBus service in a separate thread
    let sender_for_dbus = sender.clone();
    thread::spawn(move || {
        let service = dbus_service::DictionaryService::new(sender_for_dbus);
        if let Err(e) = service.start() {
            eprintln!("DBus service error: {}", e);
        }
    });

    // Handle DBus commands on the main thread using glib idle callbacks
    let app_main = Arc::clone(&app_arc);
    
    // Set up a periodic check for messages (using glib timeout)
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        // Try to receive messages
        while let Ok(cmd) = receiver.try_recv() {
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
        }
        glib::ControlFlow::Continue
    });

    // Run the GTK application
    {
        let app_guard = app_arc.lock().unwrap();
        app_guard.application().run();
    }
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(
        br#"
        .brand-title {
            font-size: 34px;
            font-weight: 700;
            letter-spacing: -0.5px;
        }

        .brand-subtitle {
            font-size: 13px;
            color: alpha(@theme_fg_color, 0.65);
        }

        .search-entry {
            padding: 8px 12px;
            border-radius: 10px;
        }

        .definition-card {
            background: @window_bg_color;
            border-radius: 14px;
            padding: 8px;
            box-shadow: 0 12px 24px alpha(black, 0.08);
        }

        .definition-text {
            font-size: 14px;
        }

        .placeholder-icon {
            color: alpha(@theme_fg_color, 0.35);
        }

        .placeholder-title {
            font-size: 18px;
            font-weight: 600;
            color: alpha(@theme_fg_color, 0.85);
        }

        .placeholder-subtitle {
            font-size: 13px;
            color: alpha(@theme_fg_color, 0.6);
        }
        "#,
    );

    if let Some(display) = gtk4::gdk::Display::default() {
        StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
