use std::time::Duration;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use remarkable_entry::RemarkableEntry;

pub mod remarkable_entry;

pub struct Tablet {
    client: Client,
    address: String,
}

impl Tablet {
    pub fn create(address: &str) -> Self {
        Tablet { client: Client::builder().timeout(Duration::from_secs(120)).build().unwrap(), address: address.to_string() }
    }

    pub fn discover(self: &Tablet, path: &str) -> Result<Vec<RemarkableEntry>, Box<dyn std::error::Error>> {
        let body: String = self.client.post(format!("http://{}/{}", self.address, path))
            .send()?
            .text()?;
        // debug!("{}", body);
        let entries: Vec<Entry> = serde_json::from_str(body.as_str())?;
        let mut remarkable_entries = Vec::new();
        for entry in entries.into_iter() {
            if entry.r#type == "CollectionType" {
                let new_path = format!("{}{}/", path, entry.id);
                let collection = self.discover(new_path.as_str())?;
                remarkable_entries.push(RemarkableEntry::Collection { name: entry.visible_name, entries: collection });
            } else {
                // debug!("{:?}", entry);
                remarkable_entries.push(RemarkableEntry::Item {
                    name: entry.visible_name,
                    id: entry.id,
                    modified: entry.modified_client.parse()?,
                    pages: entry.page_count.unwrap_or_default(),
                    size: entry.size.unwrap_or_default().parse::<u32>()?,
                });
            }
        }
        Ok(remarkable_entries)
    }

    pub fn get(self: &Tablet, content: &RemarkableEntry, full_path: &str) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
        match content.subentry(full_path) {
            Some(RemarkableEntry::Item { id, .. }) =>
                Ok(self.client.get(format!("http://{}/download/{}/placeholder", self.address, id)).send()?.bytes()?),
            _ => panic!("No document found at {}", full_path)
        }
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
    #[serde(rename = "pageCount")]
    page_count: Option<u32>,
    #[serde(rename = "ModifiedClient")]
    modified_client: String,
    #[serde(rename = "sizeInBytes")]
    size: Option<String>,
}
