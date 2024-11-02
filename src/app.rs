use std::io;
use serde_json::{Result, Value, Error,json};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use arboard::Clipboard;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Stylize ,Color,Style},
    symbols::border,
    text::{Line, Text,Span},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
        Borders,
    },
    DefaultTerminal, Frame,
};
use chrono::{Datelike, Local, Timelike};

use crate::tarea::Tarea;
use crate::tarea::{generar_id, guardar_json};
use crate::ascii_art::Animation;
use crate::scraper::my_scraper;
use std::time::{Duration, Instant};
/////////////////////////////////
/// APP
/////////////////////////////////
#[derive(Debug)]
pub struct App {
    opcion: u8,
    exit: bool,
    tareas: Vec<Tarea>,
    state: AppState,
    current_input: String,
    previous_state: AppState,
    warning_message: String,
    animation: Animation,
}
impl App {
    fn draw(&self, frame: &mut ratatui::Frame) {
        // This is where you define what to render
        let area = frame.area(); // Get the available area to draw
        frame.render_widget(self, area); // Render the app itself
    }
    
    pub fn tareas(&self) -> &Vec<Tarea> {
        &self.tareas
    }
    pub fn set_tareas(&mut self, new_tareas: Vec<Tarea>) {
        self.tareas = new_tareas;
    }
    pub fn set_animation(&mut self, new_animation: Animation) {
        self.animation = new_animation;
    }
    pub fn tareas_mut(&mut self) -> &mut Vec<Tarea> {
        &mut self.tareas
    }
    pub fn set_warning(&mut self) {
        if self.state == AppState::Warning {
            return;
        }
        self.previous_state = self.state;
        self.state = AppState::Warning;
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_frame_time = Instant::now();
        let frame_duration = Duration::from_nanos(300); // Adjust as needed
    
        while !self.exit {
            // Non-blocking poll for events with timeout
            if crossterm::event::poll(Duration::from_millis(200))? {
                match self.state {
                    AppState::Menu => self.handle_menu()?,
                    AppState::CrearTarea => self.handle_create()?,
                    AppState::VerTareas => self.handle_ver_tareas()?,
                    AppState::CompletarTarea => self.handle_completar_tarea()?,
                    AppState::Warning => self.handle_warning()?,
                    AppState::EliminarTarea => self.handle_eliminar_tarea()?,
                    AppState::Scraper => self.handle_scraper()?,
                    _ => {}
                }
            }
    
            // Check if it's time to update the frame
            if last_frame_time.elapsed() >= frame_duration {
                terminal.draw(|frame| self.draw(frame))?; // Draw the current frame
                last_frame_time = Instant::now(); // Reset the timer
                if self.state == AppState::Menu {
                    self.animation.next();
                }
            }
        }
    
        Ok(())
    }
    
