use bevy::{
    prelude::{AssetServer, Component, Handle, Image, Res},
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
    time::Time,
};

impl Material2d for BrushMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/brush.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct BrushMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    stroke_duration: f32,
    #[texture(2)]
    #[sampler(3)]
    color_texture: Option<Handle<Image>>,
    #[texture(4)]
    #[sampler(5)]
    mask_texture: Option<Handle<Image>>,
    #[uniform(6)]
    base: f32,
    #[uniform(7)]
    alpha: f32,
}

#[derive(Component)]
pub struct Animated;

impl BrushMaterial {
    pub fn increment_time(&mut self, time: &Res<Time>) {
        self.time += time.delta_seconds();
    }

    pub fn long_line_brush(asset_server: &Res<AssetServer>, delay: f32) -> BrushMaterial {
        BrushMaterial {
            time: delay,
            stroke_duration: 0.4,
            color_texture: Some(asset_server.load("images/line.png")),
            mask_texture: Some(asset_server.load("images/linear_mask.png")),
            base: 0.0,
            alpha: 1.,
        }
    }

    pub fn line_brush(asset_server: &Res<AssetServer>, delay: f32) -> BrushMaterial {
        BrushMaterial {
            time: -delay,
            stroke_duration: 0.2,
            color_texture: Some(asset_server.load("images/short_line.png")),
            mask_texture: Some(asset_server.load("images/linear_mask.png")),
            base: 0.05,
            alpha: 0.,
        }
    }

    pub fn o_brush(asset_server: &Res<AssetServer>) -> BrushMaterial {
        BrushMaterial {
            time: 0.,
            stroke_duration: 0.6,
            color_texture: Some(asset_server.load("images/o.png")),
            mask_texture: Some(asset_server.load("images/radial_mask.png")),
            base: 0.05,
            alpha: 0.,
        }
    }
}
