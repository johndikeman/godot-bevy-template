use bevy::prelude::*;
use godot::engine::Node;
use godot::prelude::*;
use bevy::utils::HashMap;

struct MyExtension;

#[derive(GodotClass)]
#[class(base=Node)]
struct BevyECS {
    app: App,
    #[base]
    node: Base<Node>,
}

#[derive(Resource)]
struct GodotSharedData(HashMap<u32, Vec3>);

impl Default for GodotSharedData {
    fn default() -> GodotSharedData {
        Self ( 
            HashMap::new(),
        ) 
    }
}



#[godot_api]
impl BevyECS {
    fn bevy_startup(_data: ResMut<GodotSharedData>, _commands: Commands) {

    }
    // put your bevy ecs methods here, use #[func] to make methods accessible to godot 
}

#[godot_api]
impl INode for BevyECS {
    fn init(node: Base<Node>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            node,
            app: App::new(),
        }
    }

    fn ready(&mut self) {
        self.app.init_resource::<GodotSharedData>();

        self.app.add_plugins(MinimalPlugins);

        self.app.add_systems(Startup, BevyECS::bevy_startup);

        godot_print!("bevy ecs ready!"); // Prints to the Godot console
    }

    fn process(&mut self, _delta: f64) {
        self.app.update();
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
