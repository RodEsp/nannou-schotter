use nannou::{
    prelude::*,
    rand::{rngs::StdRng, Rng, SeedableRng},
};
use nannou_egui::{
    self,
    egui::{self, Align2},
    Egui,
};

const COLS: u32 = 12;
const LINE_WIDTH: f32 = 0.06;
const MARGIN: u32 = 35;
const ROWS: u32 = 22;
const SIZE: u32 = 30;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 75 + 2 * MARGIN;

struct Model {
    ui: Egui,
    main_window: WindowId,
    disp_adj: f32,
    rot_adj: f32,
    gravel: Vec<Stone>,
    random_seed: u64,
}

struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    final_x: f32,
    final_y: f32,
    final_rot: f32,
}

impl Stone {
    fn new(final_x: f32, final_y: f32) -> Self {
        Stone {
            x: 0.0,
            y: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
            final_x,
            final_y,
            final_rot: 0.0,
        }
    }
}

fn main() {
    nannou::app(setup)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

fn setup(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            let stone = Stone::new(x as f32, y as f32);
            gravel.push(stone);
        }
    }

    Model {
        main_window,
        gravel,
        ui: Egui::from_window(&app.window(main_window).unwrap()),
        disp_adj: 1.0,
        rot_adj: 1.0,
        random_seed: random_range(0, 1_000_000),
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    // Draw control panel
    let ctx = model.ui.begin_frame();

    egui::Window::new("Schotter Control Panel") // Control panel title
        .anchor(Align2::CENTER_TOP, [0.0, 1.0])
        .collapsible(true)
        .show(&ctx, |ui| {
            // Displacement slider
            ui.add(egui::Slider::new(&mut model.disp_adj, 0.0..=5.0).text("Displacement Factor"));
            // Rotation slider
            ui.add(egui::Slider::new(&mut model.rot_adj, 0.0..=5.0).text("Rotation Factor"));
            // Randomizer
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Randomize")).clicked() {
                    model.random_seed = random_range(0, 1000000);

                    let mut gravel = Vec::new();
                    for y in 0..ROWS {
                        for x in 0..COLS {
                            let stone = Stone::new(x as f32, y as f32);
                            gravel.push(stone);
                        }
                    }
                    model.gravel = gravel;
                }
                ui.add_space(20.0);
                ui.add(egui::DragValue::new(&mut model.random_seed));
                ui.label("Seed");
            });
        });
    // End control panel

    let mut rng = StdRng::seed_from_u64(model.random_seed);

    // Set current positions for each stone
    for stone in &mut model.gravel {
        let factor = stone.y / ROWS as f32;
        let disp_factor = factor * model.disp_adj;
        let rot_factor = factor * model.rot_adj;
        stone.x_offset = disp_factor * rng.gen_range(-0.5..0.5);
        stone.y_offset = disp_factor * rng.gen_range(-0.5..0.5);
        stone.final_rot = rot_factor * rng.gen_range(-PI / 4.0..PI / 4.0);
        if stone.x < stone.final_x {
            stone.x += 0.5 * (COLS as f32 / ROWS as f32);
        }
        if stone.y < stone.final_y {
            stone.y += 0.5;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 1.8);

    draw.background().color(WHITESMOKE);

    for stone in &model.gravel {
        let cdraw = gdraw.x_y(stone.x, stone.y);
        cdraw
            .rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x_offset, stone.y_offset)
            .rotate(stone.final_rot);
    }

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, 1000000);
        }
        Key::S => match app.window(model.main_window) {
            Some(window) => {
                window.capture_frame(app.exe_name().unwrap() + &app.time.to_string() + ".png");
            }
            None => {}
        },
        Key::Up => {
            model.disp_adj += 0.1;
        }
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => {
            model.rot_adj += 0.1;
        }
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _other_key => {}
    }
}
