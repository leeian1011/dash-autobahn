use serde::Serialize;

#[derive (Clone, Debug, Serialize)]
pub struct Lane {
    lane: String,
    nickname: String,
    index: u64,
}

pub enum Selection {
    Add(Lane),
    Remove(Lane),
    List,
    Dash(Lane),
    Help,
}

