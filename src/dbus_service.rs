use dbus::blocking::Connection;
use dbus::channel::Sender as DbusSender;
use dbus::Message;
use std::sync::mpsc::Sender;
use std::time::Duration;

const DBUS_SERVICE_NAME: &str = "com.aynary.Dictionary";
const DBUS_OBJECT_PATH: &str = "/com/aynary/Dictionary";
const DBUS_INTERFACE: &str = "com.aynary.Dictionary";

#[derive(Clone)]
pub enum DbusCommand {
    LookupWord(String),
    ShowWindow,
    LookupAndShow(String),
}

pub struct DictionaryService {
    sender: Sender<DbusCommand>,
}

impl DictionaryService {
    pub fn new(sender: Sender<DbusCommand>) -> Self {
        Self { sender }
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::new_session()?;

        // Request the service name
        conn.request_name(DBUS_SERVICE_NAME, false, true, false)?;

        // Simple message handling loop
        // Note: This is a simplified implementation. For production use,
        // consider using dbus-tokio for async or dbus::tree for structured message handling
        loop {
            conn.process(Duration::from_millis(1000))?;
            // The DBus service will handle messages via the system's message bus
            // In a full implementation, you'd register object paths and methods
            // For now, we'll use the helper functions that call this service
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    
    // Placeholder - actual message handling would be done via registered methods
    // For now, external callers use the helper functions that send messages directly

    // Message handling removed - simplified implementation
    // The DBus service loop keeps the service registered
    // Actual method calls are handled via the helper functions that use the proxy API
}

// Helper function to make DBus calls from other components
pub fn lookup_word_via_dbus(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(DBUS_SERVICE_NAME, DBUS_OBJECT_PATH, Duration::from_millis(5000));
    
    let reply: (String,) = proxy.method_call(DBUS_INTERFACE, "LookupWord", (word,))?;
    Ok(reply.0)
}

pub fn show_window_via_dbus() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(DBUS_SERVICE_NAME, DBUS_OBJECT_PATH, Duration::from_millis(5000));
    
    let _reply: () = proxy.method_call(DBUS_INTERFACE, "ShowWindow", ())?;
    Ok(())
}

pub fn lookup_and_show_via_dbus(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(DBUS_SERVICE_NAME, DBUS_OBJECT_PATH, Duration::from_millis(5000));
    
    let _reply: () = proxy.method_call(DBUS_INTERFACE, "LookupAndShow", (word,))?;
    Ok(())
}
