use crossterm::{
    cursor::MoveTo,
    ExecutableCommand,
};
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend, layout::Constraint, layout::Direction, layout::Layout, style::Color,
    style::Style, widgets::Block, widgets::Borders, widgets::Gauge, widgets::List,
    widgets::ListItem,
};
use tui::{widgets::ListState, Terminal};

pub struct TerminalHelper<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    percentage: f64,
    leagues_list: Vec<ListItem<'a>>,
    teams_list: Vec<ListItem<'a>>,
    players_list: Vec<ListItem<'a>>,
    is_finished: bool,
}

impl<'a> TerminalHelper<'a> {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Error while instanciating terminal");

        terminal.clear().expect("Error while clearing the terminal");
        terminal
            .hide_cursor()
            .expect("Error while hiding the cursor of the terminal");

        let mut terminal_helper = Self {
            terminal,
            percentage: 0.0,
            leagues_list: vec![],
            teams_list: vec![],
            players_list: vec![],
            is_finished: false,
        };

        terminal_helper.render();

        terminal_helper
    }

    pub fn set_percentage(&mut self, percentage: f64) {
        if percentage > 100.0 {
            self.percentage = 100.0;
        } else {
            self.percentage = percentage;
        }
        self.render();
    }

    pub fn add_percentage(&mut self, percentage: f64) {
        let new_percentage = self.percentage + percentage;
        self.set_percentage(new_percentage);
    }

    pub fn close(&mut self) {
        self.is_finished = true;
        self.render();

        let terminal_size = self
            .terminal
            .size()
            .expect("Error while getting terminal size");
        self.terminal
            .backend_mut()
            .execute(MoveTo(0, terminal_size.height))
            .expect("Error while moving cursor");

        self.terminal
            .show_cursor()
            .expect("Error while showing the cursor of the terminal");
    }

    fn generate_list_item(item: String) -> ListItem<'a> {
        ListItem::new(item).style(Style::default().fg(Color::LightGreen))
    }

    pub fn push_league_item(&mut self, league_name: String) {
        self.leagues_list
            .push(Self::generate_list_item(league_name));
        self.render();
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
        self.players_list
            .push(Self::generate_list_item(player_name));
        self.render();
    }

    pub fn clean_players_list(&mut self) {
        if !self.players_list.is_empty() {
            self.players_list.clear();
        }
    }

    fn get_list_state(list: &[ListItem], finished: bool) -> ListState {
        let state = if list.is_empty() {
            None
        } else if finished {
            // No item selected but padding is maintained
            Some(list.len())
        } else {
            Some(list.len() - 1)
        };

        let mut list_state = ListState::default();
        list_state.select(state);

        list_state
    }

    fn render(&mut self) {
        let is_finished = self.is_finished;
        let percentage = self.percentage;
        let leagues_list = self.leagues_list.clone();
        let teams_list = self.teams_list.clone();
        let players_list = self.players_list.clone();

        self.terminal
            .draw(|f| {
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
                let mut leagues_list_state = Self::get_list_state(&leagues_list, is_finished);
                let leagues_list = List::new(leagues_list)
                    .block(Block::default().title(" LEAGUES ").borders(Borders::ALL))
                    .highlight_style(Style::default().fg(Color::LightCyan))
                    .highlight_symbol(">> ");
                f.render_stateful_widget(
                    leagues_list,
                    horizontal_chunks[0],
                    &mut leagues_list_state,
                );

                let mut teams_list_state = Self::get_list_state(&teams_list, is_finished);
                let teams_list = List::new(teams_list)
                    .block(Block::default().title(" TEAMS ").borders(Borders::ALL))
                    .highlight_style(Style::default().fg(Color::LightCyan))
                    .highlight_symbol(">> ");
                f.render_stateful_widget(teams_list, horizontal_chunks[1], &mut teams_list_state);

                let mut player_list_state = Self::get_list_state(&players_list, is_finished);
                let players_list = List::new(players_list)
                    .block(Block::default().title(" PLAYERS ").borders(Borders::ALL))
                    .highlight_style(Style::default().fg(Color::LightCyan))
                    .highlight_symbol(">> ");
                f.render_stateful_widget(
                    players_list,
                    horizontal_chunks[2],
                    &mut player_list_state,
                );

                // Progress-bar space
                let gauge = Gauge::default()
                    .block(
                        Block::default()
                            .title("SCRAPPING PERCENTAGE")
                            .borders(Borders::ALL),
                    )
                    .gauge_style(Style::default().fg(Color::LightMagenta))
                    .percent(percentage.round() as u16);
                f.render_widget(gauge, vertical_chunks[1]);
            })
            .expect("Error while rendering in the terminal");
    }
}

impl<'a> Drop for TerminalHelper<'a> {
    fn drop(&mut self) {
        self.close();
    }
}
