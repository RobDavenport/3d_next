use std::mem::MaybeUninit;

use camera::Camera;

use gamercade_rs::prelude as gc;
use glam::Vec3;
use graphics::Gpu;
use graphics::GraphicsDb;
use scenes::CubeScene;
use scenes::{CubesScene, PlaneScene, Scene, TriangleScene};

mod actor;
mod assets;
mod camera;
mod graphics;
mod math;
mod scenes;
mod shaders;
mod shapes;
mod types;

static mut GAME_STATE: MaybeUninit<GameState> = MaybeUninit::uninit();
static mut CAMERA: MaybeUninit<Camera> = MaybeUninit::uninit();
static mut GPU: MaybeUninit<Gpu> = MaybeUninit::uninit();
static mut GRAPHICS_DB: MaybeUninit<GraphicsDb> = MaybeUninit::uninit();

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    scene_index: usize,
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn init() {
    let screen_width = gc::width();
    let screen_height = gc::height();
    let mut graphics_db = GraphicsDb::default();

    let scenes: Vec<Box<dyn Scene>> = vec![
        (Box::new(CubeScene::new(&mut graphics_db))),
        (Box::new(CubesScene::new(&mut graphics_db))),
        (Box::new(PlaneScene::new(&mut graphics_db))),
        (Box::new(TriangleScene::new(&mut graphics_db))),
    ];

    GAME_STATE.write(GameState {
        scenes,
        scene_index: 0,
    });
    CAMERA.write(Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        screen_width as f32 / screen_height as f32,
    ));
    GPU.write(Gpu::new(screen_width, screen_height));
    GRAPHICS_DB.write(graphics_db);
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn update() {
    let camera = CAMERA.assume_init_mut();
    let game_state = GAME_STATE.assume_init_mut();

    camera.update();

    if let Some(true) = gc::button_select_pressed(0) {
        game_state.scene_index = (game_state.scene_index + 1) % game_state.scenes.len();
    }

    game_state.scenes[game_state.scene_index].update();
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn draw() {
    let gpu = GPU.assume_init_mut();
    let graphics = GRAPHICS_DB.assume_init_mut();
    let game_state = GAME_STATE.assume_init_ref();

    // Clear all of the buffers
    gpu.clear_z_buffer();
    gpu.clear_frame_buffer();

    // For Calculating MVP Later
    let camera = CAMERA.assume_init_ref();
    graphics.base_vertex_shader.projection = camera.projection;
    graphics.base_vertex_shader.view = camera.view;

    graphics.color_blend_lit.light_position = camera.position;
    graphics.color_blend_lit.light_intensity = 1.25;
    graphics.color_blend_lit.ambient_light = 0.15;

    graphics.textured_lit.light_position = camera.position;
    graphics.textured_lit.light_intensity = 1.25;
    graphics.textured_lit.ambient_light = 0.15;

    graphics.textured_normal_lit.light_position = camera.position;
    graphics.textured_normal_lit.light_intensity = 1.25;
    graphics.textured_normal_lit.ambient_light = 0.15;

    game_state.scenes[game_state.scene_index].draw(gpu, graphics);

    gc::write_pixel_buffer(0, &gpu.frame_buffer);
}
