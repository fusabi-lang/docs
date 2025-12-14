use bevy::prelude::*;
use bevy_fusabi::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .add_plugins(FusabiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, check_asset)
        .run();
}

#[derive(Resource)]
struct ScriptHandle(Handle<FusabiScript>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("hello.fsx");
    commands.insert_resource(ScriptHandle(handle));
    println!("Loading script...");
}

fn check_asset(
    mut events: EventReader<AssetEvent<FusabiScript>>,
    scripts: Res<Assets<FusabiScript>>,
) {
    for event in events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } => {
                let script = scripts.get(*id).unwrap();
                println!("Script loaded successfully: {}", script.name);
                println!("Bytecode size: {} bytes", script.bytecode.len());

                // Verify we can deserialize it
                match script.to_chunk() {
                    Ok(chunk) => println!(
                        "Deserialized chunk successfully. Opcode count: {}",
                        chunk.instructions.len()
                    ),
                    Err(e) => println!("Failed to deserialize chunk: {}", e),
                }
            }
            _ => {}
        }
    }
}
