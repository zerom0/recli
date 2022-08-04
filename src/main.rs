use std::fs;

use chrono::prelude::*;
use clap::{Parser, Subcommand};
use log::info;

use tablet::remarkable_entry::RemarkableEntry;

use crate::tablet::Tablet;

mod tablet;

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
    /// List files on reMarkable
    List {
        #[clap(short, long, action)]
        recursive: bool,
        #[clap(default_value_t = String::new(), value_parser)]
        path: String,
    },
    /// Retrieve a document rendered as PDF file from the reMarkable
    Get {
        /// Fully qualified path to the document
        path: String,
    },
    /// Synchronize rendered files from reMarkable
    Sync {
        #[clap(default_value_t = String::new(), value_parser)]
        path: String,
    },
    /// Store a PDF document on the reMarkable
    Add {},
    /// Delete a file from the reMarkable
    Remove {},
    /// Backup the reMarkable content to an archive
    Backup {},
    /// Restore the reMarkable content from an archive
    Restore {},
    /// Erase the reMarkable content
    Erase {},
    /// Install a template
    Template {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();

    let t = tablet::Tablet::create(&cli.address);
    let remarkable_content = RemarkableEntry::Collection { name: ".".to_string(), entries: t.discover("documents/")? };

    match &cli.command {
        Commands::List { recursive, path } =>
            list(remarkable_content.subentry(path).ok_or(String::from("Path not found"))?, path.as_str(), *recursive),
        Commands::Get { path } =>
            get(&t, &remarkable_content, "", path.as_str()),
        Commands::Sync { path } =>
            recursive_get(&t, remarkable_content.subentry(path).ok_or(String::from("Path not found"))?, path.as_str()),
        Commands::Add {} =>
            todo!(),
        Commands::Remove {} =>
            todo!(),
        Commands::Backup {} =>
            todo!(),
        Commands::Restore {} =>
            todo!(),
        Commands::Erase {} =>
            todo!(),
        Commands::Template {} =>
            todo!(),
    }
}

fn list(entries: &RemarkableEntry, prefix: &str, recursive: bool) -> Result<(), Box<dyn std::error::Error>> {
    if let RemarkableEntry::Collection { name: _, entries } = entries {
        if entries.is_empty() {
            println!("{}/", prefix);
        } else {
            for entry in entries {
                match entry {
                    RemarkableEntry::Item { name, modified, .. } => println!("{}/{} {} [pdf]", prefix, name, modified),
                    RemarkableEntry::Collection { name, .. } =>
                        if recursive { list(entry, format!("{}/{}", prefix, name).as_str(), recursive)? } else { println!("{}/{}/", prefix, name) },
                }
            }
        }
    }
    Ok(())
}

fn get(tablet: &Tablet, content: &RemarkableEntry, prefix: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    match content.subentry(path) {
        Some(RemarkableEntry::Item { modified, pages, .. }) => {
            let local_file_path = format!("/{}/{}.pdf", prefix, path).trim_start_matches('/').to_string();
            eprint!(" * {}", local_file_path);

            // let local_file_path = format!("{}.pdf", name);
            if version_exists(&local_file_path, modified) {
                eprintln!(" [unmodified]");
                return Ok(());
            }

            eprint!(" (fetching {} pages ..)", pages);
            let start = Utc::now();
            let document = tablet.get(content, path)?;
            fs::create_dir_all(local_file_path.rsplit_once('/').unwrap_or(("", "")).0)?;
            fs::write(local_file_path, document.as_ref())?;
            let duration = Utc::now().signed_duration_since(start);
            eprintln!(" [done]");
            info!("Download ({} Bytes) took {} ms", document.len(), duration.num_milliseconds());
        }
        Some(RemarkableEntry::Collection { .. }) => {
            panic!("Cannot get a collection");
        }
        _ => {
            panic!("No such file");
        }
    }

    Ok(())
}


/// Tests if the local file exists and is newer or the same age as the timestamp
fn version_exists(local_file_path: &str, timestamp: &DateTime<Utc>) -> bool {
    if let Ok(metadata) = fs::metadata(&local_file_path) {
        if let Ok(modified) = metadata.modified() {
            let local_timestamp: DateTime<Utc> = modified.into();
            *timestamp <= local_timestamp
        } else {
            false
        }
    } else {
        false
    }
}

fn recursive_get(tablet: &Tablet, content: &RemarkableEntry, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let RemarkableEntry::Collection { name: _, entries } = content {
        for entry in entries {
            // debug!("{:?}", entry);
            match entry {
                RemarkableEntry::Item { name, .. } => get(tablet, content, prefix, name)?,
                RemarkableEntry::Collection { name, .. } =>
                    recursive_get(tablet, entry, format!("{}/{}", prefix, name).as_str())?,
            }
        }
    }
    Ok(())
}

