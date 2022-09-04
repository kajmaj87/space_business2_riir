use bevy::prelude::*;

use crate::logic::components::{Dead, Person};

pub fn death_system(
    mut query: Query<(&Person, &mut Handle<Image>), Changed<Dead>>,
    asset_server: Res<AssetServer>,
) {
    for (_, mut image) in query.iter_mut() {
        *image = asset_server.load("dead_person.png");
    }
}
