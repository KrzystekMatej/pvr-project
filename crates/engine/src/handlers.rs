use std::marker::PhantomData;

use bevy_app::App;
use bevy_ecs::{
    resource::Resource,
    system::{
        IntoSystem, RegisteredSystemError, SystemId, SystemInput,
    },
};

pub trait HandlerKind: 'static {
    type Input: SystemInput + 'static;
}

#[derive(Resource)]
struct HandlerRegistry<K: HandlerKind> {
    handlers: Vec<SystemId<K::Input, ()>>,
    _marker: PhantomData<fn() -> K>,
}

impl<K: HandlerKind> Default for HandlerRegistry<K> {
    fn default() -> Self {
        Self {
            handlers: Vec::new(),
            _marker: PhantomData,
        }
    }
}

pub trait HandlerAppExt {
    fn register_handler<K, M>(
        &mut self,
        handler: impl IntoSystem<K::Input, (), M> + 'static,
    ) -> SystemId<K::Input, ()>
    where
        K: HandlerKind;

    fn run_handlers_with<K>(
        &mut self,
        input: <K::Input as SystemInput>::Inner<'_>,
    ) -> Result<(), RegisteredSystemError<K::Input, ()>>
    where
        K: HandlerKind,
        for<'a> <K::Input as SystemInput>::Inner<'a>: Clone;
}

impl HandlerAppExt for App {
    fn register_handler<K, M>(
        &mut self,
        handler: impl IntoSystem<K::Input, (), M> + 'static,
    ) -> SystemId<K::Input, ()>
    where
        K: HandlerKind,
    {
        self.init_resource::<HandlerRegistry<K>>();

        let id = self.register_system(handler);

        self.world_mut()
            .resource_mut::<HandlerRegistry<K>>()
            .handlers
            .push(id);

        id
    }

    fn run_handlers_with<K>(
        &mut self,
        input: <K::Input as SystemInput>::Inner<'_>,
    ) -> Result<(), RegisteredSystemError<K::Input, ()>>
    where
        K: HandlerKind,
        for<'a> <K::Input as SystemInput>::Inner<'a>: Clone,
    {
        let handlers = self
            .world()
            .get_resource::<HandlerRegistry<K>>()
            .map(|registry| registry.handlers.clone())
            .unwrap_or_default();

        for handler in handlers {
            self.world_mut()
                .run_system_with(handler, input.clone())?;
        }

        Ok(())
    }
}