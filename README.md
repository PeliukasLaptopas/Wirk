I have downloaded SDL2 (SDL2-devel-2.0.14-VC.zip) from https://www.libsdl.org/download-2.0.php and added /x64/* files to C:\Users\*\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\ in order for SDL to work

Machine needs visual c++ installed. Just simply download visual studio 2019 or you will get this error while compiling:
"please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option"
Just download vs from here:
https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=16
and install c++ option

Box2D requiriments
if cmake is not installed you will get this error while compiling:
![image](https://user-images.githubusercontent.com/38985678/112358177-c328fd00-8cd8-11eb-9e83-629307c38f25.png)

download and install cmake-3.20.0-windows-x86_64.msi
![image](https://user-images.githubusercontent.com/38985678/112358303-e489e900-8cd8-11eb-9814-ec333b93d74c.png)


This is a project that uses my engine
![image](https://user-images.githubusercontent.com/38985678/111040399-7a04bd80-843b-11eb-9734-81dddbca835a.png)

![image](https://user-images.githubusercontent.com/38985678/113760946-8a007c00-971f-11eb-8b19-a2eba59a1b96.png)
![image](https://user-images.githubusercontent.com/38985678/111040417-8b4dca00-843b-11eb-895e-96c86b3a62dc.png)

Supports
Vsync
fps calculator
fps limiter

Ball game:
```
use RustEngineLibrary::*;
use RustEngineLibrary::engine_error::failure_to_string;
use wrapped2d::b2::World;
use wrapped2d::b2::Vec2;
use wrapped2d::user_data::NoUserData;
use wrapped2d::b2;
use wrapped2d::dynamics::body::BodyType::Kinematic;
use wrapped2d::dynamics::body::BodyType::Dynamic;
use std::path::Path;
use crate::rendering::sprite::rigid_body_2d::ColliderType;
use nalgebra::Vector2;
use rendering::sprite::sprite_batch::*;
use crate::rendering::sprite::Sprite;
use crate::rendering::{vertex, sprite};
use crate::resources::Resources;
use crate::rendering::camera_2d::*;
use crate::rendering::shader::program::Program;
use legion::*;
use crate::ecs::Ecs;
use legion::systems::SystemFn;
use rand::{thread_rng, Rng};
use crate::rendering::ui::text::{Text};

fn main() -> Result<(), failure::Error> {
    #[system(simple)]
    fn update_positions() {
        // println!("DEEZ System");
    }

    let mut ecs_resources = legion::Resources::default();
    let ecs_world = legion::World::default();

    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .build();

    let ecs = Ecs {
        schedule: schedule,
        resources: ecs_resources,
        world: ecs_world
    };

    let mut maybe_engine: Result<Engine, failure::Error> = Engine::new(1200, 1200, Vec2 {x: 0.0, y: -4.8 }, ecs);

    match maybe_engine {
        Ok(mut engine) => {
            let wall_texture = engine.resources.get_texture("water.png", &engine.gl)?;
            let character_texture = engine.resources.get_texture("Character.png", &engine.gl)?;
            let circle_texture = engine.resources.get_texture("circle.png", &engine.gl)?;

            let text_texture = engine.resources.generate_from_text("Game".to_string(), &engine.gl)?;

            let wall: Sprite = engine.new_sprite(
                Vector2::new(20.0, 0.0),
                &Kinematic,
                ColliderType::Box(Vec2 { x: 50.0, y: 1.0 }),
                (1.0, 1.0, 1.0, 1.0).into(),
                &wall_texture
            )?;

            let text: Text = engine.new_text(
                Vector2::new(10.0, 25.0),
                Vector2::new(5.0, 5.0),
                (1.0, 1.0, 1.0, 1.0).into(),
                &text_texture
            );

            engine.ecs.world.push((wall, ));
            engine.ecs.world.push((text, ));

            let character: Sprite = engine.new_sprite(
                Vector2::new(20.0, 20.0),
                &Dynamic,
                ColliderType::Box(Vec2 { x: 1.6, y: 2.0 }),
                (1.0, 1.0, 1.0, 1.0).into(),
                &character_texture
            )?;

            engine.ecs.world.push((character,));

            let mut rng = thread_rng();

            for i in 0..1000 {
                let x = 0.0 + rng.gen_range(10.0..25.0) as f32;
                let y = 15.0 + rng.gen_range(0.0..150.0) as f32;
                let size = 0.0 + rng.gen_range(0.3..1.0) as f32;

                let colorr = 0.0 + rng.gen_range(0.0..1.0) as f32;
                let colorg = 0.0 + rng.gen_range(0.0..1.0) as f32;
                let colorb = 0.0 + rng.gen_range(0.0..1.0) as f32;

                let sprite: Sprite = engine.new_sprite(
                    Vector2::new(x, y),
                    &Dynamic,
                    ColliderType::Circle(size),
                    (colorr, colorg, colorb, 1.0).into(),
                    &circle_texture
                )?;

                engine.ecs.world.push((sprite,));
            }

            engine.run()
        },
        Err(e) => println!("{}", failure_to_string(e))
    }

    Ok(())
}

```
