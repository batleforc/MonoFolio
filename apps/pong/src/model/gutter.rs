use bevy::{
    asset::Assets,
    color::Color,
    math::Vec2,
    prelude::{Bundle, Commands, Component, Mesh, Mesh2d, Query, Rectangle, ResMut},
    sprite::{ColorMaterial, MeshMaterial2d},
    window::Window,
};

use super::{Position, Shape};

pub const GUTTER_HEIGHT: f32 = 20.0;

#[derive(Component)]
pub struct Gutter;

#[derive(Bundle)]
pub struct GutterBundle {
    pub gutter: Gutter,
    pub position: Position,
    pub shape: Shape,
}

impl GutterBundle {
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        GutterBundle {
            gutter: Gutter,
            position: Position(Vec2::new(x, y)),
            shape: Shape(Vec2::new(width, GUTTER_HEIGHT)),
        }
    }
}

pub fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        let top_gutter_y = window_height / 2. - GUTTER_HEIGHT / 2.;
        let bottom_gutter_y = -window_height / 2. + GUTTER_HEIGHT / 2.;

        let top_gutter = GutterBundle::new(0., top_gutter_y, window_width);
        let bottom_gutter = GutterBundle::new(0., bottom_gutter_y, window_width);

        let mesh = Mesh::from(Rectangle::from_size(top_gutter.shape.0));
        let material = ColorMaterial::from(Color::srgb(0., 0., 0.));

        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(material);

        commands.spawn((
            top_gutter,
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(material_handle.clone()),
        ));

        commands.spawn((
            bottom_gutter,
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(material_handle.clone()),
        ));
    }
}
