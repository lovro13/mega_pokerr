pub mod logic;
pub mod terminal_app;

#[cfg(feature = "run_with_sdl2")]
pub mod sdl2_app;

#[cfg(feature = "compile_with_sauron")]
mod sauron_app;
