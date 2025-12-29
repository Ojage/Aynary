// Native messaging host for browser extension
// This binary communicates with the browser extension via stdin/stdout

use dbus::blocking::Connection;
use serde_json;
use std::io::{self, BufRead, Write};
use std::time::Duration;

const DBUS_SERVICE_NAME: &str = "com.aynary.Dictionary";
const DBUS_OBJECT_PATH: &str = "/com/aynary/Dictionary";
const DBUS_INTERFACE: &str = "com.aynary.Dictionary";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let input = line?;
        
        // Parse JSON message from extension
        let json: serde_json::Value = serde_json::from_str(&input)?;
        
        if let Some(action) = json.get("action").and_then(|v| v.as_str()) {
            match action {
                "lookup" => {
                    if let Some(word) = json.get("word").and_then(|v| v.as_str()) {
                        // Call DBus service
                        let conn = Connection::new_session()?;
                        let proxy = conn.with_proxy(
                            DBUS_SERVICE_NAME,
                            DBUS_OBJECT_PATH,
                            Duration::from_millis(5000),
                        );

                        let _reply: () = proxy.method_call(DBUS_INTERFACE, "LookupAndShow", (word,))?;

                        // Send response to extension
                        let response = serde_json::json!({
                            "success": true,
                            "word": word
                        });
                        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
                        stdout.flush()?;
                    }
                }
                _ => {
                    let response = serde_json::json!({
                        "success": false,
                        "error": "Unknown action"
                    });
                    writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
                    stdout.flush()?;
                }
            }
        }
    }

    Ok(())
}

