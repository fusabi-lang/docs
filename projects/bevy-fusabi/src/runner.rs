use crate::prelude::*;
use bevy::prelude::*;
use fusabi_vm::Vm;

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_scripts);
    }
}

#[derive(Component)]
pub struct RunScript {
    pub handle: Handle<FusabiScript>,
    pub executed: bool,
}

fn run_scripts(mut query: Query<&mut RunScript>, scripts: Res<Assets<FusabiScript>>) {
    for mut runner in query.iter_mut() {
        if runner.executed {
            continue;
        }

        // We need to clone the handle to use it for lookup, as we can't borrow from runner while mutating it
        // actually we can just use &runner.handle
        if let Some(script) = scripts.get(&runner.handle) {
            println!("Executing script: {}", script.name);

            // Deserialize chunk
            match script.to_chunk() {
                Ok(chunk) => {
                    // Create VM (thread-local or on-demand)
                    let mut vm = Vm::new();

                    // Execute
                    match vm.execute(chunk) {
                        Ok(value) => {
                            println!("Script execution result: {:?}", value);
                            runner.executed = true;
                        }
                        Err(e) => {
                            println!("Script execution failed: {:?}", e);
                            // Retry? Or mark failed?
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to load chunk: {}", e);
                }
            }
        }
    }
}
