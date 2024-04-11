// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Import des modules
use serde::{Deserialize, Serialize};
use std::fs;
use std::string::String;

use std::path::Path;


// Structure pour représenter les informations sur une application
#[derive(Debug, Serialize, Deserialize)]
struct AppInfo {
    foldername: String,
    downloadlink: String,
    app_name: String,
}

fn init(_t:String)->String {
    // Vérifier si le fichier app.json existe localement
    let app_json_path = "app.json";
    if !Path::new(app_json_path).exists() {
        // Si le fichier n'existe pas localement, le récupérer depuis le serveur web
        let server_url = "https://example.com/app.json"; // Remplacez ceci par l'URL réelle de votre serveur
        let _download_result = Command::new("curl")
        .arg("-o")
        .arg(&app_json_path)
        .arg(&server_url)
        .output();
    }

    // Lire le contenu du fichier app.json
    let json_data = fs::read_to_string(app_json_path).expect("Impossible de lire le fichier app.json");

    // Désérialisation du JSON en une liste d'informations sur les applications
    let apps: Vec<AppInfo> = serde_json::from_str(&json_data).expect("Erreur lors de la désérialisation du JSON");

    // Pour chaque application, vérifier si elle est déjà installée, puis la télécharger et l'extraire si nécessaire
    for app in &apps {
        // Vérifier si le dossier de l'application existe déjà
        if !Path::new(&app.foldername).exists() {
            // Créer le chemin du fichier téléchargé
            let zip_file_path = format!("{}.zip", &app.foldername);

            // Télécharger le fichier
            let download_result = Command::new("curl")
                .arg("-o")
                .arg(&zip_file_path)
                .arg(&app.downloadlink)
                .output();

            if let Ok(output) = download_result {
                if output.status.success() {
                    println!("Téléchargement réussi de {}", &app.app_name);

                    // Extraire le fichier zip dans le dossier approprié
                    let extract_result = Command::new("unzip")
                        .arg(&zip_file_path)
                        .output();

                    if let Ok(output) = extract_result {
                        if output.status.success() {
                            println!("Extraction réussie de {}", &app.app_name);
                        } else {
                            eprintln!("Erreur lors de l'extraction de {}", &app.app_name);
                        }
                    } else {
                        eprintln!("Erreur lors de l'exécution de la commande d'extraction");
                    }

                    // Supprimer le fichier zip après extraction
                    if let Err(err) = fs::remove_file(&zip_file_path) {
                        eprintln!("Erreur lors de la suppression du fichier zip : {:?}", err);
                    }
                } else {
                    eprintln!("Erreur lors du téléchargement de {}", &app.app_name);
                }
            } else {
                eprintln!("Erreur lors de l'exécution de la commande de téléchargement");
            }
        } else {
            println!("L'application {} est déjà installée.", &app.app_name);
        }
    }
    return json_data
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn install(t:String)->String {
    return init(t);
}
// Import des modules
use std::process::{Command, Output};
#[tauri::command]


fn executor(cmde: &str, arg: &str) -> String {
    println!("Command: {}", cmde);

    let output: Result<Output, _> = Command::new(cmde)
        .arg(arg)
        .output();

    match output {
        Ok(output) => {
            // Affiche la sortie de la commande
            if output.status.success() {
                // Convertir la sortie en String
                let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                return stdout;
            } else {
                // Si la commande échoue, retourne stderr
                let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                return stderr;
            }
        }
        Err(err) => {
            // Si une erreur se produit lors de l'exécution de la commande
            return format!("Erreur: {}", err);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,install,executor])
        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
