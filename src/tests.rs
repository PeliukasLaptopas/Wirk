use std::rc::Rc;
use crate::rendering::sprite::sprite_batch::SpriteBatch;
use crate::rendering::sprite::Sprite;
use nalgebra::Vector2;
use wrapped2d::dynamics::body::BodyType::Static;
use crate::rendering::sprite::rigid_body_2d::ColliderType;
use wrapped2d::b2::World;
use wrapped2d::user_data::NoUserData;
use wrapped2d::b2;
use crate::rendering::texture::Texture;
use crate::rendering::camera_2d::Camera2D;
use crate::resources::Resources;
use crate::rendering::shader::program::Program;
use std::path::Path;
use crate::CustomUserData;


#[test]
fn test_add() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut time_subsystem = sdl.timer().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_double_buffer(true); //i think its on by default
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Alesis Zaidimas", 100, 100)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = Rc::new(
        gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void)
    );

    let mut resources = Resources::from_relative_path(Path::new("assets")).unwrap();
    let mut program = Program::from_res(&gl, &mut resources, "shaders/triangle").unwrap();

    let mut sprite_batch = SpriteBatch::new(&gl);

    let mut world = b2::World::<CustomUserData>::new(&b2::Vec2{x:0.0, y:0.0});

    let new_sprite = Sprite::new(Vector2::new(0.0, 0.0), &Static, ColliderType::Circle(1.0), (1.0, 1.0, 1.0, 1.0).into(), &mut world, &Texture{
        id: 0,
        width: 0,
        height: 0
    }).unwrap();

    let mut camera_pos = Vector2::new(0.0, 0.0);
    let camera_scale = 32.0; //32 pixels per meter
    let mut camera = Camera2D::new(camera_pos, camera_scale, 100, 100);

    sprite_batch.begin();
    assert_eq!(true, sprite_batch.glyphs.len() != 0);
    assert_eq!(true, sprite_batch.render_batches.len() != 0);
    new_sprite.draw(&mut world, &mut sprite_batch, 0.0);
    assert_eq!(true, sprite_batch.render_batches.len() != 0);
    sprite_batch.end();
    assert_eq!(true, sprite_batch.render_batches.len() != 0);
    sprite_batch.render_batch(&mut camera, &mut program, &gl);
}
