use std::fmt::Debug;

use tui_textarea::TextArea;

use super::{date_list::DateListScreen, transactions_list::TransactionListScreen, App, AppState};

#[derive(Debug, Clone)]
pub enum NewTransactionScreenFocus{
    DescriptionInput,
    DateInput
}

pub enum NewTransactionParent{
    DateList(DateListScreen),
    TransactionsList(TransactionListScreen)
}

pub struct NewTransactionScreen{
    pub description_text_area: TextArea<'static>,
    pub date_text_area: TextArea<'static>,
    pub focus: NewTransactionScreenFocus,
    pub parent: NewTransactionParent,

    pub date: Option<String>
}

impl NewTransactionScreen {
    pub fn new(parent: NewTransactionParent,date: Option<String>)->Self{
        Self {parent: parent,focus: NewTransactionScreenFocus::DescriptionInput, description_text_area: App::get_new_focused_text_area("Description", ""), date_text_area: App::get_new_text_area("Date separated by /", ""), date: date }
    }
}

impl Debug for NewTransactionScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}\n{:?} {:?}", self.description_text_area, self.date_text_area, self.focus, self.date)
    }
}

