extern crate clap;
use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use serde_json;
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    description: String,
    completed: bool,
}

fn main() {
    let matches = App::new("RustTaskManager")
        .version("1.0")
        .author("Tu Nombre")
        .about("Gestión de tareas en Rust")
        .subcommand(
            SubCommand::with_name("add")
                .about("Agrega una nueva tarea")
                .arg(Arg::with_name("title").required(true).help("Título de la tarea"))
                .arg(Arg::with_name("description").help("Descripción de la tarea")),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let title = add_matches.value_of("title").unwrap();
            let description = add_matches.value_of("description").unwrap_or("");
            add_task(title, description);
        }
        _ => println!("Comando no reconocido. Utiliza --help para ver las opciones disponibles."),
    }
}

fn add_task(title: &str, description: &str) {
    let tasks = Arc::new(Mutex::new(load_tasks()));
    let num_threads = 4;

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let tasks_clone = Arc::clone(&tasks);
            thread::spawn(move || {
                let mut tasks = tasks_clone.lock().unwrap();
                tasks.push(Task {
                    title: title.to_string(),
                    description: description.to_string(),
                    completed: false,
                });
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let tasks = tasks.lock().unwrap();
    save_tasks(&tasks);
    println!("Tarea agregada: {} - {}", title, description);
}

fn load_tasks() -> Vec<Task> {
    let path = get_data_path();
    if !Path::new(&path).exists() {
        return Vec::new();
    }

    let file = File::open(&path).expect("No se pudo abrir el archivo de datos.");
    serde_json::from_reader(file).expect("No se pudo leer el archivo de datos.")
}

fn save_tasks(tasks: &Vec<Task>) {
    let path = get_data_path();
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .expect("No se pudo abrir el archivo de datos.");
    
    serde_json::to_writer(file, tasks).expect("No se pudo escribir en el archivo de datos.");
}

fn get_data_path() -> String {
    "tasks.json".to_string()
}
