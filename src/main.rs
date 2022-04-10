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
    s3::{read_report_file_from_key, report_file_key},
    ui::draw,
};
use rusoto_core::request::HttpClient;
use rusoto_credential::ProfileProvider;
use rusoto_s3::S3Client;
use std::{
    fs, io,
    sync::mpsc,
    sync::mpsc::channel,
    thread,
    time::{Duration, Instant},
};
use tokio::runtime::Runtime;
use tui::{backend::CrosstermBackend, Terminal};

fn read_from_remote(args: &Args) -> anyhow::Result<Vec<Pattern>> {
    let (tx, rx) = channel();

    let profile = if args.profile.is_some() {
        println!("Using profile: {}", args.profile.as_ref().unwrap());
        ProfileProvider::with_default_credentials(args.profile.as_ref().unwrap())?
    } else {
        ProfileProvider::new()?
    };
    let region = args
        .region
        .as_ref()
        .unwrap_or(&"cn-northwest-1".to_string())
        .parse()?;
    let s3 = S3Client::new_with(HttpClient::new()?, profile, region);

    let rt = Runtime::new().unwrap();
    let namespace = &args.namespace;
    let app = &args.name;
    let year = args.year;
    let month = args.month;

    if namespace.is_none() || app.is_none() || year.is_none() || month.is_none() {
        return Err(anyhow::anyhow!("namespace, app, year, month must be set"));
    }
    let key = report_file_key(
        namespace.as_ref().unwrap(),
        app.as_ref().unwrap(),
        year.unwrap(),
        month.unwrap(),
    );
    rt.block_on(async {
        // retrieve report file
        match read_report_file_from_key(&s3, &key).await {
            Ok(report) => {
                tx.send(report).unwrap();
            }
            Err(_) => {
                // send error to main thread
                // TODO: handle s3 read error
            }
        }
    });
    println!("Receiving report {key} ...");
    let reports = rx.recv().expect("Bad report format");
    let mut patterns = read_report_from_str(&reports).expect("can fetch report");
    patterns.sort_by(|a, b| b.count.cmp(&a.count));

    Ok(patterns)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let local_file = &args.from_local;

    enable_raw_mode().expect("can run in raw mode");

    let patterns = if local_file.is_none() {
        read_from_remote(&args)?
    } else {
        read_report_from_file(local_file.as_ref().unwrap())?
    };

    let title = "Log Pattern Viewer";
    let mut app = App::new(title, patterns);
    app.calculate_percent();

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
                        app.handle_down_patterns();
                    }
                    MenuItem::Samples => {
                        app.handle_down_samples();
                    }
                    MenuItem::Details => {
                        app.scroll_down();
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => match app.current_menu_item() {
                    MenuItem::Pattern => {
                        app.handle_up_patterns();
                    }
                    MenuItem::Samples => {
                        app.handle_up_samples();
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

fn read_report_from_file(path: &str) -> Result<Vec<Pattern>, Error> {
    let db_content = fs::read_to_string(path)?;
    let mut patterns = read_report_from_str(&db_content)?;
    patterns.sort_by(|a, b| b.count.cmp(&a.count));
    Ok(patterns)
}

fn read_report_from_str(content: &str) -> Result<Vec<Pattern>, Error> {
    let parsed: Vec<Pattern> = serde_json::from_str(content)?;
    Ok(parsed)
}
