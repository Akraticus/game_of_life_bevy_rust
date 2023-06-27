use std::collections::HashMap;

use bevy::{prelude::*};
use bevy_ecs_tilemap::{prelude::*, helpers::square_grid::neighbors::Neighbors};
use rand::prelude::*;

use crate::{tick_plugin::TurnSystemSet};

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup)
            .add_system(set_texture_based_on_cell_type)
            .add_system(iterate_board_state.in_set(TurnSystemSet))
            ;
    }
}

#[derive(Component, Default)]
pub struct Cell{
    pub cell_state:CellState
}

#[derive(Default)]
pub enum CellState{
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
    
    let mut rng = ThreadRng::default();
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .insert(Cell{
                    cell_state: match rng.gen_bool(0.5){
                        false => CellState::Dead,
                        true => CellState::Alive
                    }
                })
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
        match cell.cell_state{
            CellState::Alive => tile_texture.0 = 5,
            CellState::Dead => tile_texture.0 = 4
        }
    }
}

fn iterate_board_state(mut tile_map_query:Query<(&TileStorage, &TilemapSize)>, mut tile_query: Query<&mut Cell>){    
    let mut next_states = HashMap::new();

    for (tile_storage, tile_map_size) in tile_map_query.iter_mut(){
        for x in 0..tile_map_size.x{
            for y in 0..tile_map_size.y{
                let tile_pos = TilePos { x, y };
                let tile_entity = match tile_storage.get(&tile_pos){
                    Some(v) => v,
                    None => continue
                };

                let current_cell = match tile_query.get(tile_entity){
                    Ok(cell) => cell,
                    Err(e) => continue
                };

                let neighbours = Neighbors::get_square_neighboring_positions(&tile_pos, tile_map_size, true).entities(tile_storage);
                let mut alive_neighbours = 0;
                for entity in neighbours.iter() {
                    if let Ok(mut cell) = tile_query.get(*entity){
                        alive_neighbours += match cell.cell_state{
                            CellState::Alive =>  1,
                            CellState::Dead => 0
                        };
                    }
                }

                
                let next_state = match current_cell.cell_state {
                     CellState::Alive => {
                        match alive_neighbours{
                            ..=1 => CellState::Dead,
                            2..=3 => CellState::Alive,
                            4.. => CellState::Dead
                        }
                     },
                     CellState::Dead => {
                        match alive_neighbours{
                            3 => CellState::Alive,
                            _ => CellState::Dead
                        }
                     }
                };

                next_states.insert(tile_entity, next_state);
            }
        }
    }

    // apply all the new states to all entities
    for next_state in next_states {
        if let Ok(mut cell) = tile_query.get_mut(next_state.0){
            cell.cell_state = next_state.1;
        }
    }
}