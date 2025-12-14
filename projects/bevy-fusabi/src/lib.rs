pub mod asset;
pub mod loader;
pub mod runner;

use asset::FusabiScript;
use bevy::prelude::*;
use loader::FusabiLoader;

pub struct FusabiPlugin;

impl Plugin for FusabiPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<FusabiScript>()
            .init_asset_loader::<FusabiLoader>();
    }
}

pub mod prelude {
    pub use crate::asset::FusabiScript;
    pub use crate::runner::{RunScript, RunnerPlugin};
    pub use crate::FusabiPlugin;
}
