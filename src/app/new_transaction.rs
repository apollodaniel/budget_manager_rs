use std::fmt::Debug;

use tui_textarea::TextArea;

use super::{date_list::DateListScreen, transactions_list::TransactionListScreen, App, MoveSelection};

#[derive(Debug, Clone, PartialEq)]
pub enum NewTransactionScreenFocus{
    DescriptionInput,
    AmountInput,
    DateInput
}
impl NewTransactionScreenFocus {
    pub fn get_new_focus(focus: NewTransactionScreenFocus,move_selection: MoveSelection)->NewTransactionScreenFocus{
        let new_focus = match move_selection {
            MoveSelection::Down => {
                match focus {
                    NewTransactionScreenFocus::DescriptionInput => Self::AmountInput,
                    NewTransactionScreenFocus::AmountInput => Self::DateInput,
                    NewTransactionScreenFocus::DateInput => Self::DescriptionInput,
                }
            },
            MoveSelection::Up => {
                match focus {
                    NewTransactionScreenFocus::DescriptionInput => Self::DateInput,
                    NewTransactionScreenFocus::AmountInput => Self::DescriptionInput,
                    NewTransactionScreenFocus::DateInput => Self::AmountInput,
                }
            }
        };
        new_focus
    }
}

pub enum NewTransactionParent{
    DateList(DateListScreen),
    TransactionsList(TransactionListScreen)
}

pub struct NewTransactionScreen{
    pub description_text_area: TextArea<'static>,
    pub date_text_area: TextArea<'static>,
    pub amount_text_area: TextArea<'static>,
    pub focus: NewTransactionScreenFocus,
    pub parent: NewTransactionParent,

    pub date: Option<String>
}

impl NewTransactionScreen {
    pub fn new(parent: NewTransactionParent,date: Option<String>)->Self{
        Self {parent: parent,focus: NewTransactionScreenFocus::DescriptionInput, description_text_area: App::get_new_focused_text_area("Description", ""), amount_text_area: App::get_new_text_area("Amount", "") ,date_text_area: App::get_new_text_area("Date separated by /", ""), date: date }
    }
}

impl Debug for NewTransactionScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}\n{:?} {:?}", self.description_text_area, self.date_text_area, self.focus, self.date)
    }
}

