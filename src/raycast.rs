use std::cmp::max;

/// Performs a raycast in a 2D grid map, returning the distance to the first encountered obstacle.
///
/// # Parameters:
/// - `start_x`: The starting x-coordinate of the ray.
/// - `start_y`: The starting y-coordinate of the ray.
/// - `theta`: The angle of the ray in radians.
/// - `step_increment`: The increment for each step of the ray.
/// - `max_distance`: The maximum distance the ray can travel.
/// - `map_width`: The width of the 2D grid map.
/// - `map_height`: The height of the 2D grid map.
/// - `map`: A reference to the 2D grid map, represented as a linear array.
///
/// # Returns:
/// The distance to the first obstacle hit by the ray, or the maximum distance if no obstacle is encountered.
pub fn slow_raycast(
    start_x: f32,
    start_y: f32,
    theta: f32,
    step_increment: f32,
    map_width: usize,
    map_height: usize,
    map: &[u8],
) -> f32 {
    let mut distance = 0.0;
    let (mut x, mut y) = (start_x, start_y);
    let (dx, dy) = (theta.cos() * step_increment, theta.sin() * step_increment);

    while distance < max(map_width, map_height) as f32 {
        let grid_x = x as usize;
        let grid_y = y as usize;

        if grid_x >= map_width || grid_y >= map_height {
            break;
        }
        if map[grid_x + grid_y * map_width] == 1 {
            return distance;
        }

        x += dx;
        y += dy;
        distance += step_increment;
    }

    distance
}

/// Performs a fast raycast in a 2D grid map to find the distance to the first obstacle from a given starting point.
///
/// # Arguments
///
/// * `start_x` - The x-coordinate of the starting point of the ray.
/// * `start_y` - The y-coordinate of the starting point of the ray.
/// * `theta` - The angle (in radians) of the ray direction from the positive x-axis.
/// * `map_width` - The width of the map.
/// * `map_height` - The height of the map.
/// - `map`: A reference to the 2D grid map, represented as a linear array.
///
/// # Returns
///
/// The function returns the distance to the first obstacle encountered in the direction of `theta`. If no obstacle is encountered within the map bounds, `f32::MAX` is returned.
pub fn fast_raycast(
    start_x: f32,
    start_y: f32,
    theta: f32,
    map_width: usize,
    map_height: usize,
    map: &[u8],
) -> (f32, f32) {
    let mut map_x = start_x as isize;
    let mut map_y = start_y as isize;

    let dir_x = theta.cos();
    let dir_y = theta.sin();

    let delta_dist_x = (1.0 / dir_x).abs().max(f32::MIN_POSITIVE);
    let delta_dist_y = (1.0 / dir_y).abs().max(f32::MIN_POSITIVE);

    let step_x = if dir_x >= 0.0 { 1 } else { -1 };
    let step_y = if dir_y >= 0.0 { 1 } else { -1 };

    let mut ray_x = match dir_x.signum() as isize {
        1 => (1.0 - start_x.fract()) * delta_dist_x,
        -1 => start_x.fract() * delta_dist_x,
        _ => f32::MAX,
    };
    let mut ray_y = match dir_y.signum() as isize {
        1 => (1.0 - start_y.fract()) * delta_dist_y,
        -1 => start_y.fract() * delta_dist_y,
        _ => f32::MAX,
    };

    let mut side;
    while map_x >= 0 && map_x < map_width as isize && map_y >= 0 && map_y < map_height as isize {
        if ray_x < ray_y {
            ray_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            ray_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }

        if map_x < 0 || map_x >= map_width as isize || map_y < 0 || map_y >= map_height as isize {
            break;
        }

        if map[(map_x as usize) + (map_y as usize) * map_width] == 1 {
            if side == 0 {
                let perp_wall_dis = ray_x - delta_dist_x;
                return (perp_wall_dis, (start_y + perp_wall_dis * dir_y).fract());
            } else {
                let perp_wall_dis = ray_y - delta_dist_y;
                return (perp_wall_dis, (start_x + perp_wall_dis * dir_x).fract());
            }
        }
    }

    (f32::MAX, f32::MAX)
}

