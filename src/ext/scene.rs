use std::borrow::Borrow;

use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    scene::SceneInstance,
};

use crate::{
    AccessResult,
    access::{AsyncEntityMut, AsyncWorld},
};

/// A component that sends a signal and removes itself
/// if a paired `Scene` is loaded.
#[derive(Debug, Component)]
pub struct SceneSignal(async_oneshot::Sender<()>);

/// Send [`SceneSignal`] once scene is loaded.
pub fn react_to_scene_load(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SceneSignal), With<SceneInstance>>,
) {
    for (entity, mut signal) in query.iter_mut() {
        let _ = signal.0.send(());
        commands.entity(entity).remove::<SceneSignal>();
    }
}

impl AsyncWorld {
    /// Spawn a scene and wait for spawning to complete.
    ///
    /// Requires [`react_to_scene_load`] to function.
    pub async fn spawn_scene(&self, bun: impl Bundle) -> AsyncEntityMut {
        let (send, recv) = async_oneshot::oneshot();
        let entity = self.spawn_bundle((bun, SceneSignal(send))).id();
        let _ = recv.await;
        AsyncEntityMut(entity)
    }
}

impl AsyncEntityMut {
    /// Obtain a child by name, alias for `child_by_name`.
    pub fn spawned(&self, name: impl Into<String> + Borrow<str>) -> AccessResult<AsyncEntityMut> {
        self.child_by_name(name)
    }
}
