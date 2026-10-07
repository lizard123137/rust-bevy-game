use bevy::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
};

use crate::GameState;
use crate::gameplay::{
    player::Frog,
    physics::Rigidbody,
};

const CELL_SIZE: f32 = 64.0;

#[derive(Resource)]
pub struct SpatialHashGrid {
    pub cells: HashMap<IVec2, HashSet<Entity>>,
    pub entities: HashMap<Entity, HashSet<IVec2>>,
}

impl Default for SpatialHashGrid {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            entities: HashMap::new(),
        }
    }
}

impl SpatialHashGrid {
    fn insert(
        &mut self,
        entity: Entity,
        pos: Vec2,
        size: Vec2
    ) {
        let cells = calculate_cells(pos, size);

        for cell in cells {
            // Add entity to cells
            self.cells
                .entry(cell)
                .or_default()
                .insert(entity);
        
            // Add cells to entity
            self.entities
                .entry(entity)
                .or_default()
                .insert(cell);
        }
    }

    fn update(
        &mut self,
        entity: Entity,
        pos: Vec2,
        size: Vec2
    ) {
        let cells = calculate_cells(pos, size);
        let old_cells = self.entities
            .entry(entity)
            .or_default()
            .clone();

        // Remove old position
        for cell in &old_cells {
            if cells.contains(&cell) {
                continue;
            }
            
            // Remove cell from entity
            self.entities
                .entry(entity)
                .or_default()
                .remove(&cell);

            // Remove entity from cell
            self.cells
                .entry(*cell)
                .or_default()
                .remove(&entity);
        }

        // Add new positions
        for cell in cells {
            if old_cells.contains(&cell) {
                continue;
            }
            
            // Add cell to entity
            self.entities
                .entry(entity)
                .or_default()
                .insert(cell);

            // Add entity to cell
            self.cells
                .entry(cell)
                .or_default()
                .insert(entity);
        }
    }

    pub fn get_entities_in_range(
        &mut self,
        pos: Vec2,
        size: Vec2,
        range: f32,
    ) -> HashSet<Entity> {
        let cells = self.get_cells_in_range(pos, size, range);

        let mut entities_in_range = HashSet::new();
        for cell in cells {
            for &entity in self.cells.get(&cell).into_iter().flatten() {
                entities_in_range.insert(entity);
            }
        }

        entities_in_range
    }


    pub fn get_cells_in_range(
        &self,
        pos: Vec2,
        size: Vec2,
        range: f32,
    ) -> HashSet<IVec2> {
        // Calculate Diagonal
        let range_full = size.x.max(size.y) + range;
        
        // We can't iterate over f32 ranges :(
        const SCALE: f32 = 256.0; 
        let x_range = (range_full * (PI / 4.0).cos() * SCALE).floor() as i32;
        let x_start = -x_range + (pos.x * SCALE).floor() as i32;
        let x_end = x_range + (pos.x * SCALE).floor() as i32;
        let y_range = (range_full * (PI / 4.0).sin() * SCALE).floor() as i32;
        let y_start = -y_range + (pos.y * SCALE).floor() as i32;
        let y_end = y_range + (pos.y * SCALE).floor() as i32;
        const STEP: usize = (CELL_SIZE / 2.0 * SCALE) as usize;

        // Iterate through rectangle with diagonal of size <range>
        let mut cells_in_range = HashSet::new();

        for x in (x_start..=x_end).step_by(STEP).map(|i| i as f32 / SCALE) {
            for y in (-y_start..=y_end).step_by(STEP).map(|i| i as f32 / SCALE) {
                cells_in_range.insert(IVec2::new(
                    (x / CELL_SIZE).floor() as i32,
                    (y / CELL_SIZE).floor() as i32
                ));
            }
        }

        cells_in_range
    }
}

pub struct SpatialHashGridPlugin;

impl Plugin for SpatialHashGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialHashGrid>();
        app.add_systems(OnEnter(GameState::Game),
            initialize_spatial_hash_grid
        );
        app.add_systems(FixedUpdate, (
            update_spatial_hash_grid.run_if(in_state(GameState::Game)),
            // _debug_gizmos,
            _debug_gizmos_player,
        ));
    }
}

fn initialize_spatial_hash_grid(
    mut grid: ResMut<SpatialHashGrid>,
    query: Query<(Entity, &Transform, &Rigidbody)>,
) {
    for (entity, transform, rb) in &query {
        grid.insert(entity, transform.translation.truncate(), rb.size);
    }
}

fn update_spatial_hash_grid(
    mut grid: ResMut<SpatialHashGrid>,
    query: Query<(Entity, &Transform, &Rigidbody)>,
    added: Query<(Entity, &Transform, &Rigidbody), Added<Rigidbody>>
) {
    for (entity, transform, rb) in &added {
        grid.insert(entity, transform.translation.truncate(), rb.size);
    }
    
    for (entity, transform, rb) in &query {
        if !rb.moveable {
            continue;
        }

        grid.update(entity, transform.translation.truncate(), rb.size);
    }
}

fn _debug_gizmos(
    mut gizmos: Gizmos,
    grid: Res<SpatialHashGrid>,
    query: Query<(Entity, &Rigidbody)>,
) {
    for (entity, _rb) in &query {
        if let Some(cells) = grid.entities.get(&entity) {
            for cell in cells {
                let center = Vec2::new(
                    cell.x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                    cell.y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                );

                gizmos.rect_2d(
                    center,
                    Vec2::splat(CELL_SIZE),
                    Color::srgb(0.2, 0.8, 0.2),
                );
            }
        }
    }
}

fn _debug_gizmos_player(
    mut gizmos: Gizmos,
    grid: Res<SpatialHashGrid>,
    frog: Single<(&Transform, &Rigidbody), With<Frog>>,
) {
    let (t, rb) = frog.into_inner();

    // Range the size of one cell
    let cells = grid.get_cells_in_range(t.translation.truncate(), rb.size, CELL_SIZE);

    for cell in cells {
        let center = Vec2::new(
            cell.x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            cell.y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
        );

        gizmos.rect_2d(
            center,
            Vec2::splat(CELL_SIZE),
            Color::srgb(0.8, 0.2, 0.2),
        );
    }
}

fn calculate_cells(position: Vec2, size: Vec2) -> HashSet<IVec2> {
    let mut result = HashSet::new();

    let start = position - size / 2.0;
    let end = start + size;

    let min_cell = (start / CELL_SIZE).floor().as_ivec2();
    let max_cell = (end / CELL_SIZE).floor().as_ivec2();

    for x in min_cell.x..=max_cell.x {
        for y in min_cell.y..=max_cell.y {
            result.insert(IVec2::new(x, y));
        }
    }

    result
}
