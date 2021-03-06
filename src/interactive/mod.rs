pub mod drag_and_drop;
pub mod highlight;
pub mod mouse_event;
pub mod window;

pub use drag_and_drop::*;
pub use highlight::*;
pub use mouse_event::*;
pub use window::*;

use bevy::prelude::Component;

#[derive(Component)]
pub struct TargetSquare;

#[derive(Component)]
pub struct SelectedSquare();
