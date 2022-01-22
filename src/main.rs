use bevy::prelude::*;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct ArenaTile;

#[derive(Component)]
struct Unloaded {
    reason: String
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
    let mut x: f32 = -200.;
    let mut y: f32 = -145.;

    for _space in GAME_GRID {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("player.png"),
                transform: Transform::from_translation(Vec3::new(
                    x, 
                    y,
                    0.)),
                ..Default::default()
            })
            .insert(ArenaTile)
            .insert(Unloaded {
                reason: String::from("tile_scale")
            });
        
        if x < 400. {
            x += 32.;
        } else {
            x = -200.;
            y += 32.;
        }
    }
}

fn camera_movement(keys: Res<Input<KeyCode>>, mut query: Query<(&Camera, &mut Transform)>) {
    for (_camera, mut transform) in query.iter_mut() {
        let speed = 10.;

        if keys.pressed(KeyCode::Right) {
            transform.translation.x += speed
        }
        if keys.pressed(KeyCode::Left) {
            transform.translation.x -= speed
        }
        if keys.pressed(KeyCode::Up) {
            transform.translation.y += speed
        }
        if keys.pressed(KeyCode::Down) {
            transform.translation.y -= speed
        }
    }
}

fn load(mut commands: Commands, mut query: Query<(Entity, &Unloaded, &mut Transform)>) {
    for (entity, unloaded, mut transform) in query.iter_mut() {
        if unloaded.reason == "tile_scale" {
            transform.scale.x = 0.5;
            transform.scale.y = 0.5;

            commands.entity(entity)
                .remove::<Unloaded>();
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(Camera);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
        .insert_resource(WindowDescriptor {
            title: format!("Chessiary"),
            //mode: bevy::window::WindowMode::SizedFullscreen,
            width: 600., //1600.,
            height: 400., //1400.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(render_grid)
        .add_system(load)
        .add_system(camera_movement)
        .run();
}
