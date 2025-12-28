use crate::api::DictionaryClient;
use crate::ui::AppWindow;
use adw::prelude::*;
use adw::Application;
use anyhow::Result;
use gtk4::glib;
use std::sync::{Arc, Mutex, mpsc};
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

        // Handle application activation - create window here (after startup signal)
        self.application.connect_activate(move |app| {
            let mut window_guard = window_ref.lock().unwrap();
            
            // Create window if it doesn't exist
            if window_guard.is_none() {
                // Create window
                let window = Arc::new(AppWindow::new(app));
                
                // Setup search entry handler
                let search_entry = window.search_entry.clone();
                let definition_view = window.definition_view.clone();
                let client_clone = client.clone();
                let runtime_clone = runtime_handle.clone();
                let window_ref_for_search = window_ref.clone();

                search_entry.connect_activate(move |entry| {
                    let word = entry.text().to_string();
                    if word.is_empty() {
                        return;
                    }

                    definition_view.buffer().set_text("Loading...");

                    let client = client_clone.clone();
                    let (tx, rx) = std::sync::mpsc::channel::<String>();
                    let window_ref_clone = window_ref_for_search.clone();

                    runtime_clone.spawn(async move {
                        let result = match client.lookup(&word).await {
                            Ok(entries) => client.format_entry(&entries),
                            Err(e) => format!("Error: {}", e),
                        };
                        let _ = tx.send(result);
                    });

                    // Poll for result on main thread
                    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
                        if let Ok(text) = rx.try_recv() {
                            let window_guard = window_ref_clone.lock().unwrap();
                            if let Some(window) = window_guard.as_ref() {
                                window.definition_view.buffer().set_text(&text);
                            }
                            glib::ControlFlow::Break
                        } else {
                            glib::ControlFlow::Continue
                        }
                    });
                });

                // Store window reference
                *window_guard = Some(window.clone());
            }
            
            // Show window
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
        let word = word.to_string();
        let (tx, rx) = std::sync::mpsc::channel::<String>();

        // Use tokio spawn for async work
        self.runtime.handle().spawn(async move {
            let result = match client.lookup(&word).await {
                Ok(entries) => client.format_entry(&entries),
                Err(e) => format!("Error: {}", e),
            };
            let _ = tx.send(result);
        });

        // Update UI from result channel on main thread
        let window_ref = Arc::clone(&self.window);
        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            if let Ok(text) = rx.try_recv() {
                let window_guard = window_ref.lock().unwrap();
                if let Some(window) = window_guard.as_ref() {
                    window.definition_view.buffer().set_text(&text);
                }
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
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

