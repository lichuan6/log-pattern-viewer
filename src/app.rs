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
