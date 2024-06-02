use iced::{Command, Element, Application};
use iced::widget::{
    column,
    row,
    button,
    container,
    text_editor,
};

use std::sync::Arc;
use std::io;
use std::path::Path;

fn main() -> iced::Result {
    App::run(iced::Settings::default())
}

struct App {
    content: text_editor::Content,
    rt: Arc<tokio::runtime::Runtime>,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Open,
    FileOpen(Result<Arc<String>, Error>),
}

impl Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();
    type Executor = iced::executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let path = format!("{}/files/Archivo_1.txt", env!("CARGO_MANIFEST_DIR"));
        // let rt = tokio::runtime::Runtime::new().unwrap();
        let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
        (
            Self {
                content: text_editor::Content::new(),
                rt: rt.clone(),
            },
            Command::perform(
                load_file(path, rt.clone()),
                Message::FileOpen
            )
        )
    }

    fn title(&self) -> String {
        String::from("Automata")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Command::none()
            }
            Message::FileOpen(result) => {
                if let Ok(content) = result {
                    self.content = text_editor::Content::with_text(&content);
                }
                Command::none()
            }
            Message::Open => {
                Command::perform(pick_file(self.rt.clone()), Message::FileOpen)
                // Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let editor = text_editor(&self.content).on_action(Message::Edit).height(iced::Length::Fill);
        let controls = row![
            button("Open").on_press(Message::Open),
        ];
        let content = column![
            controls,
            editor
        ];
        container(content).padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Nord
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
enum Error {
    DialogClosed,
    Io(io::ErrorKind),
}

async fn load_file(path: impl AsRef<Path>, rt: Arc<tokio::runtime::Runtime>)
    -> Result<Arc<String>, Error>
{
    let path = path.as_ref().to_path_buf();
    let content = rt.spawn_blocking(move || std::fs::read_to_string(path))
        .await
        .map_err(|_| Error::Io(io::ErrorKind::Other))?
        .map(Arc::new)
        .map_err(|_| Error::Io(io::ErrorKind::Other))?;
    Ok(content)
}

async fn pick_file(rt: Arc<tokio::runtime::Runtime>) -> Result<Arc<String>, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path(), rt).await
}
