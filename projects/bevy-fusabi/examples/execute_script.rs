use bevy::prelude::*;
use bevy_fusabi::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .add_plugins(FusabiPlugin)
        .add_plugins(RunnerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("hello.fsx");

    // Spawn an entity that wants to run this script
    commands.spawn(RunScript {
        handle,
        executed: false,
    });
}
