use bevy::{
    color::palettes::css::*,
    prelude::*
};

use crate::gameplay::{
    physics::Rigidbody,
    spatial_hash_grid::SpatialHashGrid,
};

pub fn ray_get_collision_points_in_range(
    gizmos: &mut Gizmos,
    grid: &SpatialHashGrid,
    origin: Vec2,
    direction: Vec2,
    range: f32,
    query: &Query<(Entity, &Transform, &Rigidbody)>,
) -> Vec<Vec2> {
    // Calculate ray general line form
    let ray_a = direction.y;
    let ray_b = -direction.x;
    let ray_c = direction.x * origin.y - direction.y * origin.x;

    // Get walls in range
    let entities = grid.get_entities_in_range(origin, Vec2::ZERO, range);
    let walls: Vec<(Vec2, Vec2)> = entities
        .into_iter()
        .map(|e| { 
            let Ok((_, t, rb)) = query.get(e) else { todo!() };

            // Change origin from center to bottom left
            let start = t.translation.truncate() - rb.size / 2.0;

            // Calculate rectangle points
            //  d *----------* c
            //    |          |
            //    |          |
            //  a *----------* b
            let a = start;
            let b = start + Vec2::new(rb.size.x, 0.0);
            let c = start + rb.size;
            let d = start + Vec2::new(0.0, rb.size.y);

            // Output the lines of the rectangle
            vec![
                (a, b),
                (b, c),
                (c, d),
                (d, a),
            ]
        })
        .into_iter()
        .flatten()
        .collect();


    walls
        .into_iter()
        .filter_map(|wall| {
            // The formula uses general form, so we need to convert our line to it.
            let wall_a = wall.0.y - wall.1.y;
            let wall_b = wall.1.x - wall.0.x;
            let wall_c = wall.0.x * wall.1.y - wall.1.x * wall.0.y;

            gizmos.line_2d(wall.0, wall.1, GREEN);

            // Calculate the intersection
            let denominator = ray_a * wall_b - wall_a * ray_b;
            
            if denominator.abs() < 1e-5 {
                return None; // Lines will never cross (almost parallel)
            }

            let x = (ray_b * wall_c - wall_b * ray_c) / denominator;
            let y = (ray_c * wall_a - wall_c * ray_a) / denominator;
            let hit = Vec2::new(x, y);


            // Check if intersection is within wall
            let wall_len = wall.1 - wall.0;
            let wall_t = (hit - wall.0).dot(wall_len) / wall_len.length_squared();
            
            if !(0.0..=1.0).contains(&wall_t) {
                return None; // Hit was outside of the wall
            }

            gizmos.circle_2d(hit, 1.0, RED);

            // Check if intersection is within range
            // TODO
            Some(hit)
        })
        .collect()
}