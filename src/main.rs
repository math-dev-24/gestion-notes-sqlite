mod model;
mod utils;
use model::db::Database;
use utils::action::{handle_add_note, handle_delete_note, handle_view_all_notes};

fn check_action(number: &str) -> Option<u32> {
    match number.trim().parse::<u32>() {
        Ok(num) if (1..=5).contains(&num) => Some(num),
        _ => None
    }
}

fn main() {
    let my_conn = match Database::new("notes.db") {
        Ok(data) => data,
        Err(e) => {
            println!("Error : {}",e);
            return;
        }
    };

    match my_conn.init() {
        Ok(_) => {
            println!("created/connected")
        },
        Err(_) => {
            println!("error")
        }
    }

    loop {
        println!("Que voulez-vous faire ?");
        println!("1. Ajouter");
        println!("2. Supprimer");
        println!("3. Details d'une note");
        println!("4. Voir tous");
        println!("5. Quitter");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Erreur lors de la lecture !");

        let num = match check_action(&choice) {
            Some(num) => num,
            None => {
                println!("Entrée invalide !");
                continue;
            }
        };

        match num {
            1 => handle_add_note(&my_conn),
            2 => handle_delete_note(&my_conn),
            4 => handle_view_all_notes(&my_conn),
            5 => {
                println!("Au revoir !");
                break;
            },
            _ => {
                println!("Entrée invalide !")
            }
        }
    }
}
