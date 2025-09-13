use crate::task::Task;
use crate::storage::{load_tasks, save_tasks};
use colored::*;

pub fn add_task(title: String) {
    let mut tasks = load_tasks();
    let id = (tasks.len() as u32) + 1;
    let task = Task::new(id, title);
    tasks.push(task);
    save_tasks(&tasks);
    println!("{}", "Tugas berhasil ditambahkan.".green());
}

pub fn list_tasks(filter: Option<&str>) {
    let tasks = load_tasks();

    if tasks.is_empty() {
        println!("{}", "Tidak ada tugas.".yellow());
        return;
    }

    for task in tasks {
        let status = if task.done { "✓" } else { " " };
        
        match filter {
            Some("done") if task.done => println!("[{}] {}: {}", status, task.id, task.title),
            Some("pending") if !task.done => println!("[{}] {}: {}", status, task.id, task.title),
            None => println!("[{}] {}: {}", status, task.id, task.title),
            _ => {}
        }
    }
}

pub fn delete_task(id: u32) {
    let mut tasks = load_tasks();
    if let Some(pos) = tasks.iter().position(|x| x.id == id) {
        tasks.remove(pos);
        save_tasks(&tasks);
        println!("{}", format!("Tugas {} berhasil dihapus.", id).green());
    } else {
        println!("{}", format!("Tugas dengan id {} tidak ditemukan.", id).red());
    }
}

pub fn edit_task(id: u32, new_title: String) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|x| x.id == id) {
        task.title = new_title;
        save_tasks(&tasks);
        println!("{}", format!("Tugas {} berhasil diperbarui.", id).green());
    } else {
        println!("{}", format!("Tugas dengan id {} tidak ditemukan.", id).red());
    }
}

pub fn mark_task_done(id: u32, done: bool) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = done;
        save_tasks(&tasks);
        if done {
            println!("{}", format!("Tugas {} ditandai sebagai selesai.", id).green());
        } else {
            println!("{}", format!("Tugas {} ditandai sebagai belum selesai.", id).green());
        }
    } else {
        println!("{}", format!("Tugas dengan id {} tidak ditemukan.", id).red());
    }
}