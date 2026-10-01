use leptos::prelude::*;

pub mod components;
pub mod hooks;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NavmenuState {
    pub shown: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavmenuStateAction {
    Open,
    Close,
    Toggle,
}

impl NavmenuState {
    pub fn reduce(self, action: NavmenuStateAction) -> Self {
        match action {
            NavmenuStateAction::Open => Self { shown: true },
            NavmenuStateAction::Close => Self { shown: false },
            NavmenuStateAction::Toggle => Self { shown: !self.shown },
        }
    }
}

pub type NavmenuStateContext = ArcRwSignal<NavmenuState>;
