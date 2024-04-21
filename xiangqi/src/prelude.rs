pub use crate::components::*;
pub use crate::menu::*;
pub use crate::resources::*;
pub use crate::status::*;
pub use crate::types::*;
pub(crate) use bevy::prelude::*;
pub(crate) use bevy_egui::egui;
pub(crate) use bevy_egui::EguiContexts;
pub(crate) use bevy_http_client::prelude::*;
pub(crate) use num_enum::IntoPrimitive;
pub(crate) use std::collections::{HashMap, HashSet};
pub(crate) use std::sync::{Arc, RwLock};
pub(crate) use transfer::*;

pub static WIDTH: f32 = 1600.0;
pub static HEIGHT: f32 = 900.0;
pub static FILES: usize = 9;
pub static RANKS: usize = 10;
pub static MOVE_DIRS: [MoveDir; 4] = [MoveDir::Left, MoveDir::Right, MoveDir::Up, MoveDir::Down];
pub static DIAG_DIRS: [DiagDir; 4] = [DiagDir::LU, DiagDir::LD, DiagDir::RU, DiagDir::RD];
