use std::f32::consts::FRAC_2_PI;
use std::mem::MaybeUninit;

use actor::Actor;
use camera::Camera;
use gamercade_rs::api::graphics_parameters::GraphicsParameters;

use gamercade_rs::prelude as gc;
use glam::{Mat4, Vec3};
use graphics::Gpu;
use graphics::{GraphicsDb, IndexList, Mesh, ParameterData, VertexList};
use shaders::PixelShaderInput;

mod actor;
mod camera;
mod graphics;
mod image;
mod math;
mod shaders;
mod shapes;
mod types;

static mut GAME_STATE: MaybeUninit<GameState> = MaybeUninit::uninit();
static mut CAMERA: MaybeUninit<Camera> = MaybeUninit::uninit();
static mut GPU: MaybeUninit<Gpu> = MaybeUninit::uninit();
static mut GRAPHICS_DB: MaybeUninit<GraphicsDb> = MaybeUninit::uninit();

pub struct GameState {
    actors: Vec<Actor<2>>,
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn init() {
    let screen_width = gc::width();
    let screen_height = gc::height();
    let mut graphics_db = GraphicsDb::default();

    let mut vertices = Vec::new();
    let mut parameters = Vec::new();

    // One parameter for each vertex
    shapes::plane(5.0)
        .into_iter()
        .enumerate()
        .for_each(|(i, x)| {
            //let color = shapes::CUBE_COLORS[i % shapes::CUBE_COLORS.len()];
            let uv = shapes::CUBE_UVS[i];
            vertices.push(x);
            parameters.push(PixelShaderInput(uv));
        });

    let indices = IndexList(
        shapes::TRI_INDICES
            //shapes::PLANE_INDICES
            //shapes::CUBE_INDICES
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let actor_id = graphics_db.push_mesh(Mesh {
        vertices: VertexList(vertices.into_boxed_slice()),
        indices,
        parameters: ParameterData(parameters.into_boxed_slice()),
    });

    let positions = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.5, -2.5, 0.0),
        Vec3::new(-2.5, 2.5, 0.0),
    ];

    let mut actors = Vec::new();

    let mut delta = 0.0;
    positions.into_iter().for_each(|position| {
        actors.push(Actor {
            mesh_id: actor_id,
            transform: Mat4::from_translation(position),
            delta,
        });

        delta += FRAC_2_PI;
    });

    GAME_STATE.write(GameState { actors });
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
    //game_state.actors.iter_mut().for_each(|a| a.update());

    // console_log(&format!(
    //     "pos: {}, for: {}, rig {}",
    //     camera.position,
    //     camera.view.forward_vector(),
    //     camera.view.right_vector(),
    // ));
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn draw() {
    let gpu = GPU.assume_init_mut();
    let graphics = GRAPHICS_DB.assume_init_ref();
    let game_state = GAME_STATE.assume_init_ref();

    // Clear all of the buffers
    gpu.clear_z_buffer();
    gc::clear_screen(GraphicsParameters::default());

    gpu.render_actor(&game_state.actors[0], &graphics.textured);
    //gpu.render_actor(&game_state.actors[1], &graphics.textured);
    //gpu.render_actor(&game_state.actors[2], &graphics.textured);
}