    fn handle_menu(&mut self) -> io::Result<()> {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Left => self.decrementar_opcion(),
                        KeyCode::Right => self.aumentar_opcion(),
                        KeyCode::Enter => self.select_main_menu(),
                        KeyCode::Esc => self.exit(),
                        KeyCode::Down => {
                            self.state = self.previous_state;
                        }
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
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        match self.datos_tarea(){
                            Ok(_tarea) => {
                                self.state = AppState::Menu;
                            }
                            Err(_e) => {
                                self.set_warning();
                            }
                        }

                        self.current_input.clear();
                    }
                    KeyCode::Backspace => {
                        self.current_input.pop();
                    }
                    KeyCode::Char(c) => {
                        self.current_input.push(c);
                    }
                    KeyCode::Down => {
                        self.state = self.previous_state;
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
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        self.state = AppState::Menu;
                    }
                    KeyCode::Down => {
                        self.state = self.previous_state;
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
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        match self.estado_tarea() {
                            Ok(()) => {
                                self.state = AppState::Menu;
                            }
                            Err(_e) => {
                                self.set_warning();
                            }
                        }
                        self.current_input.clear();
                    }
                    KeyCode::Backspace => {
                        self.current_input.pop();
                    }
                    KeyCode::Char(c) => {
                        self.current_input.push(c);
                    }
                    KeyCode::Down => {
                        self.state = self.previous_state;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn handle_eliminar_tarea(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        match self.eliminar_tarea() {
                            Ok(()) => {
                                self.state = AppState::Menu;
                            }
                            Err(_e) => {
                                self.set_warning();
                            }
                        }
                        self.current_input.clear();
                    }
                    KeyCode::Backspace => {
                        self.current_input.pop();
                    }
                    KeyCode::Char(c) => {
                        self.current_input.push(c);
                    }
                    KeyCode::Down => {
                        self.state = self.previous_state;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn handle_scraper(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        match my_scraper(self.current_input.clone().as_str()) {
                            Ok(()) => {
                                self.state = AppState::Menu;
                            }
                            Err(_e) => {
                                self.set_warning();
                            }
                        }
                        self.current_input.clear();
                    }
                    KeyCode::Backspace => {
                        self.current_input.pop();
                    }
                    KeyCode::Char(c) => {
                        self.current_input.push(c);
                    }
                    KeyCode::Down => {
                        self.state = self.previous_state;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_warning(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Esc => self.exit(),
                    KeyCode::Enter => {
                        self.state = self.previous_state;
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
                self.state = AppState::EliminarTarea;
            }
            5 => {
                self.state = AppState::Scraper;
            }
            _ => {
                self.set_warning();
            }
        }
    }
    ///////////////////////////////////////////
    /// DATOS TAREA
    ///////////////////////////////////////////
    fn datos_tarea(&mut self) -> io::Result<Tarea>{
        let descripcion =  self.current_input.clone();
        if descripcion.is_empty(){
            self.set_warning();
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Descripción vacía")); // Proper io::Error
            
        }
        let local_now = Local::now();
        let fecha = local_now.date_naive();
    
        let id = generar_id(&self.tareas); //Cambiar a last fetched id + 1
        match Tarea::crear_tarea(id, descripcion,fecha,&mut self.tareas){
            Ok(tarea) => Ok(tarea),
            Err(_e) => Err(io::Error::new(io::ErrorKind::InvalidInput, "Error al crear Tarea")), // Proper io::Error
            }

    }
    ///////////////////////////////////////////
    /// ESTADO TAREA
    ///////////////////////////////////////////
    fn estado_tarea(&mut self) -> io::Result<()>{
        let mut id: String = String::new();
        loop{
        id.clear();
        id = self.current_input.clone();
        if id.is_empty() || id.parse::<u32>().is_err(){ //NO ES UN ENTERO
            self.set_warning();
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Error al cambiar el estado de la Tarea")); // Proper io::Error
        }
        let id: u32 = id.parse().unwrap();
        if id > self.tareas.clone().last().unwrap().id(){ // ID NO EXISTE PORQUE ES MAYOR AL ULTIMO ID
            self.set_warning();
            continue;
        }
        let mut tarea_encontrada = false;
        for tarea in &mut self.tareas{
            if tarea.id() == id{
                tarea_encontrada = true;
                if tarea.completada(){
                    tarea.descompletar();
                    break;
                }else{
                    tarea.completar();
                    break;
                }
            }
        }
        if !tarea_encontrada{ // ID NO EXISTE
            self.set_warning();
            continue;
        }
        else{
            match guardar_json(&mut self.tareas){
                Ok(()) => return Ok(()),
                Err(e) => panic!("Error al guardar las tareas: {}", e),
            }
        }
        }
    }
    fn eliminar_tarea(&mut self) -> io::Result<()>{
        let mut id: String = String::new();
        loop{
        id.clear();
        id = self.current_input.clone();
        if id.is_empty() || id.parse::<u32>().is_err(){ //NO ES UN ENTERO
            self.set_warning();
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Error al eliminar la Tarea")); // Proper io::Error
        }
        let id: u32 = id.parse().unwrap();
        if id > self.tareas.clone().last().unwrap().id(){ // ID NO EXISTE PORQUE ES MAYOR AL ULTIMO ID
            self.set_warning();
            continue;
        }
        let mut tarea_encontrada = false;
        for tarea in &mut self.tareas{
            if tarea.id() == id{
                tarea_encontrada = true;
                self.tareas.retain(|x| x.id() != id);
                break;
            }
        }
        if !tarea_encontrada{ // ID NO EXISTE
            self.set_warning();
            continue;
        }
        else{
            match guardar_json(&mut self.tareas){
                Ok(()) => return Ok(()),
                Err(e) => panic!("Error al guardar las tareas: {}", e),
            }
        }
        }
    }
    ///////////////////////////////////////////
    /// MENU RENDER
    ///////////////////////////////////////////
    fn render_menu(&self, area: Rect, buf: &mut Buffer) {
        let mut title = Title::default();
        match self.opcion{
            1 => {
                title = Title::from(" Crear Tarea ".bold());
            }
            2 => {
                title = Title::from(" Ver Tareas ".bold());
            }
            3 => {
                title = Title::from(" Completar/Descompletar Tarea ".bold());
            }
            4 => {
                title = Title::from(" Eliminar Tarea ".bold());
            }
            5 => {
                title = Title::from(" Scrapper ".bold());
            }
            _ => {
                title = Title::from(" Menu de Tareas ".bold());
            }
        }
        //TITULO DEL OS
        let title_os = Animation::print_os_name(&self.animation);
        let title_os = Text::from(title_os).yellow();
        
        // Create an area for the title ASCII art
        let ascii_width = title_os.width(); // Width of ASCII art
        let ascii_height = title_os.height(); // Height of ASCII art
        if area.height > 30 && area.width > 100{
            // Calculate the centered area for ASCII art
        let ascii_area = Rect {
            x: area.x + (area.width as u16 - ascii_width as u16) / 2, // Centered x position
            y: (area.height / 4) ,
            width: ascii_width as u16, // Cast width to u16
            height: ascii_height as u16, // Cast height to u16
        };
        
        // Render the ASCII art paragraph within the centered area
        Paragraph::new(title_os)
            .block(Block::default().borders(ratatui::widgets::Borders::NONE)) // Optional: add borders to the ASCII art block
            .render(ascii_area, buf);
        }
        let instructions = Title::from(Line::from(vec![
            "<Izda>".blue().bold(),
            " Opcion ".into(),
            "<Dcha>".blue().bold(),
            " Quit ".into(),
            "<ESC> ".blue().bold(),
        ]));       
        let block = Block::bordered()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .border_set(border::THICK)
        .border_style(ratatui::style::Style::default().fg(self.color_logic()));
    
        let counter_text = Text::from(vec![Line::from(vec![
            "Opcion: ".into(),
            self.opcion.to_string().yellow(),
        ])]);
                
        
        
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
        

        //DISPLAY TIME ON TOP LEFT CORNER
        let current_time = Local::now();
        let time_text = Text::from(vec![Line::from(vec![
            "Tiempo: ".into(),
            format!("{:02}-{:02}-{:02}  {:02}:{:02}:{:02}",
                current_time.year(),
                current_time.month(),
                current_time.day(), 
                current_time.hour(), 
                current_time.minute(),
                current_time.second()).yellow(), // Format the time and apply yellow color
        ])]);
        // Render the counter text in the top right corner
        let counter_area = Rect {
            x: area.x + area.width - 55,
            y: area.y,
            width: 55,
            height: 1,
        };
        Paragraph::new(time_text)
            .block(Block::default().borders(ratatui::widgets::Borders::NONE))
            .render(counter_area, buf);
        // Assuming Animation::print_ascii_art returns a string representation of the ASCII art
        let ascii_art = Animation::print_ascii_art(&self.animation);
        let ascii_art_text = Text::from(ascii_art).yellow();
        
        // Create an area for the ASCII art
        let ascii_width = ascii_art_text.width(); // Width of ASCII art
        let ascii_height = ascii_art_text.height(); // Height of ASCII art
        if area.height > 50 && area.width > 100{
            // Calculate the centered area for ASCII art
        let ascii_area = Rect {
            x: area.x + (area.width as u16 - ascii_width as u16) / 2, // Centered x position
            y: (area.height / 5)* 3 ,
            width: ascii_width as u16, // Cast width to u16
            height: ascii_height as u16, // Cast height to u16
        };
        // Render the ASCII art paragraph within the centered area
        Paragraph::new(ascii_art_text)
            .block(Block::default().borders(ratatui::widgets::Borders::NONE)) // Optional: add borders to the ASCII art block
            .render(ascii_area, buf);
        
        }
    }
    ///////////////////////////////////////////
    /// CREAR TAREA RENDER
    ///////////////////////////////////////////
    fn render_crear_tarea(&self, area: Rect, buf: &mut Buffer) {
        // Define a block with a title and border
        let block = Block::default()
            .title(" Crear Nueva Tarea ".bold())
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(self.color_logic()));
    
        // Define the prompt text
        let prompt_text = Text::from("Descripción de la tarea:".bold());
    
        // Define the input text (this could be dynamic if capturing input)
        let input_text = Text::from(self.current_input.clone()); // Assuming you have an `input_description` field
        // Render the prompt text
        let prompt_paragraph = Paragraph::new(prompt_text)
            .block(Block::default().borders(ratatui::widgets::Borders::NONE))
            //.block(warning)
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
        buf.set_style(area, ratatui::style::Style::default().bg(self.color_logic()));
        // Optional: Draw a cursor or highlight the input area
        buf.set_style(input_area, ratatui::style::Style::default().bg(ratatui::style::Color::Black));
    }
    ///////////////////////////////////////////
    /// VER TAREAS RENDER (Should be warning safe)
    ///////////////////////////////////////////

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
        buf.set_style(area, ratatui::style::Style::default().bg(ratatui::style::Color::Cyan));
        buf.set_style(input_area, ratatui::style::Style::default().bg(ratatui::style::Color::Black));
    }
    ///////////////////////////////////////////
    /// COMPLETAR TAREA RENDER
    ///////////////////////////////////////////
    fn render_completar_tarea(&self, area: Rect, buf: &mut Buffer) {
        // Define a block with a title and border
        let block = Block::default()
        .title(" Des/completar tarea ".bold())
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(self.color_logic()));

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
        buf.set_style(area, ratatui::style::Style::default().bg(self.color_logic()));
        buf.set_style(input_area, ratatui::style::Style::default().bg(ratatui::style::Color::Black));
    }

    fn render_eliminar_tarea(&self, area: Rect, buf: &mut Buffer) {
                // Define a block with a title and border
        let block = Block::default()
        .title(" Eliminar tarea ".bold())
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(self.color_logic()));

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
        buf.set_style(area, ratatui::style::Style::default().bg(self.color_logic()));
        buf.set_style(input_area, ratatui::style::Style::default().bg(ratatui::style::Color::Black));
    }
    ///////////////////////////////////////////
    /// SCRAPER RENDER
    /// ///////////////////////////////////////////
    fn render_scraper(&self, area: Rect, buf: &mut Buffer) {
        // Define a block with a title and border
        let block = Block::default()
            .title(" Web Scraping ".bold())
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(self.color_logic()));
    
        // Define the prompt text
        let prompt_text = Text::from("URL a scrapear:".bold());
    
        // Define the input text (this could be dynamic if capturing input)
        let input_text = Text::from(self.current_input.clone()); // Assuming you have an `input_description` field
        // Render the prompt text
        let prompt_paragraph = Paragraph::new(prompt_text)
            .block(Block::default().borders(ratatui::widgets::Borders::NONE))
            //.block(warning)
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
        buf.set_style(area, ratatui::style::Style::default().bg(self.color_logic()));
        // Optional: Draw a cursor or highlight the input area
        buf.set_style(input_area, ratatui::style::Style::default().bg(ratatui::style::Color::Black));
    }
    ///////////////////////////////////////////
    /// WARNING RENDER
    ///////////////////////////////////////////
    fn color_logic(&self) -> ratatui::style::Color{
        if self.state == AppState::Warning {ratatui::style::Color::LightRed} else {ratatui::style::Color::Cyan}
    }
    fn render_warning(&self, area: Rect, buf: &mut Buffer) {
        // Calculate dimensions of the warning box (e.g., take 25% of width and 25% of height)
        let box_width = area.width / 4;
        let box_height = area.height / 4;

        // Center the warning box in the middle of the terminal
        let warning_area = Rect {
            x: (area.width - box_width) / 2,  // Center horizontally
            y: (area.height - box_height) / 2,  // Center vertically
            width: box_width,
            height: box_height,
        };

        // Create the main warning message and the "Press 'q' to dismiss" text
        let text = vec![
            Line::from(Span::styled(self.warning_message.clone(), Style::default().fg(Color::White).bold())),  // Main warning message
            Line::from(Span::raw("")),  // Empty line for spacing
            Line::from(Span::styled("Press 'ENTER' to dismiss", Style::default().fg(Color::White))),  // Bottom message
            Line::from(Span::styled("OR", Style::default().fg(Color::White))),  // Bottom message
            Line::from(Span::styled("Press 'ESC' to exit", Style::default().fg(Color::White))),  // Bottom message
        ];

        // Create the paragraph with both the warning message and the "q" message at the bottom
        let warning = Paragraph::new(Text::from(text))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))  // Border styling
            )
            .alignment(Alignment::Center);  // Center the text blocks vertically

        // Render the warning box
        warning.render(warning_area, buf);

        // Set the background style of the warning box
        buf.set_style(warning_area, Style::default().bg(self.color_logic()));
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
    Warning,
    Scraper,
}
impl Default for AppState {
    fn default() -> Self {
        AppState::Menu // Set Menu as the default state
    }
}
impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AppState::Menu, AppState::Menu) => true,
            (AppState::CrearTarea, AppState::CrearTarea) => true,
            (AppState::VerTareas, AppState::VerTareas) => true,
            (AppState::CompletarTarea, AppState::CompletarTarea) => true,
            (AppState::EliminarTarea, AppState::EliminarTarea) => true,
            (AppState::Warning, AppState::Warning) => true,
            (AppState::Scraper, AppState::Scraper) => true,
            _ => false,
        }
    }
}
impl Default for App {
    fn default() -> Self {
        App {
            opcion: 0,
            exit: false,
            tareas: Vec::new(),
            state: AppState::default(),
            current_input: String::new(),
            previous_state: AppState::default(),
            warning_message: WARNING_MESSAGES[0].to_string(),
            animation: Animation::default(),
        }
    }
}
const WARNING_MESSAGES: [&'static str; 3] = [
    "Opción no válida",
    "ID no válido",
    "No hay tareas para mostrar",
];
////////////////////////////////////////////////////////////
/// RENDER
////////////////////////////////////////////////////////////
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state == AppState::Warning{
            match self.previous_state {
                AppState::Menu => self.render_menu(area, buf),
                AppState::CrearTarea => self.render_crear_tarea(area, buf),
                AppState::VerTareas => self.render_ver_tareas(area, buf),
                AppState::CompletarTarea => self.render_completar_tarea(area, buf),
                AppState::EliminarTarea => self.render_eliminar_tarea(area, buf),
                AppState::Scraper => self.render_scraper(area, buf),
                _ => {}
            }
            self.render_warning(area, buf); //Add warning toast on top of current render
        }
        else{
        match self.state {
            AppState::Menu => self.render_menu(area, buf),
            AppState::CrearTarea => self.render_crear_tarea(area, buf),
            AppState::VerTareas => self.render_ver_tareas(area, buf),
            AppState::CompletarTarea => self.render_completar_tarea(area, buf),
            AppState::EliminarTarea => self.render_eliminar_tarea(area, buf),
            AppState::Scraper => self.render_scraper(area, buf),
            _ => {}
        }
    }
    }
    
}
