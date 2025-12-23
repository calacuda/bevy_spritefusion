//! # bevy_spritefusion
//!
//! A Bevy plugin for loading and rendering [Sprite Fusion](https://www.spritefusion.com/) map exports.
//!
//! Sprite Fusion is a free, web-based tilemap editor: https://www.spritefusion.com/ now supports Bevy exports!
//! This plugin loads those exports and converts them to [`bevy_ecs_tilemap`] entities.
//!
//! ## Quick Start
//!
//! 1. Export your map from Sprite Fusion via the Bevy export button
//! 2. Place map.json and spritesheet.png files in your `assets/` folder
//! 3. Load and spawn the map:
//!
//! ```rust,ignore
//! use bevy::prelude::*;
//! use bevy_spritefusion::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(SpriteFusionPlugin)
//!         .add_systems(Startup, spawn_map)
//!         .run();
//! }
//!
//! fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     commands.spawn(Camera2d);
//!     commands.spawn(SpriteFusionBundle {
//!         map: asset_server.load("map.json"),
//!         tileset: asset_server.load("spritesheet.png"),
//!         ..default()
//!     });
//! }
//! ```
//!
//! ## Features
//!
//! - **Layers**: Each Sprite Fusion layer becomes a separate tilemap
//! - **Colliders**: Layers marked as colliders get a `Collider` component on their tiles
//! - **Tile Attributes**: Custom attributes from Sprite Fusion are preserved as `TileAttributes` components. They can be useful for things like areas data, danger zones, etc.
//! - **bevy_ecs_tilemap Integration**: Full compatibility with the bevy_ecs_tilemap ecosystem
//!
//! ## Querying Tiles
//!
//! ```rust,ignore
//! use bevy::prelude::*;
//! use bevy_spritefusion::prelude::*;
//!
//! // Find all tiles with collision
//! fn find_colliders(query: Query<&TilePos, With<Collider>>) {
//!     for pos in query.iter() {
//!         println!("Collider at ({}, {})", pos.x, pos.y);
//!     }
//! }
//!
//! // Find tiles with specific attributes
//! fn find_collectibles(query: Query<(&TilePos, &TileAttributes)>) {
//!     for (pos, attrs) in query.iter() {
//!         if attrs.get_bool("isCollectible").unwrap_or(false) {
//!             let value = attrs.get_i64("value").unwrap_or(0);
//!             println!("Collectible at ({}, {}) worth {}", pos.x, pos.y, value);
//!         }
//!     }
//! }
//! ```

pub mod loader;
pub mod plugin;
pub mod types;

/// Convenient re-exports for common usage.
pub mod prelude {
    pub use crate::loader::SpriteFusionMapLoader;
    pub use crate::plugin::{
        PendingSpriteFusionMap, SpriteFusionBundle, SpriteFusionMapHandle, SpriteFusionPlugin,
        SpriteFusionTilesetHandle,
    };
    pub use crate::types::{
        Collider, SpriteFusionLayer, SpriteFusionLayerMarker, SpriteFusionMap,
        SpriteFusionMapMarker, SpriteFusionTile, TileAttributes,
    };
    pub use bevy_ecs_tilemap::prelude::TilePos;
}
