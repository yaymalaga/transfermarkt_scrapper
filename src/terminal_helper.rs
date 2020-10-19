use std::{cell::RefCell, cmp::min, io::{self, Stdout}};
use tui::{
    backend::CrosstermBackend, layout::Constraint, layout::Direction, layout::Layout, style::Color,
    style::Style, text::Span, text::Spans, widgets::Block, widgets::Borders, widgets::Gauge,
    widgets::List, widgets::ListItem,
};
use tui::{layout::Corner, widgets::ListState, Terminal};

pub struct TerminalHelper<'a> {
    terminal: RefCell<Terminal<CrosstermBackend<Stdout>>>,
    percentage: u16,
    leagues_list: Vec<ListItem<'a>>,
    teams_list: Vec<ListItem<'a>>,
    players_list: Vec<ListItem<'a>>,
}

impl<'a> TerminalHelper<'a> {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Error while instanciating terminal");
        terminal.clear();
        terminal.hide_cursor();

        let mut terminal_helper = Self {
            terminal: RefCell::new(terminal),
            percentage: 50,
            leagues_list: vec![],
            teams_list: vec![],
            players_list: vec![],
        };
        terminal_helper.render();

        terminal_helper
    }

    fn set_percentage(&mut self, percentage: u16) {
        self.percentage = min(percentage, 100)
    }

    fn push_list_item(list: &mut Vec<ListItem<'a>>, item: &'a str) {
        list.push(ListItem::new(item).style(Style::default().fg(Color::LightGreen)));
    }

    pub fn push_league_item(&mut self, league_name: &'a str) {
        Self::push_list_item(&mut self.leagues_list, league_name);
    }

    pub fn push_team_item(&mut self, team_name: &'a str) {
        Self::push_list_item(&mut self.teams_list, team_name);
    }

    pub fn push_player_item(&mut self, player_name: &'a str) {
        Self::push_list_item(&mut self.players_list, player_name);
    }

    fn render(&self) {
        // Internal mutability in order to access struct state
        self.terminal.borrow_mut().draw(|f| {
            // Layout
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());

            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .horizontal_margin(2)
                .constraints(
                    [
                        Constraint::Percentage(34),
                        Constraint::Percentage(34),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(vertical_chunks[0]);

            // Lists space
            let block = Block::default()
                .title(" SCRAPPING DETAILS ")
                .borders(Borders::ALL);
            f.render_widget(block, vertical_chunks[0]);

            let items = [
                ListItem::new("Item 1").style(Style::default().fg(Color::LightGreen)),
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
                .block(Block::default().title(" LEAGUES ").borders(Borders::ALL));
            f.render_widget(leagues_list, horizontal_chunks[0]);

            let teams_list = List::new(items.clone())
                .block(Block::default().title(" TEAMS ").borders(Borders::ALL));
            f.render_widget(teams_list, horizontal_chunks[1]);

            let mut state = ListState::default();
            state.select(Some(10));
            let players_list = List::new(items)
                .block(Block::default().title(" PLAYERS ").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::LightCyan))
                .highlight_symbol(">> ");
            f.render_stateful_widget(players_list, horizontal_chunks[2], &mut state);

            // Progress-bar space
            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .title("SCRAPPING PERCENTAGE")
                        .borders(Borders::ALL),
                )
                .gauge_style(Style::default().fg(Color::LightMagenta))
                .percent(self.percentage);
            f.render_widget(gauge, vertical_chunks[1]);
        });
    }
}
