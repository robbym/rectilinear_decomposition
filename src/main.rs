use macroquad::prelude::*;

use ::rand::Rng;

use rectilinear_decomposition::{generate_random_horizontally_convex_polygon, RectilinearPolygon};

fn window_conf() -> Conf {
    Conf {
        window_title: "Rectilinear Decomposition".to_owned(),
        window_width: 1920,
        window_height: 1440,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut seed = [0u8; 32];

    let height = 30;

    let vertices = generate_random_horizontally_convex_polygon(seed, height, 1);
    // let vertices = [
    //     (0, 0),
    //     (10, 0),
    //     (10, 2),
    //     (9, 2),
    //     (9, 3),
    //     (10, 3),
    //     (10, 4),
    //     (9, 4),
    //     (9, 5),
    //     (10, 5),
    //     (10, 10),
    //     (0, 10),
    // ];
    let mut rp = RectilinearPolygon::from_vertices(&vertices);
    rp.find_vchords();

    // Assert that all adjacent vertices in the boundary have either the same x or y coordinate
    for i in 0..rp.boundary.len() {
        let v1 = &rp.vertices[rp.boundary[i]];
        let v2 = &rp.vertices[rp.boundary[(i + 1) % rp.boundary.len()]];

        debug_assert!(v1.x == v2.x || v1.y == v2.y);
    }

    // Scale should decrease as height increases
    let scale = 1000.0 / height as f32;

    loop {
        clear_background(BLACK);

        let min_x = rp.min_x as f32 * scale;
        let max_x = rp.max_x as f32 * scale;
        let min_y = rp.min_y as f32 * scale;
        let max_y = rp.max_y as f32 * scale;

        // Get screen dimensions
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Center the contour
        let x_offset = (screen_width - (max_x - min_x)) / 2.0 - min_x;
        let y_offset = (screen_height - (max_y - min_y)) / 2.0 - min_y;

        // Get mouse position relative to the grid
        let mouse_x = mouse_position().0 - x_offset;
        let mouse_y = mouse_position().1 - y_offset;

        // Draw grid
        for y in rp.min_y..=rp.max_y {
            draw_line(
                rp.min_x as f32 * scale + x_offset,
                y as f32 * scale + y_offset,
                rp.max_x as f32 * scale + x_offset,
                y as f32 * scale + y_offset,
                1.0,
                DARKGRAY,
            );

            for x in rp.min_x..=rp.max_x {
                draw_line(
                    x as f32 * scale + x_offset,
                    rp.min_y as f32 * scale + y_offset,
                    x as f32 * scale + x_offset,
                    rp.max_y as f32 * scale + y_offset,
                    1.0,
                    DARKGRAY,
                );
            }
        }

        // Draw grid row and column highlights depending on mouse position
        let mouse_x = mouse_x / scale;
        let mouse_y = mouse_y / scale;

        let mouse_x = mouse_x.floor() as i64;
        let mouse_y = mouse_y.floor() as i64;

        draw_line(
            rp.min_x as f32 * scale + x_offset,
            mouse_y as f32 * scale + y_offset,
            rp.max_x as f32 * scale + x_offset,
            mouse_y as f32 * scale + y_offset,
            1.0,
            LIGHTGRAY,
        );

        draw_line(
            mouse_x as f32 * scale + x_offset,
            rp.min_y as f32 * scale + y_offset,
            mouse_x as f32 * scale + x_offset,
            rp.max_y as f32 * scale + y_offset,
            1.0,
            LIGHTGRAY,
        );

        // Draw contour
        for i in 0..rp.boundary.len() {
            let v1 = &rp.vertices[rp.boundary[i]];
            let v2 = &rp.vertices[rp.boundary[(i + 1) % rp.boundary.len()]];
            draw_line(
                v1.x as f32 * scale + x_offset,
                v1.y as f32 * scale + y_offset,
                v2.x as f32 * scale + x_offset,
                v2.y as f32 * scale + y_offset,
                2.0,
                WHITE,
            );
        }

        // Draw horizontal chords in boundary
        for &i in &rp.boundary {
            let v = &rp.vertices[i];
            if let Some(h_chord) = v.h_chord {
                let v2 = &rp.vertices[h_chord];
                draw_line(
                    v.x as f32 * scale + x_offset,
                    v.y as f32 * scale + y_offset,
                    v2.x as f32 * scale + x_offset,
                    v2.y as f32 * scale + y_offset,
                    1.0,
                    RED,
                );
            }
        }

        // Draw vertical chords in boundary
        for &i in &rp.boundary {
            let v = &rp.vertices[i];
            if let Some(v_chord) = v.v_chord {
                let v2 = &rp.vertices[v_chord];
                draw_line(
                    v.x as f32 * scale + x_offset,
                    v.y as f32 * scale + y_offset,
                    v2.x as f32 * scale + x_offset,
                    v2.y as f32 * scale + y_offset,
                    1.0,
                    GREEN,
                );
            }
        }

        // Check if mouse clicked
        if is_mouse_button_pressed(MouseButton::Left) {
            ::rand::thread_rng().fill(&mut seed);
            let vertices = generate_random_horizontally_convex_polygon(seed, height, 1);
            rp = RectilinearPolygon::from_vertices(&vertices);
            rp.find_vchords();
        } else if is_mouse_button_pressed(MouseButton::Right) {
            rp.make_cuts();
            rp.compute_boundary();
        }

        next_frame().await;
    }
}
