use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    test_counter: i64,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
                self.test_counter = self.test_counter + 1;
            },
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let message = format!("Current tick: {}", self.test_counter);
        let span = Span::styled(message, Style::new().dim());
        let line = Line::from(vec![span]);
        let paragraph = Paragraph::new(line);

        frame.render_widget(paragraph, area);
        Ok(())
    }
}