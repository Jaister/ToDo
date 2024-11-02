mod app;
mod tarea;
mod ascii_art;
mod scraper;

use std::io;
use crate::app::App;
use crate::tarea::Tarea;
use crate::ascii_art::Animation;
use crate::scraper::my_scraper;

fn main() -> io::Result<()> {
    let tareas: Vec<Tarea> = match Tarea::leer_tareas() {
        Ok(tareas_leidas) => tareas_leidas,
        Err(e) => panic!("Error al leer las tareas: {}", e),
    };

    let mut terminal = ratatui::init();
    let mut app = App::default();
    app.set_tareas(tareas.clone());
    app.set_animation(Animation::default());
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
