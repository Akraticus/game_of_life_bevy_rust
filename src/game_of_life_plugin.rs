use bevy::{prelude::*};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;

use crate::{utility::keycode_just_pressed, tick_plugin::TurnSystemSet};

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup)
            .add_system(set_texture_based_on_cell_type)
            // .add_system(random_change_cell_type.run_if(resource_exists_and_equals(WillTick(true))))
            .add_system(random_change_cell_type.in_set(TurnSystemSet))
            ;
    }
}

#[derive(Component, Default)]
pub struct Cell{
    pub cell_type:CellType
}

#[derive(Default)]
pub enum CellType{
    #[default]
    Dead,
    Alive
}

fn setup(mut commands:Commands, asset_server:Res<AssetServer>){
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

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
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .insert(Cell::default())
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn set_texture_based_on_cell_type(mut query:Query<(&Cell, &mut TileTextureIndex)>){
    for (cell, mut tile_texture) in query.iter_mut() {
        // assets/tiles.png
        match cell.cell_type{
            CellType::Alive => tile_texture.0 = 5,
            CellType::Dead => tile_texture.0 = 4
        }
    }
}

fn random_change_cell_type(mut query: Query<&mut Cell>){
    let mut rng = rand::thread_rng();

    for mut cell in query.iter_mut(){
        match rng.gen_range(0..=1){
            0 => cell.cell_type = CellType::Dead,
            1 => cell.cell_type = CellType::Alive,
            _ => ()
        }
    }
}