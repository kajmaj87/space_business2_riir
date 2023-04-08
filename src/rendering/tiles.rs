use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    config::Config,
    logic::components::{FoodAmount, FoodSource, GridCoords},
};
use crate::logic::components::FoodLookup;

const FIRST_FOOD_TILE_INDEX: u32 = 2;
pub const TILE_SIZE: f32 = 16.0;

pub fn update_food_tiles(
    mut query: Query<(&mut TileTextureIndex, &FoodAmount), Changed<FoodAmount>>,
) {
    for (mut tile, food_amount) in query.iter_mut() {
        tile.0 = food_amount.0 + FIRST_FOOD_TILE_INDEX;
    }
}

pub fn randomize_tiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TileTextureIndex, &GridCoords)>,
    config: Res<Config>,
    mut food_lookup: ResMut<FoodLookup>,
) {
    let mut random = thread_rng();
    for (entity, mut tile, coords) in query.iter_mut() {
        if random.gen_range(0.0..1.0) < (50.0 - ((coords.x + coords.y) as f32) * 0.4) / 100.0 {
            tile.0 = random.gen_range(2..6);
        } else {
            // tile.0 = random.gen_range(0..2);
            tile.0 = 0;
        }
        if (2..6).contains(&tile.0) {
            let food_amount = tile.0 - FIRST_FOOD_TILE_INDEX;
            commands
                .entity(entity)
                .insert(FoodSource())
                .insert(FoodAmount(food_amount));
            // insert entity to food_lookup using coords as key
            food_lookup.food.insert(*coords, entity);
        }
    }
    info!("Tiles were randomized");
}

pub fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let tilemap_size = TilemapSize {
        x: config.map.size_x.value,
        y: config.map.size_y.value,
    };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(tilemap_size);

    // Spawn the elements of the tilemap.
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let coords = GridCoords { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .insert(coords)
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        // transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
        //     &tilemap_size,
        //     &tile_size,
        //     0.0,
        // ),
        ..Default::default()
    });
    info!("Tiles were set up");
}
