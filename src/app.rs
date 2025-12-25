use crate::api::DictionaryClient;
use crate::ui::AppWindow;
use adw::prelude::*;
use adw::Application;
use anyhow::Result;
use glib::MainContext;
use gtk::glib;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

pub struct App {
    application: Application,
    window: Arc<Mutex<Option<Arc<AppWindow>>>>,
    client: DictionaryClient,
    runtime: Runtime,
}

impl App {
    pub fn new() -> Self {
        let application = Application::builder()
            .application_id("com.aynary.Dictionary")
            .build();

        let runtime = Runtime::new().expect("Failed to create tokio runtime");

        Self {
            application,
            window: Arc::new(Mutex::new(None)),
            client: DictionaryClient::new(),
            runtime,
        }
    }

    pub fn application(&self) -> &Application {
        &self.application
    }

    pub fn setup(&mut self) {
        let application = self.application.clone();
        let client = self.client.clone();
        let window_ref = Arc::clone(&self.window);
        let runtime_handle = self.runtime.handle().clone();

        // Create window
        let window = Arc::new(AppWindow::new(&application));
        
        // Setup search entry handler
        let search_entry = window.search_entry.clone();
        let definition_view = window.definition_view.clone();
        let client_clone = client.clone();
        let runtime_clone = runtime_handle.clone();

        search_entry.connect_activate(move |entry| {
            let word = entry.text().to_string();
            if word.is_empty() {
                return;
            }

            definition_view.buffer().set_text("Loading...");

            let client = client_clone.clone();
            let view = definition_view.clone();
            runtime_clone.spawn(async move {
                match client.lookup(&word).await {
                    Ok(entries) => {
                        let formatted = client.format_entry(&entries);
                        MainContext::default().invoke(move || {
                            view.buffer().set_text(&formatted);
                        });
                    }
                    Err(e) => {
                        let error_msg = format!("Error: {}", e);
                        MainContext::default().invoke(move || {
                            view.buffer().set_text(&error_msg);
                        });
                    }
                }
            });
        });

        // Store window reference
        {
            let mut window_guard = window_ref.lock().unwrap();
            *window_guard = Some(window.clone());
        }

        // Handle application activation
        self.application.connect_activate(move |_app| {
            let window_guard = window_ref.lock().unwrap();
            if let Some(w) = window_guard.as_ref() {
                w.show();
            }
        });
    }

    pub fn lookup_word(&mut self, word: &str) -> Result<String> {
        let window_guard = self.window.lock().unwrap();
        let window = window_guard
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Window not initialized"))?;

        window.set_loading(true);
        window.show();

        let client = self.client.clone();
        let definition_view = window.definition_view.clone();
        let runtime_handle = self.runtime.handle().clone();

        runtime_handle.spawn(async move {
            match client.lookup(word).await {
                Ok(entries) => {
                    let formatted = client.format_entry(&entries);
                    MainContext::default().invoke(move || {
                        definition_view.buffer().set_text(&formatted);
                    });
                }
                Err(e) => {
                    let error_msg = format!("Error: {}", e);
                    MainContext::default().invoke(move || {
                        definition_view.buffer().set_text(&error_msg);
                    });
                }
            }
        });

        Ok(String::from("Lookup initiated"))
    }

    pub fn show_window(&self) {
        let window_guard = self.window.lock().unwrap();
        if let Some(window) = window_guard.as_ref() {
            window.show();
        }
    }

    pub fn lookup_and_show(&mut self, word: &str) -> Result<()> {
        self.show_window();
        self.lookup_word(word)?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

