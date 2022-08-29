//! Our application state and relevant methods
//!
//! Druid uses `Data` to know when it should re-render

use druid::Data;

#[derive(Clone, Data)]
pub struct AppState {}
