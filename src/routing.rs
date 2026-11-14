use crate::data::projects::Project;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Page {
    Home,
    Projects,
    Resume,
    About,
}

impl Page {
    pub(crate) fn from_hash(hash: &str) -> Self {
        match hash {
            "#projects" | "#school_projects" | "#personal_projects" => Self::Projects,
            _ if Project::from_hash(hash).is_some() => Self::Projects,
            "#about" => Self::About,
            "#resume" => Self::Resume,
            _ => Self::Home,
        }
    }
}

pub(crate) fn current_hash() -> String {
    window().location().hash().unwrap_or_default()
}
