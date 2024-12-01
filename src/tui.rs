use crate::communication::{DbMessage, TuiMessage};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;
use tokio::{select, sync::mpsc};

#[derive(Debug)]
pub struct App {
    input: String,
    output: String,
    history: Vec<String>,
    exit: bool,
    tui_rx: mpsc::Receiver<TuiMessage>,
    db_tx: mpsc::Sender<DbMessage>,
}

impl App {
    pub fn new(tui_rx: mpsc::Receiver<TuiMessage>, db_tx: mpsc::Sender<DbMessage>) -> Self {
        Self {
            input: String::new(),
            history: Vec::new(),
            exit: false,
            tui_rx,
            db_tx,
            output: String::new(),
        }
    }

    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    async fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).await
            }
            _ => {}
        };
        Ok(())
    }

    async fn handle_enter(&mut self) {
        let input = self.input.clone();
        self.input = String::default();

        if input == "quit" {
            self.exit();
            let _ = self.db_tx.send(DbMessage::Quit).await;
        } else {
            let _ = self.db_tx.send(DbMessage::Query(input.clone())).await;
            if let Some(response) = self.tui_rx.recv().await {
                match response {
                    TuiMessage::QueryResponse(value) => self.output = value,
                    TuiMessage::Failure(value) => self.output = value,
                }
            }
        }

        self.history.push(input);
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(char) => self.input.push(char),
            KeyCode::Backspace => {
                // Ignore the returned value, we don't care
                self.input.pop();
            }
            KeyCode::Enter => self.handle_enter().await,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Tables ".bold());

        let instructions = Line::from(vec!["type quit to quit ".into()]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut lines = self
            .history
            .clone()
            .into_iter()
            .map(|x| Line::from(vec!["> ".to_string().yellow(), x.to_string().yellow()]))
            .collect::<Vec<Line>>();

        lines.push(Line::from(vec![
            "> ".to_string().yellow(),
            self.input.to_string().yellow(),
        ]));

        if self.output.is_empty() == false {
            lines.push(Line::from(vec![
                "> ".to_string().yellow(),
                self.output.to_string().blue(),
            ]));
        }

        let shell_text = Text::from(lines);

        Paragraph::new(shell_text)
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {}
