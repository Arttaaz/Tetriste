use amethyst::prelude::*;
use crate::game_data::TetristeGameData;

pub struct MenuState;

impl<'a, 'b> State<TetristeGameData<'a, 'b>, StateEvent> for MenuState {
    fn on_start(&mut self, data: StateData<TetristeGameData>) {
        initialise_menu(data.world);
    }

    fn update(&mut self, data: StateData<TetristeGameData>) -> Trans<TetristeGameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, true, false);
        Trans::None
    }
}

fn initialise_menu(_world: &mut World) {
    
}
