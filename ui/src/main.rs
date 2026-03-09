use iced::{Element, Task};
use iced::widget::{text, column, button};

mod ipc;
mod dashboard;
mod dialog;
mod database;
mod sysbar;

pub fn main() -> iced::Result {
    // Initialize system tray gadget
    let _tray_icon = match sysbar::create_sysbar() {
        Ok(tray) => Some(tray),
        Err(e) => {
            eprintln!("Failed to initialize system tray: {}", e);
            None
        }
    };

    iced::application(
        SecureSnitchUI::new,
        SecureSnitchUI::update,
        SecureSnitchUI::view,
    )
    .title(SecureSnitchUI::title)
    .run()
}

#[derive(Default)]
struct SecureSnitchUI {}

#[derive(Debug, Clone)]
enum Message {
    ConnectIpc,
    AllowConnection,
    DenyConnection,
}

impl SecureSnitchUI {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn title(&self) -> String {
        String::from("SecureSnitch - Rust & Iced")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ConnectIpc => println!("Connecting to IPC..."),
            Message::AllowConnection => println!("Connection Allowed!"),
            Message::DenyConnection => println!("Connection Denied!"),
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            text("SecureSnitch Dashboard").size(40),
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

    // Smoke Test
    #[test]
    fn smoke_test_ui_init() {
        let _ui = SecureSnitchUI::default();
        // Just verifying initialization works without panic
        assert!(true);
    }
}