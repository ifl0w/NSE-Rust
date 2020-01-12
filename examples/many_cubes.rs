use cgmath::{Angle, Deg, Euler, Matrix4, Quaternion, Vector3, Rad, Vector4, Transform, Matrix3};
use winit::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::dpi::LogicalPosition;
use winit::WindowEvent::{
    KeyboardInput,
    CursorMoved
};

use nse;
use nse::core::{Entity, System, Filter};
use nse::NSE;
use nse::rendering::{Camera, Cube, Mesh, RenderSystem, Transformation};
use std::sync::{Mutex, Arc};
use std::time::Duration;

mod shared;

use crate::shared::fps_camera_system::FPSCameraSystem;

fn main() {
    let mut engine: NSE = NSE::new();

    let render_system = RenderSystem::new(&engine);
    let fps_camera_system = FPSCameraSystem::new();

    engine.system_manager.add_system(render_system.clone());
    engine.system_manager.add_system(fps_camera_system.clone());

    // add camera
    let camera = Entity::new();
    camera.lock().unwrap()
        .add_component(Camera::new(0.1, 1000.0, 90.0, [800.0, 600.0]))
        .add_component(Transformation {
            position: Vector3::new(0.0, 0.0, 3.0),
            ..Default::default()
        });
    engine.entity_manager.add_entity(camera);

    // add cubes
    let cube_mesh = Mesh::new::<Cube>(&render_system.lock().unwrap());

    let num_cubes = 0..50;
    let offset = Vector3::new(-num_cubes.end as f32, -num_cubes.end as f32, -num_cubes.end as f32 * 4.0);
    for x in num_cubes.clone() {
        for y in num_cubes.clone() {
            for z in num_cubes.clone() {
                let mut position = Vector3::new(x as f32 * 2.0, y as f32 * 2.0, z as f32 * 2.0);
                position += offset;

                let mut entity = Entity::new();
                entity.lock().unwrap()
                    .add_component(cube_mesh.clone())
                    .add_component(Transformation {
                        position,
                        ..Default::default()
                    });
                engine.entity_manager.add_entity(entity);
            }
        }
    }

    engine.run();
}