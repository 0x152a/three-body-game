cfg_if::cfg_if! {
    if #[cfg(feature = "gui-macroquad")] {
        mod gui_macroquad;
        use gui_macroquad as app;
    } else
    if #[cfg(feature = "gui-nannou")] {
        mod gui_nannou;
        use gui_nannou as app;
    } else
    if #[cfg(feature = "tui")] {
        pub mod tui;
        use tui as app;
    }
    else{
        compile_error!("No frontend enabled");
    }
}

pub use app::main;
