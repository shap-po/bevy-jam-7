use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

pub trait SetImage {
    /// Set the image if not already set
    fn set_image(&mut self, image: &Handle<Image>);
}

impl SetImage for Sprite {
    fn set_image(&mut self, image: &Handle<Image>) {
        if self.image != *image {
            self.image = image.clone();
        }
    }
}

#[derive(Component)]
#[require(Sprite)]
#[component(on_add = deferred_sprite_on_add::<R>)]
/// Allows selecting sprite image without passing around the assets.
pub struct DeferredSprite<R>(Box<dyn Fn(&R) -> Handle<Image> + Send + Sync + 'static>)
where
    R: Resource;

impl<R> DeferredSprite<R>
where
    R: Resource,
{
    pub fn new(image_getter: impl Fn(&R) -> Handle<Image> + Send + Sync + 'static) -> Self {
        Self(Box::new(image_getter))
    }
}

fn deferred_sprite_on_add<R>(mut world: DeferredWorld, HookContext { entity, .. }: HookContext)
where
    R: Resource,
{
    let resource = world.get_resource::<R>().unwrap(); // panic if no resource
    let Some(deferred_sprite) = world.get::<DeferredSprite<R>>(entity) else {
        return;
    };
    let image = deferred_sprite.0(resource);
    if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
        sprite.image = image;
    }
}
