mod engine;
mod examples;
use engine::{CellType, World};
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use std::time;

const CONTROLS_PANE_HEIGHT: usize = 40; // in pixels
const WORLD_PANE_SIZE: usize = 512; // in pixels (width and height)
const WINDOW_WIDTH: usize = WORLD_PANE_SIZE; // in pixels
const WINDOW_HEIGHT: usize = CONTROLS_PANE_HEIGHT + WORLD_PANE_SIZE; // in pixels
const TIME_STEP: f32 = 0.02; // in seconds

// Launch window
fn window_conf() -> Conf {
    println!("Launching window...");
    Conf {
        window_title: "Rust Fall".to_owned(),
        window_height: WINDOW_HEIGHT as i32,
        window_width: WINDOW_WIDTH as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut insert: bool;
    let mut world = populate_world();
    println!("World initialised");
    let mut pixels_per_cell: f32 = WORLD_PANE_SIZE as f32 / world.world_size as f32;
    let mut remainder_ms: i32 = 0;
    let mut counter: usize = 0;
    let mut paused: bool = false;
    let mut material_type: CellType = CellType::Sand;
    let time_step_ms: usize = (TIME_STEP * 1000.0) as usize;
    let start_time = time::Instant::now();
    let mut time_of_last_update = start_time.elapsed().as_millis();
    // Initiate loop
    loop {
        // Draw world
        draw(&world, pixels_per_cell);
        // Draw UI
        if root_ui().button(Vec2::new(10.0, 10.), "Pause/resume") {
            paused = !paused;
            println!("Simulation {}", if paused { "paused" } else { "resumed" });
            time_of_last_update = start_time.elapsed().as_millis();
        }
        if root_ui().button(Vec2::new(108.0, 10.0), "Step") {
            if paused {
                world.update();
                draw(&world, pixels_per_cell);
                println!("Single step performed");
            }
        }
        if root_ui().button(Vec2::new(150.0, 10.), "Reset") {
            world = populate_world();
            pixels_per_cell = WORLD_PANE_SIZE as f32 / world.world_size as f32;
            println!("Simulation reset");
        }
        if root_ui().button(
            Vec2::new(199.0, 10.0),
            match material_type {
                CellType::Empty => "Empty",
                CellType::Sand => "Sand",
                CellType::Water => "Water",
                CellType::Wall => "Wall",
                CellType::Steam => "Steam",
            },
        ) {
            material_type = toggle(material_type);
            println!("Material toggled");
        }
        // Button to print current world state to the console - not currently used
        //if root_ui().button(Vec2::new(250.0, 10.0), "Export") {
        //    print_world(&world);
        //}
        if is_mouse_button_down(MouseButton::Left) {
            insert = true;
        } else {
            insert = false;
        }
        // Update world
        if insert {
            let mut pos: (f32, f32) = mouse_position();
            pos.0 = clamp_number(pos.0, 0.0, (WORLD_PANE_SIZE - 1) as f32);
            pos.1 = clamp_number(
                pos.1,
                0.0,
                (CONTROLS_PANE_HEIGHT + WORLD_PANE_SIZE - 1) as f32,
            );
            if pos.1 > CONTROLS_PANE_HEIGHT as f32 {
                let x = (pos.0 / pixels_per_cell).floor() as usize;
                let mut y = (world.world_size as f32
                    - ((pos.1 - CONTROLS_PANE_HEIGHT as f32) / pixels_per_cell))
                    .floor() as usize;
                if y >= world.world_size as usize {
                    y = (world.world_size as f32 - 1.0) as usize;
                }
                world.grid.cells[y as usize][x as usize].cell_type = material_type;
            }
        }
        if !paused {
            remainder_ms += (start_time.elapsed().as_millis() - time_of_last_update) as i32;
            // This 'keep up' updates the world without rendering as many times as necessary to keep up with the target time step
            while remainder_ms > time_step_ms as i32 {
                world.update();
                time_of_last_update = start_time.elapsed().as_millis();
                remainder_ms -= time_step_ms as i32;
                counter += 1;
                // Break out of the 'keep up' loop after 10 consecutive iterations to ensure application responsiveness (simulation not keeping up with target time step)
                if counter >= 10 {
                    counter = 0;
                    break;
                }
            }
        }
        // Wait for next frame
        next_frame().await
    }
}

fn draw(world: &World, pixels_per_cell: f32) {
    // Draw controls pane background
    draw_rectangle(
        0.0,
        0.0,
        WORLD_PANE_SIZE as f32,
        CONTROLS_PANE_HEIGHT as f32,
        WHITE,
    );
    // Draw world background
    draw_rectangle(
        0.0,
        CONTROLS_PANE_HEIGHT as f32,
        WORLD_PANE_SIZE as f32,
        WORLD_PANE_SIZE as f32,
        GRAY,
    );
    // Draw world
    let mut cell_type: CellType;
    for row in 0..world.grid.rows {
        for column in 0..world.grid.columns {
            cell_type = world.grid.cells[row][column].cell_type;
            if world.grid.cells[row][column].cell_type != CellType::Empty {
                draw_rectangle(
                    column as f32 * pixels_per_cell,
                    CONTROLS_PANE_HEIGHT as f32 + WORLD_PANE_SIZE as f32
                        - ((row as f32) * pixels_per_cell + pixels_per_cell),
                    pixels_per_cell,
                    pixels_per_cell,
                    if cell_type == CellType::Wall {
                        BLACK
                    } else if cell_type == CellType::Sand {
                        BROWN
                    } else if cell_type == CellType::Water {
                        if row < (world.grid.rows - 1) {
                            if world.grid.cells[row + 1][column].cell_type == CellType::Empty {
                                SKYBLUE
                            } else {
                                BLUE
                            }
                        } else {
                            SKYBLUE
                        }
                    } else if cell_type == CellType::Steam {
                        LIGHTGRAY
                    } else {
                        panic!("Invalid cell type");
                    },
                )
            };
        }
    }
    // Draw FPS and step count
    draw_text(&format!("FPS: {}", get_fps()), 280.0, 25.0, 20.0, DARKGRAY);
    draw_text(
        &format!("STEPS: {}", &world.steps),
        370.0,
        25.0,
        20.0,
        DARKGRAY,
    );
}

fn toggle(current_type: CellType) -> CellType {
    match current_type {
        CellType::Empty => CellType::Wall,
        CellType::Wall => CellType::Sand,
        CellType::Sand => CellType::Water,
        CellType::Water => CellType::Steam,
        CellType::Steam => CellType::Empty,
    }
}

fn clamp_number<T>(value: T, lower: T, upper: T) -> T
where
    T: PartialOrd,
{
    if value > upper {
        upper
    } else if value < lower {
        lower
    } else {
        value
    }
}

fn populate_world() -> World {
    let mut world = World::new(examples::EXAMPLE_1.0);
    for row in 0..world.grid.rows {
        for column in 0..world.grid.columns {
            world.grid.cells[row][column].cell_type = match examples::EXAMPLE_1.1[row][column] {
                0 => CellType::Empty,
                1 => CellType::Wall,
                2 => CellType::Sand,
                3 => CellType::Water,
                4 => CellType::Steam,
                _ => CellType::Empty,
            };
        }
    }
    world
}

// Prints the current world to the console - not currently used
// fn print_world(world: &World) {
//     let mut cells: Vec<Vec<u8>> = vec![vec![0; world.world_size]; world.world_size];
//     for row in 0..world.grid.rows {
//         for column in 0..world.grid.columns {
//             cells[row][column] = world.grid.cells[row][column].cell_type as u8;
//         }
//     }
//     let world_state: (usize, Vec<Vec<u8>>) = (world.world_size, cells);
//     println!("Cells: {:?}", world_state);
// }
