use amethyst::core::{ArcThreadPool, SystemBundle};
use amethyst::ecs::prelude::{Dispatcher, DispatcherBuilder, System, World};
use amethyst::{DataInit, Error, Result};

pub struct TetristeGameData<'a, 'b> {
    pub base     : Dispatcher<'a, 'b>,
    pub menu     : Dispatcher<'a, 'b>,
    pub ingame   : Dispatcher<'a, 'b>,
    pub running  : Dispatcher<'a, 'b>,
}

impl<'a, 'b> TetristeGameData<'a, 'b> {
    pub fn update(&mut self, world: &mut World, menu: bool, running: bool) {
        self.base.dispatch(&world.res);

        if menu {
            self.menu.dispatch(&world.res);
        } else {
            self.ingame.dispatch(&world.res);
            if running {
                self.running.dispatch(&world.res);
            }
        }
    }
}

pub struct TetristeGameDataBuilder<'a, 'b> {
    pub base    : DispatcherBuilder<'a, 'b>,
    pub menu    : DispatcherBuilder<'a, 'b>,
    pub ingame  : DispatcherBuilder<'a, 'b>,
    pub running : DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for TetristeGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        TetristeGameDataBuilder::new()
    }
}

impl<'a, 'b> TetristeGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        TetristeGameDataBuilder {
            base: DispatcherBuilder::new(),
            menu: DispatcherBuilder::new(),
            ingame: DispatcherBuilder::new(),
            running: DispatcherBuilder::new(),
        }
    }

    pub fn with_base<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
        where for<'c> S: System<'c> + Send + 'a,
    {
        self.base.add(system, name, dependencies);
        self
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> Result<Self>
        where B: SystemBundle<'a, 'b>
    {
        bundle
            .build(&mut self.base)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_menu<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
        where for<'c> S: System<'c> + Send + 'a,
    {
        self.menu.add(system, name, dependencies);
        self
    }

    pub fn with_menu_bundle<B>(mut self, bundle: B) -> Result<Self>
        where B: SystemBundle<'a, 'b>
    {
        bundle
            .build(&mut self.menu)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_ingame<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
        where for<'c> S: System<'c> + Send + 'a,
    {
        self.ingame.add(system, name, dependencies);
        self
    }

    pub fn with_ingame_bundle<B>(mut self, bundle: B) -> Result<Self>
        where B: SystemBundle<'a, 'b>
    {
        bundle
            .build(&mut self.ingame)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_running<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
        where for<'c> S: System<'c> + Send + 'a,
    {
        self.running.add(system, name, dependencies);
        self
    }

    pub fn with_running_bundle<B>(mut self, bundle: B) -> Result<Self>
        where B: SystemBundle<'a, 'b>
    {
        bundle
            .build(&mut self.running)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }
}

impl<'a, 'b> DataInit<TetristeGameData<'a, 'b>> for TetristeGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> TetristeGameData<'a, 'b> {
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut base_dispatcher = self.base.with_pool(pool.clone()).build();
        let mut menu_dispatcher = self.menu.with_pool(pool.clone()).build();
        let mut ingame_dispatcher = self.ingame.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();

        base_dispatcher.setup(&mut world.res);
        menu_dispatcher.setup(&mut world.res);
        ingame_dispatcher.setup(&mut world.res);
        running_dispatcher.setup(&mut world.res);

        TetristeGameData {
            base: base_dispatcher,
            menu: menu_dispatcher,
            ingame: ingame_dispatcher,
            running: running_dispatcher,
        }
    }
}
