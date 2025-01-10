use serde::Serialize;

use crate::state;

#[derive(Serialize)]
pub struct Metrics {
    pub joke_count: u32,
    pub jokes_sent: u32,
    pub greets_sent: u32,
}

pub fn get_metrics() -> Metrics {
    state::read(|state| Metrics {
        joke_count: state.joke_count(),
        jokes_sent: state.jokes_sent(),
        greets_sent: state.greets_sent(),
    })
}
