extern crate amethyst;

mod game_data;
mod states;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

use game_data::TetristeGameDataBuilder;
use states::MenuState;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = TetristeGameDataBuilder::default()
        .with_base_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;
    let mut game = Application::build("./", MenuState)?.build(game_data)?;

    game.run();

    Ok(())
}
