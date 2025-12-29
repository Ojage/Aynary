use crate::api::DictionaryClient;
use crate::ui::AppWindow;
use adw::prelude::*;
use adw::Application;
use anyhow::Result;
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

        // Handle application activation - create window here (after startup signal)
        application.connect_activate(move |app| {
            let mut window_guard = window_ref.lock().unwrap();
            
            // Create window if it doesn't exist
            if window_guard.is_none() {
                // Create window
                let window = Arc::new(AppWindow::new(app));
                
                // Setup search entry handler
                let search_entry = window.search_entry.clone();
                let client_clone = client.clone();
                let window_ref_for_search = window_ref.clone();

                search_entry.connect_activate(move |entry| {
                    let word = entry.text().to_string();
                    if word.is_empty() {
                        return;
                    }

                    // Set loading state (switches to definition view)
                    {
                        let window_guard = window_ref_for_search.lock().unwrap();
                        if let Some(window) = window_guard.as_ref() {
                            window.set_loading(true);
                        }
                    }

                    let client = client_clone.clone();
                    let window_ref_clone = window_ref_for_search.clone();

                    // Perform synchronous lookup and update UI
                    // #region agent log
                    use std::fs::OpenOptions;
                    use std::io::Write;
                    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
                        let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"D\",\"location\":\"app.rs:73\",\"message\":\"About to call client.lookup\",\"data\":{{\"word\":\"{}\"}},\"timestamp\":{}}}", word, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
                    }
                    // #endregion
                    let result = match client.lookup(&word) {
                        Ok(entries) => {
                            // #region agent log
                            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
                                let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"D\",\"location\":\"app.rs:76\",\"message\":\"Lookup succeeded\",\"data\":{{\"word\":\"{}\",\"entries_count\":{}}},\"timestamp\":{}}}", word, entries.len(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
                            }
                            // #endregion
                            client.format_entry(&entries)
                        }
                        Err(e) => {
                            // #region agent log
                            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
                                let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"D\",\"location\":\"app.rs:82\",\"message\":\"Lookup failed\",\"data\":{{\"word\":\"{}\",\"error\":\"{}\"}},\"timestamp\":{}}}", word, format!("{:?}", e).replace("\"", "\\\"").replace("\n", "\\n"), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
                            }
                            // #endregion
                            format!("Error: {}", e)
                        }
                    };

                    let window_guard = window_ref_clone.lock().unwrap();
                    if let Some(window) = window_guard.as_ref() {
                        window.set_definition(&result);
                    }
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

        // Perform synchronous lookup
        let result = match self.client.lookup(word) {
            Ok(entries) => self.client.format_entry(&entries),
            Err(e) => format!("Error: {}", e),
        };

        window.definition_view.buffer().set_text(&result);

        Ok(String::from("Lookup completed"))
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

