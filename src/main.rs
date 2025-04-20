use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let white_material = materials.add(Color::WHITE);
    let light_blue_material = materials.add(Color::srgb_u8(124, 144, 255));

    let circle_mesh = meshes.add(Circle::default());
    let cube_mesh = meshes.add(Cuboid::default());

    let _camera = commands.spawn((
        Camera3d { ..default() },
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    let _light = commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let _circular_base = commands.spawn((
        Mesh3d(circle_mesh),
        MeshMaterial3d(white_material),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::splat(4.0)),
    ));

    let _cube = commands.spawn((
        Mesh3d(cube_mesh),
        MeshMaterial3d(light_blue_material),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
