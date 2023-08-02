use iced::{Element, Renderer, Theme};

pub type ShortElement<'a> = Element<'a, Message, Renderer<Theme>>;

#[derive(Clone, Debug)]
pub enum Message {
    SetPage(Page),
    UpdateCell1(String),
    UpdateCell2(String),
    UpdateCell3(String),
    Calculate
}

#[derive(Clone, Debug, Default)]
pub enum Page {
    #[default]
    Selection,
    QuadraticEquations,
    BasesConverter
}
