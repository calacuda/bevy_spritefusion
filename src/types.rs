//! Sprite Fusion map data types.
//!
//! These types match the JSON export format from Sprite Fusion.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete SpriteFusion map export.
///
/// This is the root type that gets deserialized from the SpriteFusion JSON export.
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SpriteFusionMap {
    /// Size of each tile in pixels.
    pub tile_size: u32,
    /// Width of the map in tiles.
    pub map_width: u32,
    /// Height of the map in tiles.
    pub map_height: u32,
    /// All layers in the map, ordered from top to bottom (first layer is on top, last is background).
    pub layers: Vec<SpriteFusionLayer>,
}

/// A single layer in a SpriteFusion map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteFusionLayer {
    /// Name of the layer.
    pub name: String,
    /// Whether this layer should have collision enabled.
    #[serde(default)]
    pub collider: bool,
    /// All tiles in this layer.
    pub tiles: Vec<SpriteFusionTile>,
}

/// A single tile in a SpriteFusion layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteFusionTile {
    /// Tile ID referencing the index in the spritesheet.
    /// This is a string in SpriteFusion's format (e.g., "0", "1").
    pub id: String,
    /// X position in tile coordinates.
    pub x: i32,
    /// Y position in tile coordinates.
    pub y: i32,
    /// Optional custom attributes attached to this tile.
    #[serde(default)]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
}

impl SpriteFusionTile {
    /// Get the tile ID as a u32.
    pub fn tile_id(&self) -> u32 {
        self.id.parse().unwrap_or(0)
    }
}

/// Component attached to spawned tilemap entities.
#[derive(Component, Debug, Clone)]
pub struct SpriteFusionMapMarker {
    /// The original map data.
    pub map: SpriteFusionMap,
}

/// Component attached to layer entities.
#[derive(Component, Debug, Clone)]
pub struct SpriteFusionLayerMarker {
    /// Name of the layer.
    pub name: String,
    /// Layer index (0 = bottom).
    pub index: usize,
    /// Whether this layer has collision.
    pub collider: bool,
}

/// Component attached to tiles that have custom attributes.
#[derive(Component, Debug, Clone)]
pub struct TileAttributes(pub HashMap<String, serde_json::Value>);

impl TileAttributes {
    /// Get an attribute as a string.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.as_str())
    }

    /// Get an attribute as a bool.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.0.get(key).and_then(|v| v.as_bool())
    }

    /// Get an attribute as an i64.
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.0.get(key).and_then(|v| v.as_i64())
    }

    /// Get an attribute as an f64.
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.0.get(key).and_then(|v| v.as_f64())
    }

    /// Check if an attribute exists.
    pub fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }
}

/// Marker component for tiles that are on a collider layer.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Collider;
