use std::fs::File;
use std::io::Write;
use chrono::DateTime;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(short, long, action, default_value_t = String::from("10.11.99.1"), value_parser)]
    address: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check connection to reMarkable
    Status {},
    /// List files on reMarkable
    List {
        #[clap(short, long, action)]
        recursive: bool,
        #[clap(default_value_t = String::from("documents/"), value_parser)]
        path: String,
    },
    /// Synchronize rendered files from reMarkable
    Sync {},
    /// Store a file on the reMarkable
    Add {},
    /// Retrieve a file from the reMarkable
    Get {
        file_name: String,
        id: String,
    },
    /// Delete a file from the reMarkable
    Remove {},
    /// Backup the reMarkable content to an archive
    Backup {},
    /// Erase the reMarkable content
    Erase {},
    /// Restore the reMarkable content from an archive
    Restore {},
    /// Install a template
    Template {},
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let result = match &cli.command {
        Commands::Status { } => todo!(),
        Commands::List { recursive, path } =>
            list(*recursive, &cli.address, path, &String::new()),
        Commands::Sync { } => todo!(),
        Commands::Add { } => todo!(),
        Commands::Get { file_name, id } => get(&cli.address, file_name, id),
        Commands::Remove { } => todo!(),
        Commands::Backup { } => todo!(),
        Commands::Erase { } => todo!(),
        Commands::Restore { } => todo!(),
        Commands::Template { } => todo!(),
    };

    if let Err(msg) = result {
        println!("Error: {}", msg);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    #[serde(rename = "VissibleName")]
    visible_name: String,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "fileType")]
    file_type: Option<String>,
//    pageCount: Option<u32>,
    #[serde(rename = "ModifiedClient")]
    modified_client: String,
//    sizeInBytes: String,
}

enum _RemarkableEntry {
    Collection {
        name: String,
        id: String,
    },
    Item {
        name: String,
        id: String,
    },
}

fn list(recursive: bool, address: &String, path: &String, readable_path: &String) -> Result<(), String> {
    let client = Client::new();
    let body : String = client.post(format!("http://{}/{}", address, path))
        .send().map_err(|e| e.to_string())?
        .text().map_err(|e| e.to_string())?;
    let entries : Vec<Entry> = serde_json::from_str(body.as_str()).map_err(|e| e.to_string())?;
    for entry in entries.iter() {
        if recursive && entry.r#type == "CollectionType" {
            let new_path = format!("{}{}/", path, entry.id);
            let new_readable_path = format!("{}{}/", readable_path, entry.visible_name);
            list(recursive, address, &new_path, &new_readable_path)?;
        } else {
            let rfc3339 = DateTime::parse_from_rfc3339(entry.modified_client.as_str()).map_err(|e| e.to_string())?;
            println!("{} /{}{}{}  -> {}", rfc3339.format(" \u{1F4C5}%Y-%m-%d \u{1F551}%H:%M:%S"), readable_path, entry.visible_name, if entry.r#type == "CollectionType" { "/" } else { "" }, entry.id);
        }
    }
    Ok(())
}

fn get(address: &String, file_name: &String, id: &String) -> Result<(), String> {
    // http://10.11.99.1/download/aa889098-2e7f-45f8-b8de-be4dbb964aa7/placeholder
    let client = Client::new();
    let response = client.get(format!("http://{}/download/{}/placeholder", address, id))
        .send().map_err(|e| e.to_string())?;
    let body = response.bytes().map_err(|e| e.to_string())?;

    let mut file = File::create(file_name).map_err(|e| e.to_string())?;
    file.write_all(body.as_ref()).map_err(|e| e.to_string())?;
    Ok(())
}