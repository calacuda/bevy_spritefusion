//! Basic example showing how to load and display a Sprite Fusion map. Sprite Fusion is a free, web-based tilemap editor: https://www.spritefusion.com/
//!
//! Run with: `cargo run --example basic`
//!
//! Make sure you have `map.json` and `spritesheet.png` in the `assets/` folder.

use bevy::prelude::*;
use bevy_spritefusion::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // You need this for crisp pixel art rendering
        .add_plugins(SpriteFusionPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_movement, print_collectibles))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(304.0, 112.0, 0.0),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn(SpriteFusionBundle {
        map: SpriteFusionMapHandle(asset_server.load("map.json")),
        tileset: SpriteFusionTilesetHandle(asset_server.load("spritesheet.png")),
        ..default()
    });

    info!("Loading SpriteFusion map...");
}

fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    let transform = &mut *camera;

    let speed = 200.0 * time.delta_secs();

    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= speed;
    }
}

/// Access the tile custom attributes you can set in Sprite Fusion.
fn print_collectibles(query: Query<(&TilePos, &TileAttributes)>, mut has_run: Local<bool>) {
    if query.is_empty() || *has_run {
        return;
    }
    *has_run = true;

    info!("Tiles with attributes:");
    for (pos, attrs) in query.iter() {
        if let Some(name) = attrs.get_str("name") {
            let value = attrs.get_i64("value").unwrap_or(0);
            let is_collectible = attrs.get_bool("isCollectible").unwrap_or(false);
            info!(
                "  - '{}' at ({}, {}), value: {}, collectible: {}",
                name, pos.x, pos.y, value, is_collectible
            );
        }
    }
}
