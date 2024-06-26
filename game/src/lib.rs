use std::mem::MaybeUninit;

use camera::Camera;

use gamercade_rs::prelude as gc;
use glam::Vec3A;
use graphics::Gpu;
use scenes::*;

mod actor;
mod animation;
mod camera;
pub mod generated;
mod graphics;
mod math;
mod scenes;
mod shaders;

static mut GAME_STATE: MaybeUninit<GameState> = MaybeUninit::uninit();
static mut CAMERA: MaybeUninit<Camera> = MaybeUninit::uninit();
static mut GPU: MaybeUninit<Gpu> = MaybeUninit::uninit();

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    scene_index: usize,
}
static mut DATA_PTR: *const u8 = std::ptr::null();
static mut DATA_LEN: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn datapack(len: i32) -> i32 {
    let leak: Vec<u8> = Vec::with_capacity(len as usize);
    let ptr = leak.as_ptr();

    DATA_PTR = ptr as *const u8;
    DATA_LEN = len as usize;

    std::mem::forget(leak);
    ptr as i32
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn init() {
    let screen_width = gc::width();
    let screen_height = gc::height();

    let text = std::ffi::CStr::from_ptr(DATA_PTR as *const i8)
        .to_str()
        .unwrap();
    gc::console_log(&format!("{text}"));

    let scenes: Vec<Box<dyn Scene>> = vec![
        (Box::new(VsScene::new())),
        (Box::new(MultimeshScene::new())),
        (Box::new(MechScene::new())),
        (Box::new(HelmetModelScene::new())),
        (Box::new(CubeModelScene::new())),
        (Box::new(FoxModelScene::new())),
        (Box::new(CubeScene::new())),
        (Box::new(CubesScene::new())),
        (Box::new(PlaneScene::new())),
        (Box::new(TriangleScene::new())),
    ];

    GAME_STATE.write(GameState {
        scenes,
        scene_index: 0,
    });
    CAMERA.write(Camera::new(
        Vec3A::new(0.0, 0.0, 5.0),
        screen_width as f32 / screen_height as f32,
    ));
    GPU.write(Gpu::new(screen_width, screen_height));
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn update() {
    let camera = CAMERA.assume_init_mut();
    let game_state = GAME_STATE.assume_init_mut();

    camera.update();

    if let Some(true) = gc::button_select_pressed(0) {
        game_state.scene_index += 1;
        game_state.scene_index %= game_state.scenes.len();
    } else if let Some(true) = gc::button_start_pressed(0) {
        if game_state.scene_index == 0 {
            game_state.scene_index = game_state.scenes.len();
        }
        game_state.scene_index -= 1;
    }

    game_state.scenes[game_state.scene_index].update();
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn draw() {
    let gpu = GPU.assume_init_mut();
    let game_state = GAME_STATE.assume_init_ref();

    // Clear all of the buffers
    gpu.reset_frame();

    // For Calculating MVP Later
    let camera = CAMERA.assume_init_ref();
    gpu.uniforms.projection = camera.projection;
    gpu.uniforms.view = camera.view;

    gpu.uniforms.light_position = camera.position;
    gpu.uniforms.light_intensity = 1.05;
    gpu.uniforms.ambient_light = 0.25;

    game_state.scenes[game_state.scene_index].draw(gpu);

    gc::write_pixel_buffer(0, gpu.generate_frame_buffer());
}
