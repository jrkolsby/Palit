extern crate cursive;
extern crate wavefile;
extern crate itertools;

use std::fs;

use cursive::Cursive;
use cursive::theme::{BorderStyle, Color, BaseColor, Palette, PaletteColor, Theme};

mod components;
mod views;
mod core;

use views::{Home, HomeState};

// const HOME_DIR = "/usr/local/palit/" // PROD
const HOME_DIR: &str = "storage/";


fn main() -> std::io::Result<()> {

    let mut index = Cursive::default();

    index.add_global_callback('q', |s| s.quit());
    index.add_global_callback('~', |s| s.toggle_debug_console());

    let mut palette: Palette = Palette::default();

    palette[PaletteColor::Background] = Color::TerminalDefault;
    palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::White);
    palette[PaletteColor::TitleSecondary] = Color::Light(BaseColor::White);

    index.set_theme(Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: palette,
    });

    let mut home_projects: Vec<String> = Vec::new();

    let home_paths = fs::read_dir(HOME_DIR).unwrap();
    for path in home_paths {
        home_projects.push(path.unwrap().path().display().to_string());
    };

    index.add_layer(Home::new(HomeState {
        openProject: 0,
        projects: home_projects,
        motd: "Heyo!!".to_string()
    }));

    index.run();

    Ok(())
}
