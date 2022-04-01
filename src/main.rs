use clap::Parser;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::enable_raw_mode,
};
use log_pattern_viewer::{
    app::{App, Event, MenuItem},
    args::Args,
    error::Error,
    pattern::Pattern,
    ui::draw,
};
use std::{
    fs, io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let pattern_file = args.file;

    enable_raw_mode().expect("can run in raw mode");

    let patterns = read_report(&pattern_file).expect("can fetch report");
    let title = "Log Pattern Viewer";
    let mut app = App::new(title, patterns);

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|f| draw(f, &mut app))?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('p') => app.active_menu_item = MenuItem::Samples,
                KeyCode::Char('a') => {}
                KeyCode::Char('d') => {
                    match app.current_menu_item() {
                        MenuItem::Pattern => {}
                        MenuItem::Samples => {
                            // display json in third tab
                            let rawlog = app.current_sample_rawlog();
                            match serde_json::from_str::<serde_json::Value>(rawlog) {
                                Ok(json) => {
                                    let log = serde_json::to_string_pretty(&json).unwrap();
                                    app.current_rawlog = log;
                                }
                                Err(_) => {
                                    app.current_rawlog = rawlog.to_string();
                                }
                            }
                            app.on_right();
                        }
                        MenuItem::Details => {
                            app.scroll_down();
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => match app.current_menu_item() {
                    MenuItem::Pattern => {
                        if let Some(selected) = app.pattern_table_state.selected() {
                            let amount_patterns = app.patterns.len();
                            if selected >= amount_patterns - 1 {
                                app.pattern_table_state.select(Some(0));
                            } else {
                                app.pattern_table_state.select(Some(selected + 1));
                            }
                        }
                    }
                    MenuItem::Samples => {
                        let current_amount_samples = app.current_amount_samples();
                        if let Some(selected) = app.sample_table_state.selected() {
                            if selected >= current_amount_samples - 1 {
                                app.sample_table_state.select(Some(0));
                            } else {
                                app.sample_table_state.select(Some(selected + 1));
                            }
                        }
                    }
                    MenuItem::Details => {
                        app.scroll_down();
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => match app.current_menu_item() {
                    MenuItem::Pattern => {
                        if let Some(selected) = app.pattern_table_state.selected() {
                            let amount_patterns = app.patterns.len();
                            if selected > 0 {
                                app.pattern_table_state.select(Some(selected - 1));
                            } else {
                                app.pattern_table_state.select(Some(amount_patterns - 1));
                            }
                        }
                    }
                    MenuItem::Samples => {
                        let current_amount_samples = app.current_amount_samples();
                        if let Some(selected) = app.sample_table_state.selected() {
                            if selected > 0 {
                                app.sample_table_state.select(Some(selected - 1));
                            } else {
                                app.sample_table_state
                                    .select(Some(current_amount_samples - 1));
                            }
                        }
                    }
                    MenuItem::Details => {
                        app.scroll_up();
                    }
                },
                KeyCode::Left | KeyCode::Char('h') => app.on_left(),
                KeyCode::Right | KeyCode::Char('l') => app.on_right(),
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

// fn render_home<'a>() -> Paragraph<'a> {
//     let home = Paragraph::new(vec![
//         Spans::from(vec![Span::raw("")]),
//         Spans::from(vec![Span::raw("Welcome")]),
//         Spans::from(vec![Span::raw("")]),
//         Spans::from(vec![Span::raw("to")]),
//         Spans::from(vec![Span::raw("")]),
//         Spans::from(vec![Span::styled(
//             "Log Pattern Viewer",
//             Style::default().fg(Color::LightBlue),
//         )]),
//         Spans::from(vec![Span::raw("")]),
//         Spans::from(vec![Span::raw("Press 'p' to access log patterns")]),
//     ])
//     .alignment(Alignment::Center)
//     .block(
//         Block::default()
//             .borders(Borders::ALL)
//             .style(Style::default().fg(Color::White))
//             .title("Home")
//             .border_type(BorderType::Plain),
//     );
//     home
// }

fn read_report(path: &str) -> Result<Vec<Pattern>, Error> {
    let db_content = fs::read_to_string(path)?;
    let parsed: Vec<Pattern> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}
