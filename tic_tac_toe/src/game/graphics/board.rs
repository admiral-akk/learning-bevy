use bevy::{
    prelude::{AssetServer, Color, Commands, Quat, Res, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use super::square::SQUARE_SIZE;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/line.png"),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(3. * SQUARE_SIZE, 25.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(SQUARE_SIZE / 2., 0., 0.),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.),
            ..Default::default()
        },

        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/line.png"),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(3. * SQUARE_SIZE, 25.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-SQUARE_SIZE / 2., 0., 0.),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/line.png"),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(3. * SQUARE_SIZE, 25.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., SQUARE_SIZE / 2., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/line.png"),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(3. * SQUARE_SIZE, 25.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., -SQUARE_SIZE / 2., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
