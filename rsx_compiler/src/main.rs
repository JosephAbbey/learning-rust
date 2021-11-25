mod lib;
use lib::rsx;

fn main() {
    println!(
        "{}",
        rsx("
use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from(\"Counter - Iced\")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        <Column padding=\"{20}\">
            <Button on_press=\"{Message::IncrementPressed}\">Increment</Button>
            <Text size=\"{50}\">{self.value.to_string()}</Text>
            <Button on_press=\"{Message::DecrementPressed}\">Decrement</Button>
        </Column>
    }
}
"
        .to_string())
    );
}
