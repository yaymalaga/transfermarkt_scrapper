use std::io;
use tui::{Terminal, layout::Corner, widgets::ListState};
use tui::{
    backend::CrosstermBackend, layout::Constraint, layout::Direction, layout::Layout, style::Color,
    style::Style, text::Span, text::Spans, widgets::Block, widgets::Borders, widgets::Gauge,
    widgets::List, widgets::ListItem,
};

pub struct TerminalHelper;

impl TerminalHelper {
    pub fn new() {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Error while instanciating terminal");
        terminal.clear();
        terminal.hide_cursor();
        terminal.draw(|f| {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());

            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(vertical_chunks[0]);

            let block = Block::default()
                .title(" SCRAPPING DETAILS ")
                .borders(Borders::ALL);
            f.render_widget(block, vertical_chunks[0]);

            let items = [
                ListItem::new("Item 1").style(Style::default().bg(Color::Green)),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
                ListItem::new("Item 3"),
            ];

            let leagues_list = List::new(items.clone())
                .block(Block::default().title(" LEAGUES ").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
            f.render_widget(leagues_list, horizontal_chunks[0]);

            let teams_list = List::new(items.clone())
                .block(Block::default().title(" TEAMS ").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
            f.render_widget(teams_list, horizontal_chunks[1]);
            
            let mut state = ListState::default();
            state.select(Some(10));
            let players_list = List::new(items)
                .block(Block::default().title(" PLAYERS ").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .bg(Color::White)
                        .fg(Color::Black)
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(players_list, horizontal_chunks[2], &mut state);

            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .title("SCRAPPING PERCENTAGE")
                        .borders(Borders::ALL),
                )
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(50);
            f.render_widget(gauge, vertical_chunks[1]);
        });
    }
}
