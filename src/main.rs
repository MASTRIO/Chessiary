use bevy::prelude::*;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct ArenaTile;

#[derive(Component)]
struct Unloaded {
    reason: UnloadReason
}

#[derive(PartialEq)]
enum UnloadReason {
    TileScale
}

const GAME_GRID: [&str; 400] = [
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", "1", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", "2", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "1", " ", " ", " ", " ", " ", " ", " ", " ",
];

fn render_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for _space in GAME_GRID {
        let mut x_increase: f32 = 0.;
        let mut y_increase: f32 = 0.;

        for _number in 0..=x {
            x_increase += 16.
        }
        for _number in 0..=y {
            y_increase += 16.
        }

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("player.png"),
                transform: Transform::from_translation(Vec3::new(
                    x_increase - 275., // * window.scale_factor() as f32, 
                    y_increase - 167.5,
                    0.)),
                ..Default::default()
            })
            .insert(ArenaTile)
            .insert(Unloaded {
                reason: UnloadReason::TileScale
            });
        
        if x < 19 {
            x += 1;
        } else {
            x = 0;
            y += 1;
        }
    }
}

fn camera_movement(keys: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<(&Camera, &mut Transform)>) {
    for (_camera, mut transform) in query.iter_mut() {
        let speed = 100.;

        if keys.pressed(KeyCode::Right) {
            transform.translation.x += speed * time.delta_seconds()
        }
        if keys.pressed(KeyCode::Left) {
            transform.translation.x -= speed * time.delta_seconds()
        }
        if keys.pressed(KeyCode::Up) {
            transform.translation.y += speed * time.delta_seconds()
        }
        if keys.pressed(KeyCode::Down) {
            transform.translation.y -= speed * time.delta_seconds()
        }
    }
}

fn load(mut commands: Commands, mut query: Query<(Entity, &Unloaded, &mut Transform)>) {
    for (entity, unloaded, mut transform) in query.iter_mut() {
        if unloaded.reason == UnloadReason::TileScale {
            transform.scale.x = 0.5;
            transform.scale.y = 0.5;

            commands.entity(entity)
                .remove::<Unloaded>();
        }
    }
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(Camera);

    let window = windows.get_primary_mut().unwrap();
    
    window.update_scale_factor_from_backend(0.5);

    println!("===== window data =====");
    println!("Title: {}", window.title());
    println!("Requested resolution: ({}, {})", window.requested_width(), window.requested_height());
    println!("Physical resolution: ({}, {})", window.physical_width(), window.physical_height());
    println!("Backend scale factor: {}", window.backend_scale_factor());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.02, 0.02, 0.04)))
        .insert_resource(WindowDescriptor {
            title: format!("Chessiary"),
            //mode: bevy::window::WindowMode::SizedFullscreen,
            width: 1280., //900., //1600.,
            height: 720., //600., //1400.,
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(render_grid)
        .add_system(load)
        .add_system(camera_movement)
        .run();
}
