use monod::tui::app::App;

fn main() -> Result<(), anyhow::Error> {
    let series = ratatui::run(|terminal| App::default().run(terminal))?;

    println!(
        "{:#?}",
        series
            .iter()
            .map(|entry| entry.cpu_usage)
            .collect::<Vec<f32>>()
    );
    Ok(())
}
