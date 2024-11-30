use crate::model::db::Database;


pub fn handle_add_note(db: &Database) {
    println!("Entrer le nom de la note :");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Nom invalide !");
    println!("Entrer la description :");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("Description invalide !");

    match db.add_note(&name.trim(), &description.trim()) {
        Ok(_) => println!("Vous venez d'ajouter : {}", name.trim()),
        Err(_) => println!("Une erreur s'est produite pendant l'ajout !"),
    }
}

pub fn handle_delete_note(db: &Database) {
    println!("Entrez l'index à supprimer :");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("Une erreur est survenue");

    match index.trim().parse::<u32>() {
        Ok(num) => match db.delete_note(num) {
            Ok(_) => println!("Supprimé !"),
            Err(e) => println!("Erreur : {}", e),
        },
        Err(e) => println!("Index invalide : {}", e),
    }
}

pub fn handle_view_all_notes(db: &Database) {
    match db.get_all_notes() {
        Ok(list) => {
            for note in list {
                println!("id: {} - title: {}", note.id, note.name.trim());
            }
        }
        Err(e) => println!("Erreur lors de la récupération des notes : {}", e),
    }
}