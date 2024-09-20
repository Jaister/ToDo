use std::io;
use std::io::Write;
use std::fs;
use std::fmt;
use std::string;
use serde_json::{Result, Value, Error,json};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::time::Duration;
use crate::tarea::Tarea;
use crate::tarea::{generar_id, guardar_json};

#[derive(Debug, Default)]
pub struct App {
    opcion: u8,
    exit: bool,
    tareas: Vec<Tarea>,
    state: AppState,
    current_input: String,
}
impl App {
    pub fn tareas(&self) -> &Vec<Tarea> {
        &self.tareas
    }
    pub fn set_tareas(&mut self, new_tareas: Vec<Tarea>) {
        self.tareas = new_tareas;
    }
    pub fn tareas_mut(&mut self) -> &mut Vec<Tarea> {
        &mut self.tareas
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            // Non-blocking poll for events with timeout
            if crossterm::event::poll(Duration::from_millis(200))? {
                match self.state {
                    AppState::Menu => self.handle_menu()?,
                    AppState::CrearTarea => self.handle_create()?,
                    AppState::VerTareas => self.handle_ver_tareas()?,
                    AppState::CompletarTarea => self.handle_completar_tarea()?,
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_menu(&mut self) -> io::Result<()> {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => self.exit(),
                        KeyCode::Left => self.decrementar_opcion(),
                        KeyCode::Right => self.aumentar_opcion(),
                        KeyCode::Enter => self.select_main_menu(),
                        KeyCode::Esc => self.exit(),
                        _ => {}
                    }
                }
            }
        
        Ok(())
    }
    fn handle_create(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Left => self.decrementar_opcion(),
                    KeyCode::Right => self.aumentar_opcion(),
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        self.datos_tarea();
                        self.state = AppState::Menu;
                        self.current_input.clear();
                    }
                    KeyCode::Backspace => {
                        self.current_input.pop();
                    }
                    KeyCode::Char(c) => {
                        self.current_input.push(c);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn handle_ver_tareas(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Left => self.decrementar_opcion(),
                    KeyCode::Right => self.aumentar_opcion(),
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        self.state = AppState::Menu;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn handle_completar_tarea(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Left => self.decrementar_opcion(),
                    KeyCode::Right => self.aumentar_opcion(),
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        self.estado_tarea();
                        self.state = AppState::Menu;
                    }
                    KeyCode::Backspace => {
                        self.current_input.pop();
                    }
                    KeyCode::Char(c) => {
                        self.current_input.push(c);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn aumentar_opcion(&mut self) {
        self.opcion += 1;
    }

    fn decrementar_opcion(&mut self) {
        if self.opcion > 0 {
        self.opcion -= 1;}

    }
    fn select_main_menu(&mut self) {
        match self.opcion {
            1 => {
                //Creacion de tarea, guardamos ultima tarea creada
                self.state = AppState::CrearTarea;
            }
            2 => {
                //Salida por pantalla de las tareas
                self.state = AppState::VerTareas;
            }
            3 => {
                self.state = AppState::CompletarTarea;
            }
            4 => {
                //Eliminar tarea
                /*println!("{}","Ingrese el id de la tarea a eliminar");
                let mut id: String = String::new();
                loop{
                    id.clear();
                    let id: u32 = entrada_opcion(&mut id); arreglar
                    if id > self.tareas.last().unwrap().id{
                        println!("{}","Introduzca un ID valido");
                        continue;
                    }
                    match eliminar_tarea(&mut self.tareas, id){
                        Ok(tarea) => {
                            println!("{}","Tarea eliminada");
                            break;
                        }
                        Err(e) => panic!("Error al eliminar la tarea: {}", e),
                    }
                }*/
            }
            5 => {
                //Salida del programa
                println!("{}","Saliendo...");
                self.exit();}
            _ => {
                self.state = AppState::Menu;
                println!("{}","Opcion no valida");  
            }
        }
    }
    /*
    CREAR TAREA
    */
    fn datos_tarea(&mut self) -> Tarea{
        let mut descripcion: String = String::new();
        descripcion =  self.current_input.clone();
        if descripcion.is_empty(){
            println!("{}","La descripcion no puede estar vacia");
            return Tarea::default();
        }
    
        let id = generar_id(&self.tareas); //Cambiar a last fetched id + 1
        match Tarea::crear_tarea(id, descripcion,&mut self.tareas){
            Ok(tarea) => tarea,
            Err(e) => panic!("Error al crear la tarea: {}", e),
        }

    }
    fn estado_tarea(&mut self){
        println!("{}","Ingrese el id de la tarea a completar");
        let mut id: String = String::new();
        loop{
        id.clear();
        id = self.current_input.clone();
        if id.is_empty() || id.parse::<u32>().is_err(){
            println!("{}","El ID ha de ser un entero");
            return;
        }
        let id: u32 = id.parse().unwrap();
        if id > self.tareas.clone().last().unwrap().id(){
            println!("{}","Introduzca un ID valido");
            continue;
        }
        let mut tarea_encontrada = false;
        for tarea in &mut self.tareas{
            if tarea.id() == id{
                tarea_encontrada = true;
                if tarea.completada(){
                    tarea.descompletar();
                    println!("{}","Tarea descompletada");
                    break;
                }else{
                    tarea.completar();
                    println!("{}","Tarea completada");
                    break;
                }
            }
        }
        if !tarea_encontrada{
            println!("{}","Tarea no encontrada");
            continue;
        }
        else{
            guardar_json(&mut self.tareas);
            break;
        }
        }
    }
    fn render_menu(&self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Menu de Tareas ".bold());
        let instructions = Title::from(Line::from(vec![
            "<Izda>".blue().bold(),
            " Opcion ".into(),
            "<Dcha>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Opcion: ".into(),
            self.opcion.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_crear_tarea(&self, area: Rect, buf: &mut Buffer) {
        // Define a block with a title and border
        let block = Block::default()
            .title(" Crear Nueva Tarea ".bold())
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
    
        // Define the prompt text
        let prompt_text = Text::from("Descripción de la tarea:".bold());
    
        // Define the input text (this could be dynamic if capturing input)
        let input_text = Text::from(self.current_input.clone()); // Assuming you have an `input_description` field
    
        // Render the prompt text
        let prompt_paragraph = Paragraph::new(prompt_text)
            .block(Block::default().borders(ratatui::widgets::Borders::NONE))
            .alignment(Alignment::Left);
        prompt_paragraph.render(area, buf);
    
        // Render the input text below the prompt
        let input_area = Rect {
            x: area.x,
            y: area.y + 2, // Position the input text a bit below the prompt
            width: area.width,
            height: area.height - 2,
        };
        let input_paragraph = Paragraph::new(input_text)
            .block(block)
            .alignment(Alignment::Left);
        input_paragraph.render(input_area, buf);
    
        // Optional: Draw a cursor or highlight the input area
        buf.set_style(area, ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));
        buf.set_style(input_area, ratatui::style::Style::default().bg(ratatui::style::Color::Black));
    }
    

    fn render_ver_tareas(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Crear Nueva Tarea ".bold())
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
    
        // Define the prompt text
        let prompt_text = Text::from("Tareas existentes:".bold());
    
        // Define the input text (this could be dynamic if capturing input)
        let mut string: String = String::new();
        for tarea in self.tareas.clone(){
            string.push_str(&tarea.to_string());
            string.push_str("\n");
        }

        let input_text = Text::from(string); // Assuming you have an `input_description` field
    
        // Render the prompt text
        let prompt_paragraph = Paragraph::new(prompt_text)
            .block(Block::default().borders(ratatui::widgets::Borders::NONE))
            .alignment(Alignment::Left);
        prompt_paragraph.render(area, buf);
    
        // Render the input text below the prompt
        let input_area = Rect {
            x: area.x,
            y: area.y + 2, // Position the input text a bit below the prompt
            width: area.width,
            height: area.height - 2,
        };
        let input_paragraph = Paragraph::new(input_text)
            .block(block)
            .alignment(Alignment::Left);
        input_paragraph.render(input_area, buf);
    
        // Optional: Draw a cursor or highlight the input area
        buf.set_style(area, ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));
        buf.set_style(input_area, ratatui::style::Style::default().bg(ratatui::style::Color::Black));
    }

    fn render_completar_tarea(&self, area: Rect, buf: &mut Buffer) {
        // Define a block with a title and border
        let block = Block::default()
        .title(" Des/completar tarea ".bold())
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

        // Define the prompt text
        let prompt_text = Text::from("Id de la tarea:".bold());

        // Define the input text (this could be dynamic if capturing input)
        let input_text = Text::from(self.current_input.clone()); // Assuming you have an `input_description` field

        // Render the prompt text
        let prompt_paragraph = Paragraph::new(prompt_text)
            .block(Block::default().borders(ratatui::widgets::Borders::NONE))
            .alignment(Alignment::Left);
        prompt_paragraph.render(area, buf);

        // Render the input text below the prompt
        let input_area = Rect {
            x: area.x,
            y: area.y + 2, // Position the input text a bit below the prompt
            width: area.width,
            height: area.height - 2,
        };
        let input_paragraph = Paragraph::new(input_text)
            .block(block)
            .alignment(Alignment::Left);
        input_paragraph.render(input_area, buf);

        // Optional: Draw a cursor or highlight the input area
        buf.set_style(area, ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));
        buf.set_style(input_area, ratatui::style::Style::default().bg(ratatui::style::Color::Black));
    }

    fn render_eliminar_tarea(&self, area: Rect, buf: &mut Buffer) {
        // Implement the rendering logic for EliminarTarea state
    }

    fn render_salir(&self, area: Rect, buf: &mut Buffer) {
        // Implement the rendering logic for Salir state
    }    
}
////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
enum AppState {
    Menu,
    CrearTarea,
    VerTareas,
    CompletarTarea,
    EliminarTarea,
    Salir,
}
impl Default for AppState {
    fn default() -> Self {
        AppState::Menu // Set Menu as the default state
    }
}
////////////////////////////////////////////////////////////
/// RENDER
////////////////////////////////////////////////////////////
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            AppState::Menu => self.render_menu(area, buf),
            AppState::CrearTarea => self.render_crear_tarea(area, buf),
            AppState::VerTareas => self.render_ver_tareas(area, buf),
            AppState::CompletarTarea => self.render_completar_tarea(area, buf),
            AppState::EliminarTarea => self.render_eliminar_tarea(area, buf),
            AppState::Salir => self.render_salir(area, buf),
        }
    }
}