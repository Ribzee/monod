use std::{thread::sleep, time::Duration};

use ratatui::{DefaultTerminal, Frame};

use crate::metrics::collector::SystemMonitor;

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut system = SystemMonitor::new();
    let rate = 0.5f32;

    loop {
        system.update_state(&rate);
        terminal.draw(|frame| frame.render_widget(format!("{:#?}", system), frame.area()))?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
        sleep(Duration::from_secs_f32(rate));
    }
}
