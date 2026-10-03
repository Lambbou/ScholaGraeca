use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;
pub const MAP_WIDTH: i32 = 40;
pub const MAP_HEIGHT: i32 = 30;

#[derive(Component)]
pub struct SolidObstacle;

#[derive(Component)]
pub struct MapTile;

pub fn spawn_overworld_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let water_handle: Handle<Image> = asset_server.load("textures/tile_water.png");
    let sand_handle: Handle<Image> = asset_server.load("textures/tile_sand.png");
    let grass_handle: Handle<Image> = asset_server.load("textures/tile_grass.png");
    let rock_handle: Handle<Image> = asset_server.load("textures/tile_rock.png");
    let stonepath_handle: Handle<Image> = asset_server.load("textures/tile_stonepath.png");
    let column_handle: Handle<Image> = asset_server.load("textures/prop_column.png");

    let offset_x = -(MAP_WIDTH as f32 * TILE_SIZE) / 2.0;
    let offset_y = -(MAP_HEIGHT as f32 * TILE_SIZE) / 2.0;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let world_x = offset_x + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let world_y = offset_y + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

            // Découpage géographique du Litus Epiri :
            // Gauche / Ouest : Mer Ionienne (x < 8)
            // Bord de mer : Plage de sable (8 <= x < 20)
            // Est : Végétation et sentier antique (20 <= x < 35)
            // Bordures Nord/Est : Falaises rocheuses (y < 2 ou y > 27 ou x >= 37)
            let is_cliff = x >= 37 || y <= 1 || y >= MAP_HEIGHT - 2;
            let is_water = x <= 6;
            let is_path = (y == 14 || y == 15) && x >= 14 && x <= 36;
            let is_sand = x > 6 && x < 22 && !is_path && !is_cliff;

            let (texture, is_solid, z_index) = if is_cliff {
                (rock_handle.clone(), true, 1.0)
            } else if is_water {
                (water_handle.clone(), true, 0.0)
            } else if is_path {
                (stonepath_handle.clone(), false, 0.5)
            } else if is_sand {
                (sand_handle.clone(), false, 0.0)
            } else {
                (grass_handle.clone(), false, 0.0)
            };

            let mut entity = commands.spawn((
                Sprite::from_image(texture),
                Transform::from_xyz(world_x, world_y, z_index),
                MapTile,
            ));

            if is_solid {
                entity.insert(SolidObstacle);
            }

            // Décor : Ruines de colonnes antiques le long du sentier
            if (x == 16 && y == 17) || (x == 26 && y == 17) || (x == 32 && y == 11) {
                commands.spawn((
                    Sprite::from_image(column_handle.clone()),
                    Transform::from_xyz(world_x, world_y + 8.0, 2.0),
                    SolidObstacle,
                    MapTile,
                ));
            }
        }
    }
}
