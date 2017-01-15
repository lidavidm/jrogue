use specs;

pub mod title;

pub use self::title::TitleScreen;

pub trait Screen {
    fn render(&self, planner: &mut specs::Planner<()>);
    fn update(&mut self, planner: &mut specs::Planner<()>);
    fn teardown(&mut self, planner: &mut specs::Planner<()>);
}

// TODO: type aliases, macros for mpsc channels and subscreens
