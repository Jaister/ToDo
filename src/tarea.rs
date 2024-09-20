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

#[derive(Debug, Default)]
pub struct Tarea {
    id: u32,
    descripcion: String,
    completada: bool,
}

impl Tarea{
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn descripcion(&self) -> &str {
        &self.descripcion
    }

    pub fn completada(&self) -> bool {
        self.completada
    }
    pub fn crear_tarea(id: u32, descripcion: String,tareas: &mut Vec<Tarea> ) -> serde_json::Result<Tarea>{
        let tarea = Tarea {
            id,
            descripcion: descripcion,
            completada: false,
        };
        tareas.push(tarea.clone());
        guardar_json(tareas);
        Ok(tarea)   
    }
    pub fn leer_tareas() -> serde_json::Result<Vec<Tarea>>{
        if fs::metadata("tareas.json").is_err(){
            fs::write("tareas.json", "{}").map_err(|e| serde_json::Error::io(e))?;
        }
        let json_string = fs::read_to_string("tareas.json").map_err(|e| serde_json::Error::io(e))?;
        if json_string.is_empty(){
            return Ok(Vec::new());
        }
        let data: Value = serde_json::from_str(&json_string)?;
        let mut tareas: Vec<Tarea> = Vec::new();
        if let Some(array) = data.as_array() {
            // If it's an array, iterate over it
            for tarea in array {
                let id = tarea["id"].as_u64().unwrap() as u32;
                let descripcion = tarea["descripcion"].as_str().unwrap().to_string();
                let completada = tarea["completada"].as_bool().unwrap();
                tareas.push(Tarea {
                    id,
                    descripcion,
                    completada,
                });
            }
        } else if let Some(object) = data.as_object() {
            if object.is_empty() {
                return Ok(Vec::new());
            }
            // If it's a single object, handle it as a single task
            let id = object["id"].as_u64().unwrap() as u32;
            let descripcion = object["descripcion"].as_str().unwrap().to_string();
            let completada = object["completada"].as_bool().unwrap();
            tareas.push(Tarea {
                id,
                descripcion,
                completada,
            });
        }
        Ok(tareas)
    }
    pub fn completar(&mut self){
        self.completada = true;
    }
    pub fn descompletar(&mut self){
        self.completada = false;
    }
    pub fn to_string(&self) -> String{
        format!("{} - {} - {}", self.id, self.descripcion, self.completada)
    }

}
/*
DISPLAY
*/
impl fmt::Display for Tarea{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} - {} - {}", self.id, self.descripcion, self.completada)
    }
}
/*
CLONE TAREAS
*/
impl Clone for Tarea{
    fn clone(&self) -> Tarea{
        Tarea{
            id: self.id,
            descripcion: self.descripcion.clone(),
            completada: self.completada,
        }
    }
}
pub fn generar_id(tareas: &Vec<Tarea>) -> u32 {
    if tareas.is_empty() {
        return 1;
    }
    else{
        tareas.last().unwrap().id +1
    }
}
pub fn guardar_json(tareas: &mut Vec<Tarea>) -> serde_json::Result<()>{
    let mut json_array: Vec<Value> = Vec::new();
        for tarea in tareas.clone(){
            let tarea2 = json!({
                "id": tarea.id,
                "descripcion": tarea.descripcion,
                "completada": tarea.completada,
            });
            json_array.push(tarea2);
        }
        let tareas_string = serde_json::to_string_pretty(&json_array)?;
        fs::write("tareas.json", tareas_string).map_err(|e| serde_json::Error::io(e))?;
        Ok(()) 
}
pub fn eliminar_tarea(tareas: &mut Vec<Tarea>, id: u32) -> serde_json::Result<Tarea>{
    let mut tarea_encontrada = false;
    for (index, tarea) in tareas.iter().enumerate(){
        if tarea.id == id{
            tareas.remove(index);
            tarea_encontrada = true;
            break;
        }
    }
    if !tarea_encontrada{
        panic!("Tarea no encontrada");
    }
    guardar_json(tareas);
    Ok(tareas[1].clone())
}