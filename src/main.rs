use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::prelude::*;
use bevy_prototype_debug_lines::*;
use rand::{thread_rng, Rng};

#[derive(Component)]
struct Name(String);
fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup_tiles)
        .add_startup_system_to_stage(StartupStage::PostStartup, randomize_tiles)
        .add_system(debug_system)
        .run();
}

#[derive(Bundle)]
struct CelestialBody {
    name: Name,

    #[bundle]
    sprite: SpriteBundle,
}

fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let tilemap_size = TilemapSize { x: 32, y: 32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(tilemap_size);

    // Spawn the elements of the tilemap.
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                0.0,
            ),
            ..Default::default()
        });
    info!("Tiles were set up");
}

fn randomize_tiles(mut query: Query<&mut TileTexture>) {
    let mut random = thread_rng();
    for mut tile in query.iter_mut() {
        tile.0 = random.gen_range(0..6);
        info!("Tile texture index is: {}", tile.0);
    }
    info!("Tiles were randomized");
}

fn debug_system(mut lines: ResMut<DebugLines>) {
    let start = Vec3::splat(-89.0);
    let end = Vec3::splat(29.0);
    let duration = 0.0; // Duration of 0 will show the line for 1 frame.
    lines.line(start, end, duration);
}
