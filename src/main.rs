mod model;

use model::db::Database;

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
            println!("created")
        },
        Err(_) => {
            println!("error")
        }
    }

    if let Err(e) = my_conn.add_note("test note", "Mon contenu") {
        println!("Erreur lors de l'ajout de la note : {}", e);
    } else {
        println!("Note ajoutée avec succès !");
    }

    match my_conn.get_all_notes() {
        Ok(notes) => {
            println!("Liste de notes :");
            for note in notes {
                println!("- {}", note);
            }
        }
        Err(e) => println!("Erreur lors de la récupération des notes : {}", e),
    }
}
