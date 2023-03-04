use bevy::prelude::Commands;
use extend::ext;

#[ext]
impl<'w, 's> Commands<'w, 's> {
    fn spawn_extra() {}
}
