use iced::{Element, Task};
use iced::widget::{text, column, button};

mod ipc;
mod dashboard;
mod dialog;
mod database;
mod sysbar;

// Include protocol in UI crate
pub mod protocol {
    tonic::include_proto!("protocol");
}

pub fn main() -> iced::Result {
    // Initialize system tray gadget
    let _tray_icon = match sysbar::create_sysbar() {
        Ok(tray) => Some(tray),
        Err(e) => {
            eprintln!("Failed to initialize system tray: {}", e);
            None
        }
    };

    iced::run(OpenSnitchUI::update, OpenSnitchUI::view)
}

#[derive(Default)]
struct OpenSnitchUI {
    connected: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ConnectIpc,
    IpcConnected(bool),
    AllowConnection,
    DenyConnection,
}

impl OpenSnitchUI {
    fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            Message::ConnectIpc => {
                println!("Connecting to SecureSnitch IPC...");
                return Task::perform(
                    async {
                        match ipc::create_client("http://127.0.0.1:50051").await {
                            Ok(_) => true,
                            Err(e) => {
                                eprintln!("Connection failed: {}", e);
                                false
                            }
                        }
                    },
                    Message::IpcConnected
                );
            }
            Message::IpcConnected(success) => {
                state.connected = success;
                if success {
                    println!("SecureSnitch IPC Connected!");
                }
            }
            Message::AllowConnection => println!("Connection Allowed!"),
            Message::DenyConnection => println!("Connection Denied!"),
        }
        Task::none()
    }

    fn view(state: &Self) -> Element<'_, Message> {
        let status_text = if state.connected {
            text("Status: Connected").color([0.0, 0.8, 0.0])
        } else {
            text("Status: Disconnected").color([0.8, 0.0, 0.0])
        };

        column![
            text("SecureSnitch Dashboard").size(40),
            status_text,
            button("Connect Daemon IPC").on_press(Message::ConnectIpc),
            button("Simulate: Allow Connection").on_press(Message::AllowConnection),
            button("Simulate: Deny Connection").on_press(Message::DenyConnection),
        ]
        .padding(20)
        .spacing(10)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test_ui_init() {
        let _ui = OpenSnitchUI::default();
        assert!(true);
    }
}