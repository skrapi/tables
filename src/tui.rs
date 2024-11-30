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
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct App {
    input: String,
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
        }
    }

    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_enter(&mut self) {
        let input = self.input.clone();
        self.input = String::default();

        if input == "quit" {
            self.exit();
        }

        self.history.push(input);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(char) => self.input.push(char),
            KeyCode::Backspace => {
                // Ignore the returned value, we don't care
                self.input.pop();
            }
            KeyCode::Enter => self.handle_enter(),
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

        let shell_text = Text::from(lines);

        Paragraph::new(shell_text)
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
