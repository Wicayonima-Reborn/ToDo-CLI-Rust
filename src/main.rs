mod task;
mod storage;
mod commands;

use std::env;
use commands::*;

fn print_help() {
    println!("  Todo CLI - Perintah yang tersedia:");
    println!("  add \"judul task\"       → Tambah task baru");
    println!("  list                    → Tampilkan semua task");
    println!("  list --done             → Tampilkan hanya task selesai");
    println!("  list --pending          → Tampilkan hanya task pending");
    println!("  delete <id>             → Hapus task berdasarkan id");
    println!("  edit <id> \"judul baru\" → Edit judul task");
    println!("  done <id>               → Tandai task selesai");
    println!("  undone <id>             → Tandai task belum selesai");
    println!("  help                    → Tampilkan bantuan");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("⚠️ Contoh: cargo run -- add \"Belajar Rust\"");
            } else {
                let title = args[2..].join(" ");
                add_task(title);
            }
        }
        "list" => {
            if args.len() > 2 {
                match args[2].as_str() {
                    "--done" => list_tasks(Some("done")),
                    "--pending" => list_tasks(Some("pending")),
                    _ => {
                        println!("⚠️ Flag tidak dikenali!");
                        print_help();
                    }
                }
            } else {
                list_tasks(None);
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("⚠️ Contoh: cargo run -- delete 1");
            } else {
                let id: u32 = args[2].parse().unwrap_or(0);
                delete_task(id);
            }
        }
        "edit" => {
            if args.len() < 4 {
                println!("⚠️ Contoh: cargo run -- edit 1 \"Judul Baru\"");
            } else {
                let id: u32 = args[2].parse().unwrap_or(0);
                let new_title = args[3..].join(" ");
                edit_task(id, new_title);
            }
        }
        "done" => {
            if args.len() < 3 {
                println!("⚠️ Contoh: cargo run -- done 1");
            } else {
                let id: u32 = args[2].parse().unwrap_or(0);
                mark_task_done(id, true);
            }
        }
        "undone" => {
            if args.len() < 3 {
                println!("⚠️ Contoh: cargo run -- undone 1");
            } else {
                let id: u32 = args[2].parse().unwrap_or(0);
                mark_task_done(id, false);
            }
        }
        "help" => print_help(),
        _ => {
            println!("⚠️ Command tidak dikenali.");
            print_help();
        }
    }
}