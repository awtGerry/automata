mod automata;

use iced::{Command, Element, Application};
use iced::widget::{
    column,
    row,
    text,
    button,
    container,
    text_editor,
};

use std::sync::Arc;
use std::io;
use std::path::{PathBuf, Path};

use automata::Automata;

fn main() -> iced::Result {
    App::run(iced::Settings::default())
}

struct App {
    path: Option<PathBuf>,
    content: text_editor::Content,
    result: String,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Open,
    Save,
    Automata,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    FileSaved(Result<(), Error>),
}

impl Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();
    type Executor = iced::executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                path: None,
                content: text_editor::Content::new(),
                result: String::new(),
                error: None,
            },
            Command::perform(
                load_file(default_file()),
                Message::FileOpened
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
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
            Message::Save => {
                let contents = self.content.text();
                Command::perform(save_file(self.path.clone(), contents), Message::FileSaved)
            }
            /* Call the automata function to perform all the operations */
            Message::Automata => {
                let content = self.content.text();
                let automata = Automata::new(content);
                self.result = automata.analyze();
                Command::none()
            }
            Message::FileSaved(Ok(())) => Command::none(),
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::FileOpened(Ok((path, content))) => {
                self.path = Some(path);
                self.content = text_editor::Content::with_text(&content);
                Command::none()
            }
            Message::FileOpened(Err(_)) => {
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let controls = row![
            button("Open").on_press(Message::Open),
            button("Save").on_press(Message::Save),
            button("Automata").on_press(Message::Automata),
        ].spacing(15);

        let editor = text_editor(&self.content).on_action(Message::Edit).height(iced::Length::Fill);

        let file_path = match self.path.as_deref().and_then(Path::to_str) {
            Some(path) => text(path).size(14),
            None => text(""),
        };

        let content = column![
            controls,
            editor,
            text(&self.result).size(10),
            file_path,
        ].spacing(10);

        container(content).padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::GruvboxLight
    }
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/files/Archivo_2.txt", env!("CARGO_MANIFEST_DIR")))
}

#[derive(Debug, Clone)]
#[allow(unused)]
enum Error {
    DialogClosed,
    IOFailed(io::ErrorKind),
}

async fn load_file(path: PathBuf)
    -> Result<(PathBuf, Arc<String>), Error>
{
    let content = Arc::new(std::fs::read_to_string(&path).map_err(|_| Error::IOFailed(io::ErrorKind::Other))?);

    Ok((path, content))
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn save_file(path: Option<PathBuf>, text: String)
    -> Result<(), Error>
{
    let path = match path {
        Some(path) => path,
        None => {
            let handle = rfd::AsyncFileDialog::new()
                .set_title("Save a text file...")
                .pick_folder()
                .await
                .ok_or(Error::DialogClosed)?;

            handle.path().to_owned()
        }
    };

    std::fs::write(&path, text).map_err(|error| Error::IOFailed(error.kind()))?;
    Ok(())
}
