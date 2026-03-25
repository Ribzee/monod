use std::{io, thread::sleep, time::Duration};

use bytesize::ByteSize;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{Marker, border},
    text::{Line, Text},
    widgets::{Axis, Block, Cell, Chart, Dataset, GraphType, Paragraph, Row, Table, Widget},
};

use crate::metrics::collector::{SystemMonitor, SystemState};

#[derive(Debug)]
pub struct App {
    pub system_monitor: SystemMonitor,
    pub rate: f32,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            system_monitor: SystemMonitor::default(),
            rate: 0.5,
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<Vec<SystemState>> {
        self.system_monitor.update_state(&self.rate);
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            //self.handle_event()?;
            self.system_monitor.update_state(&self.rate);
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        self.exit = true;
                    }
                }
            }
            sleep(Duration::from_secs_f32(self.rate));
        }

        Ok(self.system_monitor.system_state_series.clone())
    }

    fn draw(&self, frame: &mut Frame) {
        self.render(frame.area(), frame.buffer_mut());
    }
}

impl App {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let [top, bottom] = area.layout(&Layout::vertical([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(2, 3),
        ]));

        self.render_cpu(top, buf);

        let [left, right] = bottom.layout(&Layout::horizontal([
            Constraint::Percentage(45),
            Constraint::Percentage(55),
        ]));

        self.render_processes(right, buf);
    }

    fn render_cpu(&self, area: Rect, buf: &mut Buffer) {
        let max_points = (area.width * 2) as f64;

        let block = Block::bordered().border_set(border::ROUNDED);

        let data = self
            .system_monitor
            .system_state_series
            .iter()
            .rev()
            .take(max_points as usize)
            .enumerate()
            .map(|(i, state)| (max_points - i as f64, state.cpu_usage as f64))
            .collect::<Vec<(f64, f64)>>();

        let dataset = Dataset::default()
            .name("")
            .marker(Marker::Braille)
            .graph_type(GraphType::Bar)
            .style(Style::default().fg(Color::Cyan))
            .data(&data);

        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([0.0, 100.0]))
            .x_axis(Axis::default().bounds([0.0, max_points]))
            .block(block)
            .render(area, buf);
    }

    fn render_processes(&self, area: Rect, buf: &mut Buffer) {
        let header = Row::new(["Pid:", "Program:", "Command:", "User:", "MemB", "Cpu%"])
            .style(Style::default().bold());

        let mut processes = self.system_monitor.system_state.processes.clone();
        processes.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap());
        processes.reverse();

        let rows = processes.iter().map(|process| {
            let process = process.clone();
            let cpu_usage_temp: Vec<&str> = process.cpu_usage.to_string().split('.').collect();
            Row::new([
                Cell::from(process.pid.to_string()),
                Cell::from(process.name).style(Style::default().cyan()),
                Cell::from(process.command),
                Cell::from(process.user),
                Cell::from(
                    ByteSize::b(process.memory_usage)
                        .display()
                        .iec_short()
                        .to_string(),
                )
                .style(Style::default().cyan()),
                Cell::from(format!("{:.1}", process.cpu_usage)).style(Style::default().cyan()),
            ])
        });

        let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ];

        Table::new(rows, widths)
            .header(header)
            .column_spacing(1)
            .row_highlight_style(Modifier::REVERSED)
            .block(Block::bordered().border_set(border::ROUNDED))
            .render(area, buf);
    }
}
