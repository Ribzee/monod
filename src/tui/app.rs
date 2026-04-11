use std::{io, thread::sleep, time::Duration};

use bytesize::ByteSize;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{Marker, border, merge::MergeStrategy},
    text::{Line, Span, Text},
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

        let [mem, net] = left.layout(&Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]));

        self.render_memory_block(mem, buf);
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

    fn render_memory_block(&self, area: Rect, buf: &mut Buffer) {
        let [mem_blocks, disks] = area.layout(
            &Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .spacing(-1),
        );

        self.render_memory(mem_blocks, buf);
        self.render_io(disks, buf);

        Block::bordered()
            .border_set(border::ROUNDED)
            .title("mem")
            .merge_borders(MergeStrategy::Exact)
            .style(Style::default().green())
            .render(area, buf);
    }

    fn render_io(&self, area: Rect, buf: &mut Buffer) {
        let disks_blocks = area.layout_vec(
            &Layout::vertical(vec![
                Constraint::Fill(1);
                self.system_monitor.system_state.disk_io.len()
            ])
            .spacing(-1),
        );

        let mut disk_iterator = self.system_monitor.system_state.disk_io.keys();

        for block in disks_blocks.iter() {
            let [read, write] = block.layout(
                &Layout::vertical(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .spacing(0),
            );

            let max_points = read.width as f64;
            let key = disk_iterator.next().unwrap();

            let read_data = self
                .system_monitor
                .system_state_series
                .iter()
                .rev()
                .take(max_points as usize)
                .enumerate()
                .map(|(i, state)| {
                    (
                        max_points - i as f64,
                        state.disk_io.get(key).unwrap_or(&(0u64, 0u64)).0 as f64,
                    )
                })
                .collect::<Vec<(f64, f64)>>();

            let write_data = self
                .system_monitor
                .system_state_series
                .iter()
                .rev()
                .take(max_points as usize)
                .enumerate()
                .map(|(i, state)| {
                    (
                        max_points - i as f64,
                        state.disk_io.get(key).unwrap_or(&(0u64, 0u64)).1 as f64,
                    )
                })
                .collect::<Vec<(f64, f64)>>();

            let read_dataset = Dataset::default()
                .name("")
                .marker(Marker::Braille)
                .graph_type(GraphType::Bar)
                .style(Style::default().fg(Color::Cyan))
                .data(&read_data);

            let write_dataset = Dataset::default()
                .name("")
                .marker(Marker::Braille)
                .graph_type(GraphType::Bar)
                .style(Style::default().fg(Color::Cyan))
                .data(&write_data);

            Chart::new(vec![read_dataset])
                .y_axis(Axis::default().bounds([0.0, 100.0]).title("R"))
                .x_axis(Axis::default().bounds([0.0, max_points]))
                .render(read, buf);

            Chart::new(vec![write_dataset])
                .y_axis(Axis::default().bounds([0.0, 100.0]).title("W"))
                .x_axis(Axis::default().bounds([0.0, max_points]))
                .style(Style::default().red())
                .render(write, buf);

            Block::bordered()
                .title(key.to_string())
                .border_set(border::PLAIN)
                .merge_borders(MergeStrategy::Exact)
                .style(Style::default().gray())
                .render(block.to_owned(), buf);
        }
    }

    fn render_memory(&self, area: Rect, buf: &mut Buffer) {
        let [
            mem_total,
            mem_used,
            mem_available,
            swap_total,
            swap_used,
            swap_free,
        ] = area.layout(
            &Layout::vertical([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .spacing(-1),
        );

        Paragraph::new(Line::from(vec![
            Span::raw("Total: "),
            Span::raw(
                ByteSize::b(self.system_monitor.system_info.total_mem)
                    .display()
                    .iec_short()
                    .to_string(),
            ),
        ]))
        .style(Style::default().bold())
        .block(
            Block::bordered()
                .border_set(border::PLAIN)
                .title_alignment(ratatui::layout::HorizontalAlignment::Left)
                .merge_borders(MergeStrategy::Exact)
                .style(Style::default().gray()),
        )
        .render(mem_total, buf);

        let max_points = (mem_used.width * 2) as f64;

        let mem_used_data = self
            .system_monitor
            .system_state_series
            .iter()
            .rev()
            .take(max_points as usize)
            .enumerate()
            .map(|(i, state)| (max_points - i as f64, state.mem_percentage as f64))
            .collect::<Vec<(f64, f64)>>();

        let dataset = Dataset::default()
            .name("")
            .marker(Marker::Braille)
            .graph_type(GraphType::Bar)
            .style(Style::default().fg(Color::Cyan))
            .data(&mem_used_data);

        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([0.0, 100.0]))
            .x_axis(Axis::default().bounds([0.0, max_points]))
            .block(
                Block::bordered()
                    .title(format!(
                        "Used: {}",
                        ByteSize::b(self.system_monitor.system_state.mem_usage)
                            .display()
                            .iec_short()
                    ))
                    .border_set(border::PLAIN)
                    .merge_borders(MergeStrategy::Exact)
                    .style(Style::default().gray()),
            )
            .render(mem_used, buf);

        let mem_available_data = self
            .system_monitor
            .system_state_series
            .iter()
            .rev()
            .take(max_points as usize)
            .enumerate()
            .map(|(i, state)| {
                (
                    max_points - i as f64,
                    self.system_monitor.system_info.total_mem as f64 - state.mem_percentage as f64,
                )
            })
            .collect::<Vec<(f64, f64)>>();

        let dataset = Dataset::default()
            .name("")
            .marker(Marker::Braille)
            .graph_type(GraphType::Bar)
            .style(Style::default().fg(Color::Cyan))
            .data(&mem_available_data);

        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([0.0, 100.0]))
            .x_axis(Axis::default().bounds([0.0, max_points]))
            .block(
                Block::bordered()
                    .title(format!(
                        "Available: {}",
                        ByteSize::b(
                            self.system_monitor.system_info.total_mem
                                - self.system_monitor.system_state.mem_usage
                        )
                        .display()
                        .iec_short()
                    ))
                    .border_set(border::PLAIN)
                    .merge_borders(MergeStrategy::Exact)
                    .style(Style::default().gray()),
            )
            .render(mem_available, buf);

        Paragraph::new(Line::from(vec![
            Span::raw("Swap: "),
            Span::raw(
                ByteSize::b(self.system_monitor.system_info.total_swap)
                    .display()
                    .iec_short()
                    .to_string(),
            ),
        ]))
        .style(Style::default().bold())
        .block(
            Block::bordered()
                .border_set(border::PLAIN)
                .title_alignment(ratatui::layout::HorizontalAlignment::Left)
                .merge_borders(MergeStrategy::Exact)
                .style(Style::default().gray()),
        )
        .render(swap_total, buf);

        let swap_used_data = self
            .system_monitor
            .system_state_series
            .iter()
            .rev()
            .take(max_points as usize)
            .enumerate()
            .map(|(i, state)| (max_points - i as f64, state.swap_percentage as f64))
            .collect::<Vec<(f64, f64)>>();

        let dataset = Dataset::default()
            .name("")
            .marker(Marker::Braille)
            .graph_type(GraphType::Bar)
            .style(Style::default().fg(Color::Cyan))
            .data(&swap_used_data);

        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([0.0, 100.0]))
            .x_axis(Axis::default().bounds([0.0, max_points]))
            .block(
                Block::bordered()
                    .title(format!(
                        "Used: {}",
                        ByteSize::b(self.system_monitor.system_state.swap_usage)
                            .display()
                            .iec_short()
                    ))
                    .border_set(border::PLAIN)
                    .merge_borders(MergeStrategy::Exact)
                    .style(Style::default().gray()),
            )
            .render(swap_used, buf);

        let swap_available_data = self
            .system_monitor
            .system_state_series
            .iter()
            .rev()
            .take(max_points as usize)
            .enumerate()
            .map(|(i, state)| {
                (
                    max_points - i as f64,
                    self.system_monitor.system_info.total_swap as f64
                        - state.swap_percentage as f64,
                )
            })
            .collect::<Vec<(f64, f64)>>();

        let dataset = Dataset::default()
            .name("")
            .marker(Marker::Braille)
            .graph_type(GraphType::Bar)
            .style(Style::default().fg(Color::Cyan))
            .data(&swap_available_data);

        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([0.0, 100.0]))
            .x_axis(Axis::default().bounds([0.0, max_points]))
            .block(
                Block::bordered()
                    .title(format!(
                        "Available: {}",
                        ByteSize::b(
                            self.system_monitor.system_info.total_swap
                                - self.system_monitor.system_state.swap_usage
                        )
                        .display()
                        .iec_short()
                    ))
                    .border_set(border::PLAIN)
                    .merge_borders(MergeStrategy::Exact)
                    .style(Style::default().gray()),
            )
            .render(swap_free, buf);
    }
}
