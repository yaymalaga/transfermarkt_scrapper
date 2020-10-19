use std::{cell::RefCell, cmp::min, io::{self, Stdout}};
use tui::{
    backend::CrosstermBackend, layout::Constraint, layout::Direction, layout::Layout, style::Color,
    style::Style, text::Span, text::Spans, widgets::Block, widgets::Borders, widgets::Gauge,
    widgets::List, widgets::ListItem,
};
use tui::{layout::Corner, widgets::ListState, Terminal};

pub struct TerminalHelper<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
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
            terminal,
            percentage: 50,
            leagues_list: vec![],
            teams_list: vec![],
            players_list: vec![],
        };
        terminal_helper.render();

        terminal_helper
    }

    pub fn set_percentage(&mut self, percentage: u16) {
        self.percentage = min(percentage, 100);
        self.render();
    }

    pub fn close(&mut self) {
        // Force full re-drawn so the terminal's new line appears below the TUI
        self.terminal.clear();
        self.render()
    }

    fn generate_list_item(item: String) -> ListItem<'a> {
        ListItem::new(item).style(Style::default().fg(Color::LightGreen))
    }

    pub fn push_league_item(&mut self, league_name: String) {
        self.leagues_list.push(Self::generate_list_item(league_name));
        self.render();
    }

    pub fn clean_leagues_list(&mut self) {
        if !self.leagues_list.is_empty() {
            self.leagues_list.clear();
        }
    }

    pub fn push_team_item(&mut self, team_name: String) {
        self.teams_list.push(Self::generate_list_item(team_name));
        self.render();
    }

    pub fn clean_teams_list(&mut self) {
        if !self.teams_list.is_empty() {
            self.teams_list.clear();
        }
    }

    pub fn push_player_item(&mut self, player_name: String) {
        self.players_list.push(Self::generate_list_item(player_name));
        self.render();
    }

    pub fn clean_players_list(&mut self) {
        if !self.players_list.is_empty() {
            self.players_list.clear();
        }
    }

    fn get_list_state(list: &Vec<ListItem>) -> ListState {
        let state = if list.len() == 0 {
            None
        } else {
            Some(list.len() - 1)
        };

        let mut list_state = ListState::default();
        list_state.select(state);

        list_state
    }

    fn render(&mut self) {
        let percentage = self.percentage;
        let leagues_list = self.leagues_list.clone();
        let teams_list = self.teams_list.clone();
        let players_list = self.players_list.clone();

        self.terminal.draw(|f| {
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

            let block = Block::default()
                .title(" SCRAPPING DETAILS ")
                .borders(Borders::ALL);
            f.render_widget(block, vertical_chunks[0]);

            // Lists space
            let mut leagues_list_state = Self::get_list_state(&leagues_list);
            let leagues_list = List::new(leagues_list)
                .block(Block::default().title(" LEAGUES ").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::LightCyan))
                .highlight_symbol(">> ");
            f.render_stateful_widget(leagues_list, horizontal_chunks[0], &mut leagues_list_state);

            let mut teams_list_state = Self::get_list_state(&teams_list);
            let teams_list = List::new(teams_list)
                .block(Block::default().title(" TEAMS ").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::LightCyan))
                .highlight_symbol(">> ");
            f.render_stateful_widget(teams_list, horizontal_chunks[1], &mut teams_list_state);

            let mut player_list_state = Self::get_list_state(&players_list);
            let players_list = List::new(players_list)
                .block(Block::default().title(" PLAYERS ").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::LightCyan))
                .highlight_symbol(">> ");
            f.render_stateful_widget(players_list, horizontal_chunks[2], &mut player_list_state);

            // Progress-bar space
            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .title("SCRAPPING PERCENTAGE")
                        .borders(Borders::ALL),
                )
                .gauge_style(Style::default().fg(Color::LightMagenta))
                .percent(percentage);
            f.render_widget(gauge, vertical_chunks[1]);
        });
    }
}
