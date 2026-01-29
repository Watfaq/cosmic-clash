use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element, Length, Task};

fn main() -> iced::Result {
    iced::application(
        "Clash Iced - Clash Client",
        ClashApp::update,
        ClashApp::view,
    )
    .window_size((800.0, 600.0))
    .run_with(ClashApp::new)
}

#[derive(Default)]
struct ClashApp {
    proxy_url: String,
    port: String,
    status: String,
}

#[derive(Debug, Clone)]
enum Message {
    ProxyUrlChanged(String),
    PortChanged(String),
    StartProxy,
    StopProxy,
}

impl ClashApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                proxy_url: String::new(),
                port: String::from("7890"),
                status: String::from("Stopped"),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ProxyUrlChanged(value) => {
                self.proxy_url = value;
            }
            Message::PortChanged(value) => {
                self.port = value;
            }
            Message::StartProxy => {
                self.status = String::from("Running");
            }
            Message::StopProxy => {
                self.status = String::from("Stopped");
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Clash Iced").size(32);

        let status_text = text(format!("Status: {}", self.status)).size(20);

        let proxy_input = text_input("Enter proxy URL...", &self.proxy_url)
            .on_input(Message::ProxyUrlChanged)
            .padding(10);

        let port_input = text_input("Port", &self.port)
            .on_input(Message::PortChanged)
            .padding(10);

        let start_button = button(text("Start Proxy").size(16))
            .on_press(Message::StartProxy)
            .padding(10);

        let stop_button = button(text("Stop Proxy").size(16))
            .on_press(Message::StopProxy)
            .padding(10);

        let controls = row![start_button, stop_button].spacing(10);

        let content = column![
            title,
            status_text,
            text("Proxy URL:").size(14),
            proxy_input,
            text("Port:").size(14),
            port_input,
            controls,
        ]
        .spacing(20)
        .padding(20)
        .align_x(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
