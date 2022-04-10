use crate::pattern::Pattern;
use tui::widgets::TableState;

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Pattern,
    Samples,
    Details,
}
impl From<usize> for MenuItem {
    fn from(input: usize) -> MenuItem {
        match input {
            0 => MenuItem::Pattern,
            1 => MenuItem::Samples,
            2 => MenuItem::Details,
            _ => todo!(),
        }
    }
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Pattern => 0,
            MenuItem::Samples => 1,
            MenuItem::Details => 2,
            //  _ => 2,
        }
    }
}

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

/// App holds the state of the application
pub struct App<'a> {
    /// All patterns for logs
    pub patterns: Vec<Pattern>,
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub pattern_table_state: TableState,
    pub sample_table_state: TableState,
    pub active_menu_item: MenuItem,
    pub current_rawlog: String,
    pub scroll: u16,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, patterns: Vec<Pattern>) -> App<'a> {
        let tabs = TabsState::new(vec!["Pattern", "Sample", "Detail"]);
        let mut pattern_table_state = TableState::default();
        let mut sample_table_state = TableState::default();
        pattern_table_state.select(Some(0));
        sample_table_state.select(Some(0));
        let active_menu_item = MenuItem::Pattern;
        App {
            patterns,
            title,
            tabs,
            pattern_table_state,
            sample_table_state,
            active_menu_item,
            current_rawlog: String::new(),
            scroll: 0,
        }
    }
    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn calculate_percent(&mut self) {
        // get total count from patterns
        let mut total = 0;
        for pattern in &self.patterns {
            total += pattern.count;
        }

        // calculate percent
        for pattern in self.patterns.iter_mut() {
            let percent = (pattern.count as f32 / total as f32) * 100.0;
            pattern.percent = Some(percent);
        }
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn current_menu_item(&self) -> MenuItem {
        self.tabs.index.into()
    }

    pub fn current_amount_samples(&self) -> usize {
        self.patterns[self.pattern_table_state.selected().unwrap()]
            .samples
            .len()
    }

    pub fn handle_down_patterns(&mut self) {
        if let Some(selected) = self.pattern_table_state.selected() {
            let amount_patterns = self.patterns.len();
            if selected >= amount_patterns - 1 {
                self.pattern_table_state.select(Some(0));
            } else {
                self.pattern_table_state.select(Some(selected + 1));
            }
        }
    }
    pub fn handle_down_samples(&mut self) {
        let current_amount_samples = self.current_amount_samples();
        if let Some(selected) = self.sample_table_state.selected() {
            if selected >= current_amount_samples - 1 {
                self.sample_table_state.select(Some(0));
            } else {
                self.sample_table_state.select(Some(selected + 1));
            }
        }
    }
    pub fn handle_up_patterns(&mut self) {
        if let Some(selected) = self.pattern_table_state.selected() {
            let amount_patterns = self.patterns.len();
            if selected > 0 {
                self.pattern_table_state.select(Some(selected - 1));
            } else {
                self.pattern_table_state.select(Some(amount_patterns - 1));
            }
        }
    }
    pub fn handle_up_samples(&mut self) {
        let current_amount_samples = self.current_amount_samples();
        if let Some(selected) = self.sample_table_state.selected() {
            if selected > 0 {
                self.sample_table_state.select(Some(selected - 1));
            } else {
                self.sample_table_state
                    .select(Some(current_amount_samples - 1));
            }
        }
    }

    pub fn current_pattern(&self) -> &Pattern {
        &self.patterns[self.pattern_table_state.selected().unwrap()]
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll += 1;
    }

    pub fn current_sample_rawlog(&self) -> &str {
        &self.patterns[self.pattern_table_state.selected().unwrap()].samples
            [self.sample_table_state.selected().unwrap()]
        .rawlog
    }
}
