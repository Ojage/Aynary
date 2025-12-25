use dbus::blocking::Connection;
use dbus::Message;
use glib::Sender;
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

        // Start handling messages
        loop {
            conn.process(Duration::from_millis(1000))?;

            // Check for method calls
            let msg = conn.incoming(Duration::from_millis(100));
            if let Some(msg) = msg {
                if msg.msg_type() == dbus::MessageType::MethodCall {
                    if let Some(path) = msg.path() {
                        if path == DBUS_OBJECT_PATH {
                            if let Some(interface) = msg.interface() {
                                if interface == DBUS_INTERFACE {
                                    self.handle_message(&conn, &msg)?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_message(&self, conn: &Connection, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(member) = msg.member() {
            match member.as_str() {
                "LookupWord" => {
                    let word: String = msg.read1()?;
                    let _ = self.sender.send(DbusCommand::LookupWord(word));
                    let reply = msg.method_return().append1("Lookup initiated".to_string());
                    conn.send(reply)?;
                }
                "ShowWindow" => {
                    let _ = self.sender.send(DbusCommand::ShowWindow);
                    let reply = msg.method_return();
                    conn.send(reply)?;
                }
                "LookupAndShow" => {
                    let word: String = msg.read1()?;
                    let _ = self.sender.send(DbusCommand::LookupAndShow(word));
                    let reply = msg.method_return();
                    conn.send(reply)?;
                }
                _ => {
                    let reply = msg.error(
                        "org.freedesktop.DBus.Error.UnknownMethod",
                        &format!("Unknown method: {}", member),
                    );
                    conn.send(reply)?;
                }
            }
        }
        Ok(())
    }
}

// Helper function to make DBus calls from other components
pub fn lookup_word_via_dbus(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(DBUS_SERVICE_NAME, DBUS_OBJECT_PATH, Duration::from_millis(5000));
    
    let msg = Message::method_call(
        DBUS_SERVICE_NAME,
        DBUS_OBJECT_PATH,
        DBUS_INTERFACE,
        "LookupWord",
    )?
    .append1(word);

    let reply = proxy.method_call(msg)?;
    let (result,): (String,) = reply.read1()?;
    Ok(result)
}

pub fn show_window_via_dbus() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(DBUS_SERVICE_NAME, DBUS_OBJECT_PATH, Duration::from_millis(5000));
    
    let msg = Message::method_call(
        DBUS_SERVICE_NAME,
        DBUS_OBJECT_PATH,
        DBUS_INTERFACE,
        "ShowWindow",
    )?;

    let _reply = proxy.method_call(msg)?;
    Ok(())
}

pub fn lookup_and_show_via_dbus(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(DBUS_SERVICE_NAME, DBUS_OBJECT_PATH, Duration::from_millis(5000));
    
    let msg = Message::method_call(
        DBUS_SERVICE_NAME,
        DBUS_OBJECT_PATH,
        DBUS_INTERFACE,
        "LookupAndShow",
    )?
    .append1(word);

    let _reply = proxy.method_call(msg)?;
    Ok(())
}
