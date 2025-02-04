use bevy::prelude::*;

mod actors;
mod animation;
mod atom;
mod chunk;
mod chunk_group;
mod chunk_manager;
mod consts;
mod debug;
mod geom_tools;
mod manager_api;
mod player;
mod prelude {
    pub use crate::atom::State;
    pub use crate::{
        actors::*, animation::*, atom::*, chunk::*, chunk_group::*, chunk_manager::*, consts::*,
        debug::*, geom_tools::*, manager_api::*, player::*,
    };
    pub use bevy::input::mouse::MouseScrollUnit;
    pub use bevy::input::mouse::MouseWheel;
    pub use bevy::math::{ivec2, uvec2, vec2, vec3};
    pub use bevy::prelude::*;
    pub use std::collections::{HashMap, HashSet};
}

use crate::animation::AnimationPlugin;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //local plugins
        .add_plugins((
            ChunkManagerPlugin,
            //DebugPlugin,
            ActorsPlugin,
            PlayerPlugin,
            AnimationPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.camera.hdr = true;
    camera.transform.scale.x = 0.67;
    camera.transform.scale.y = 0.67;

    commands.spawn(camera);
    commands.spawn(PreviousMousePos(None));
}
