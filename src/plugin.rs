//! Sprite Fusion plugin for Bevy.

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    loader::SpriteFusionMapLoader,
    types::{Collider, SpriteFusionLayerMarker, SpriteFusionMap, SpriteFusionMapMarker, TileAttributes},
};

/// Plugin that enables loading and rendering Sprite Fusion maps. Sprite Fusion is a free, web-based tilemap editor: https://www.spritefusion.com/
///
/// # Example
///
/// ```rust,ignore
/// use bevy::prelude::*;
/// use bevy_spritefusion::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(SpriteFusionPlugin)
///         .add_systems(Startup, spawn_map)
///         .run();
/// }
///
/// fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
///     commands.spawn(SpriteFusionBundle {
///         map: SpriteFusionMapHandle(asset_server.load("maps/level1.sf.json")),
///         tileset: SpriteFusionTilesetHandle(asset_server.load("maps/spritesheet.png")),
///         ..default()
///     });
/// }
/// ```
pub struct SpriteFusionPlugin;

impl Plugin for SpriteFusionPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteFusionMap>()
            .init_asset_loader::<SpriteFusionMapLoader>()
            .add_plugins(TilemapPlugin)
            .add_systems(Update, spawn_spritefusion_maps);
    }
}

/// Handle wrapper for SpriteFusion map assets.
#[derive(Component, Default, Clone, Debug, Deref, DerefMut)]
pub struct SpriteFusionMapHandle(pub Handle<SpriteFusionMap>);

/// Handle wrapper for tileset/spritesheet images.
#[derive(Component, Default, Clone, Debug, Deref, DerefMut)]
pub struct SpriteFusionTilesetHandle(pub Handle<Image>);

/// Bundle for spawning a SpriteFusion map.
#[derive(Bundle, Default)]
pub struct SpriteFusionBundle {
    /// Handle to the SpriteFusion map JSON file.
    pub map: SpriteFusionMapHandle,
    /// Handle to the tileset/spritesheet image.
    pub tileset: SpriteFusionTilesetHandle,
    /// Transform for the map.
    pub transform: Transform,
    /// Global transform (computed automatically).
    pub global_transform: GlobalTransform,
    /// Visibility of the map.
    pub visibility: Visibility,
    /// Inherited visibility (computed automatically).
    pub inherited_visibility: InheritedVisibility,
    /// View visibility (computed automatically).
    pub view_visibility: ViewVisibility,
    /// Marker that this map hasn't been spawned yet.
    pub pending: PendingSpriteFusionMap,
}

/// Marker component for maps that haven't been spawned yet.
#[derive(Component, Default)]
pub struct PendingSpriteFusionMap;


/// System that spawns tilemaps for pending SpriteFusion maps.
fn spawn_spritefusion_maps(
    mut commands: Commands,
    pending_maps: Query<(Entity, &SpriteFusionMapHandle, &SpriteFusionTilesetHandle, &Transform), With<PendingSpriteFusionMap>>,
    map_assets: Res<Assets<SpriteFusionMap>>,
    image_assets: Res<Assets<Image>>,
) {
    for (entity, map_handle, tileset_handle, transform) in pending_maps.iter() {
        // Wait for both assets to be loaded
        let Some(map) = map_assets.get(&**map_handle) else {
            continue;
        };
        let Some(_tileset_image) = image_assets.get(&**tileset_handle) else {
            continue;
        };

        // Remove pending marker and add map marker
        commands.entity(entity).remove::<PendingSpriteFusionMap>();
        commands.entity(entity).insert(SpriteFusionMapMarker {
            map: map.clone(),
        });

        let tile_size = map.tile_size;

        // Spawn each layer as a separate tilemap
        for (layer_index, layer) in map.layers.iter().enumerate() {
            let map_size = TilemapSize {
                x: map.map_width,
                y: map.map_height,
            };

            let tilemap_entity = commands.spawn_empty().id();
            let mut tile_storage = TileStorage::empty(map_size);

            // Spawn tiles for this layer
            for tile in &layer.tiles {
                let tile_id = tile.tile_id();
                let tile_pos = TilePos {
                    x: tile.x as u32,
                    y: (map.map_height - 1) - tile.y as u32, // Sprite Fusion uses top-left origin
                };

                // Calculate texture index from tile ID
                let texture_index = TileTextureIndex(tile_id);

                let mut tile_entity_commands = commands.spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index,
                    ..default()
                });

                // Add collider marker if layer has collision
                if layer.collider {
                    tile_entity_commands.insert(Collider);
                }

                // Add tile attributes if present
                if let Some(attrs) = &tile.attributes {
                    if !attrs.is_empty() {
                        tile_entity_commands.insert(TileAttributes(attrs.clone()));
                    }
                }

                let tile_entity = tile_entity_commands.id();
                tile_storage.set(&tile_pos, tile_entity);
            }

            let tile_size_vec = TilemapTileSize {
                x: tile_size as f32,
                y: tile_size as f32,
            };
            let grid_size = tile_size_vec.into();
            let map_type = TilemapType::Square;

            // Get the tileset handle from the wrapper
            let texture = TilemapTexture::Single(tileset_handle.0.clone());

            // Layer Z offset. In Sprite Fusion, layer 0 is on top, last layer is background
            // So need to invert: higher index = lower Z
            let layer_z = -((layer_index as f32) * 0.1);
            let layer_transform = Transform::from_translation(Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z + layer_z,
            ));

            commands.entity(tilemap_entity).insert((
                TilemapBundle {
                    grid_size,
                    map_type,
                    size: map_size,
                    storage: tile_storage,
                    texture,
                    tile_size: tile_size_vec,
                    transform: layer_transform,
                    ..default()
                },
                SpriteFusionLayerMarker {
                    name: layer.name.clone(),
                    index: layer_index,
                    collider: layer.collider,
                },
            ));

            // Make the tilemap a child of the map entity
            commands.entity(entity).add_child(tilemap_entity);
        }

        let tiles_with_attrs = map.layers.iter()
            .flat_map(|l| l.tiles.iter())
            .filter(|t| t.attributes.as_ref().map(|a| !a.is_empty()).unwrap_or(false))
            .count();
        
        info!(
            "Spawned SpriteFusion map with {} layers ({} tiles total, {} with attributes)",
            map.layers.len(),
            map.layers.iter().map(|l| l.tiles.len()).sum::<usize>(),
            tiles_with_attrs
        );
    }
}
