use crate::app::App;

use tui::{
    backend::Backend,
    layout::Rect,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    let main = chunks[1];
    match app.tabs.index {
        0 => draw_patterns(f, app, main),
        1 => draw_samples(f, app, main),
        2 => draw_details(f, app, main),
        _ => {}
    };
}

fn draw_patterns<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let apps_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let (pattern, sample) = render_patterns(app);
    // split horizontal of right rect
    f.render_stateful_widget(pattern, apps_chunks[0], &mut app.pattern_table_state);
    f.render_widget(sample, apps_chunks[1]);
}

fn draw_samples<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);

    let (pattern, sample) = render_samples(app);
    // split horizontal of right rect
    f.render_widget(pattern, chunks[0]);
    f.render_stateful_widget(sample, chunks[1], &mut app.sample_table_state);
}

fn draw_details<B>(f: &mut Frame<B>, app: &mut App, _area: Rect)
where
    B: Backend,
{
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(size);

    let create_block = |title| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };
    // TODO: render pattern info for current sample
    // let paragraph = Paragraph::new(text.clone())
    //     .style(Style::default().bg(Color::White).fg(Color::Black))
    //     .block(create_block("Left, no wrap"))
    //     .alignment(Alignment::Left);
    // f.render_widget(paragraph, chunks[0]);
    let paragraph = Paragraph::new(app.current_rawlog.clone())
        .block(create_block("Log sample"))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    f.render_widget(paragraph, chunks[1]);
}

// fn render_details<'a>(app: &App) -> (Table<'a>, Table<'a>) {
//     let mut patterns = Vec::new();
//     for pattern in &app.patterns {
//         let row = Row::new(vec![
//             Cell::from(Span::raw(format!("{}", pattern.count))),
//             Cell::from(Span::raw(pattern.patterns.clone())),
//         ]);
//         patterns.push(row);
//     }
//
//     let selected_style = Style::default().add_modifier(Modifier::REVERSED);
//     let pattern_table = Table::new(patterns)
//         .header(Row::new(vec![
//             Cell::from(Span::styled(
//                 "Count",
//                 Style::default().add_modifier(Modifier::BOLD),
//             )),
//             Cell::from(Span::styled(
//                 "Pattern",
//                 Style::default().add_modifier(Modifier::BOLD),
//             )),
//         ]))
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .style(Style::default().fg(Color::White))
//                 .title("Pattern")
//                 .border_type(BorderType::Plain),
//         )
//         .highlight_style(selected_style)
//         .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)]);
//
//     let samples = app
//         .patterns
//         .get(
//             app.pattern_table_state
//                 .selected()
//                 .expect("always a selected pattern"),
//         )
//         .unwrap()
//         .samples
//         .clone();
//
//     let mut rows = Vec::new();
//     for sample in samples {
//         let row = Row::new(vec![
//             Cell::from(Span::raw(sample.date.to_string())),
//             Cell::from(Span::raw(sample.rawlog)),
//         ]);
//         rows.push(row);
//     }
//
//     let sample_detail = Table::new(rows)
//         .header(Row::new(vec![
//             Cell::from(Span::styled(
//                 "Date",
//                 Style::default().add_modifier(Modifier::BOLD),
//             )),
//             Cell::from(Span::styled(
//                 "log",
//                 Style::default().add_modifier(Modifier::BOLD),
//             )),
//         ]))
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .style(Style::default().fg(Color::White))
//                 .title("Samples")
//                 .border_type(BorderType::Plain),
//         )
//         .highlight_style(selected_style)
//         .widths(&[Constraint::Percentage(20), Constraint::Percentage(80)]);
//
//     (pattern_table, sample_detail)
// }

fn render_samples<'a>(app: &App) -> (Table<'a>, Table<'a>) {
    let mut patterns = Vec::new();
    let row = Row::new(vec![
        Cell::from(Span::raw(format!("{}", app.current_pattern().count))),
        Cell::from(Span::raw(app.current_pattern().patterns.clone())),
    ]);
    patterns.push(row);

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let pattern_table = Table::new(patterns)
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "Count",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Pattern",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Pattern")
                .border_type(BorderType::Plain),
        )
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)]);

    let samples = app
        .patterns
        .get(
            app.pattern_table_state
                .selected()
                .expect("always a selected pattern"),
        )
        .unwrap()
        .samples
        .clone();

    let mut rows = Vec::new();
    for sample in samples {
        let row = Row::new(vec![
            Cell::from(Span::raw(sample.date.to_string())),
            Cell::from(Span::raw(sample.rawlog)),
        ]);
        rows.push(row);
    }

    let sample_detail = Table::new(rows)
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "Date",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "log",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Samples")
                .border_type(BorderType::Plain),
        )
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(20), Constraint::Percentage(80)]);

    (pattern_table, sample_detail)
}

fn render_patterns<'a>(app: &App) -> (Table<'a>, Table<'a>) {
    let mut patterns = Vec::new();
    for pattern in &app.patterns {
        let row = Row::new(vec![
            Cell::from(Span::raw(format!("{}", pattern.count))),
            Cell::from(Span::raw(format!("{:.2}%", pattern.percent.unwrap_or(0.0)))),
            Cell::from(Span::raw(pattern.patterns.clone())),
        ]);
        patterns.push(row);
    }

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let pattern_table = Table::new(patterns)
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "Count",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Percent",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Pattern",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Patterns")
                .border_type(BorderType::Plain),
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Percentage(8),
            Constraint::Percentage(8),
            Constraint::Percentage(84),
        ]);

    let samples = app
        .patterns
        .get(
            app.pattern_table_state
                .selected()
                .expect("always a selected pattern"),
        )
        .unwrap()
        .samples
        .clone();

    let mut rows = Vec::new();
    for sample in samples {
        let row = Row::new(vec![
            Cell::from(Span::raw(sample.date.to_string())),
            Cell::from(Span::raw(sample.rawlog)),
        ]);
        rows.push(row);
    }

    let sample_detail = Table::new(rows)
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "Date",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "log",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Samples")
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Percentage(20), Constraint::Percentage(80)]);

    (pattern_table, sample_detail)
}
