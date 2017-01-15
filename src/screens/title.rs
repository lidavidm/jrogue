use bear_lib_terminal;
use specs;

use super::Screen;

pub struct TitleScreen;

impl Screen for TitleScreen {
    fn render(&self, _planner: &mut specs::Planner<()>) {
        bear_lib_terminal::terminal::print_xy(0, 0, "jRogue");
    }

    fn update(&mut self, planner: &mut specs::Planner<()>) {
        planner.dispatch(());
    }

    fn teardown(&mut self, planner: &mut specs::Planner<()>) {
        planner.systems.clear();
    }
}
