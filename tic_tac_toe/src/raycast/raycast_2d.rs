use bevy::{
    prelude::{
        Added, Changed, Entity, GlobalTransform, Or, Query,
        RemovedComponents, ResMut, Resource, Vec2, Vec3,
    },
    sprite::Sprite,
};

#[derive(Resource, Debug)]
pub struct Raycaster2d {
    objects: Vec<GameObject>,
}

#[derive(Debug)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        Size { width, height }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        f32::max(point.x, -point.x) <= self.width / 2.
            && f32::max(point.y, -point.y) <= self.height / 2.
    }
}

#[derive(Debug)]
struct GameObject {
    id: Entity,
    size: Size,
    transform: GlobalTransform,
}

impl GameObject {
    pub fn new(id: Entity, size: Size, transform: GlobalTransform) -> GameObject {
        GameObject {
            id,
            size,
            transform,
        }
    }
}

impl Raycaster2d {
    pub fn new() -> Raycaster2d {
        Raycaster2d {
            objects: Vec::new(),
        }
    }

    pub fn register(&mut self, id: Entity, size: Size, transform: GlobalTransform) {
        self.objects.push(GameObject::new(id, size, transform));
    }

    pub fn deregister(&mut self, id: Entity) {
        while let Some(index) = self.objects.iter().position(|o| o.id == id) {
            self.objects.remove(index);
        }
    }

    // Returns a list of entities
    pub fn raycast(&self, position: Vec2) -> Vec<Entity> {
        let position = Vec3::new(position.x, position.y, 0.);
        let mut entities = Vec::new();
        for go in &self.objects {
            let transform = go.transform;
            let position2 = transform.affine().inverse().transform_point3(position);
            if go.size.contains(Vec2::new(position2.x, position2.y)) {
                entities.push(go.id);
            }
        }

        entities
    }
}

pub fn register_sprite(
    mut raycaster: ResMut<Raycaster2d>,
    entities: Query<(Entity, &GlobalTransform, &Sprite), Added<Sprite>>,
) {
    for (id, transform, sprite) in entities.iter() {
        let size = sprite.custom_size.unwrap();
        let size = Size::new(size.x, size.y);
        raycaster.register(id, size, *transform);
    }
}

pub fn update_sprite(
    mut raycaster: ResMut<Raycaster2d>,
    entities: Query<
        (Entity, &GlobalTransform, &Sprite),
        Or<(Changed<Sprite>, Changed<GlobalTransform>)>,
    >,
) {
    for (id, transform, sprite) in entities.iter() {
        raycaster.deregister(id);
        let size = sprite.custom_size.unwrap();
        let size = Size::new(size.x, size.y);
        raycaster.register(id, size, *transform);
    }
}

pub fn deregister_sprite(
    mut raycaster: ResMut<Raycaster2d>,
    removed_entities: RemovedComponents<Sprite>,
) {
    for removed in removed_entities.iter() {
        raycaster.deregister(removed);
    }
}
