use bevy::prelude::*;

use super::components::Name;

#[derive(Bundle)]
pub struct Person {
    name: Name,
    #[bundle]
    sprite: SpriteBundle,
}

pub fn init_people(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Person {
        name: Name(String::from("Test guy")),
        sprite: SpriteBundle {
            texture: asset_server.load("dead_person.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                ..Default::default()
            },
            ..Default::default()
        },
    });
}
