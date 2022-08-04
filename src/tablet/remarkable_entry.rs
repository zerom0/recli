use chrono::prelude::*;

#[derive(Debug)]
pub enum RemarkableEntry {
    Collection {
        name: String,
        entries: Vec<RemarkableEntry>,
    },
    Item {
        name: String,
        id: String,
        modified: DateTime<Utc>,
        pages: u32,
        size: u32,
    },
}

impl RemarkableEntry {
    /// Retrieve the entry identified by path.
    ///
    /// # Arguments
    ///
    /// * `path`: Path to the entry relative to the entry the function is called on.
    ///
    /// returns: &RemarkableEntry
    ///
    pub fn subentry(self: &RemarkableEntry, path: &str) -> Option<&RemarkableEntry> {
        let (term, remainder) = match path.split_once('/') {
            Some((f, r)) => (f, r),
            None => (path, ""),
        };

        if term.is_empty() {
            return Some(self);
        }

        if let RemarkableEntry::Collection { name: _, entries } = self {
            for entry in entries {
                if remainder.is_empty() {
                    match entry {
                        RemarkableEntry::Item { name, .. } => if name == term { return Some(entry); },
                        RemarkableEntry::Collection { name, .. } => if name == term { return Some(entry); },
                    }
                } else {
                    match entry {
                        RemarkableEntry::Item { name, .. } =>
                            if name == term { return None; },
                        RemarkableEntry::Collection { name, .. } =>
                            if name == term { return entry.subentry(remainder); },
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn subentry_of_empty_collection() {
        let without_entries = RemarkableEntry::Collection { name: "Documents".to_string(), entries: Vec::new() };
        assert!(matches!(without_entries.subentry(""), Some(RemarkableEntry::Collection { name, entries: _ }) if name == "Documents"));
    }

    #[test]
    fn subentry_of_collection() {
        let entries = RemarkableEntry::Collection {
            name: "Documents".to_string(),
            entries: vec![
                RemarkableEntry::Item { name: "Notes".to_string(), id: "0815".to_string(), modified: "".to_string() },
                RemarkableEntry::Collection { name: "EmptyCollection".to_string(), entries: Vec::new() },
                RemarkableEntry::Collection {
                    name: "Collection".to_string(),
                    entries: vec![
                        RemarkableEntry::Item { name: "Project".to_string(), id: "4711".to_string(), modified: "".to_string() },
                    ],
                },
            ],
        };
        assert!(matches!(entries.subentry(""), Some(RemarkableEntry::Collection { name, entries: _ }) if name == "Documents"));
        assert!(matches!(entries.subentry("/"), Some(RemarkableEntry::Collection { name, entries: _ }) if name == "Documents"));
        assert!(matches!(entries.subentry("Notes"), Some(RemarkableEntry::Item {name: _, id, modified: _}) if id == "0815"));
        assert!(matches!(entries.subentry("Notes/"), Some(RemarkableEntry::Item {name: _, id, modified: _}) if id == "0815"));
        assert!(matches!(entries.subentry("Notes/Notes"), None));
        assert!(matches!(entries.subentry("EmptyCollection"), Some(RemarkableEntry::Collection {name, entries: _}) if name == "EmptyCollection"));
        assert!(matches!(entries.subentry("Collection"), Some(RemarkableEntry::Collection {name, entries: _}) if name == "Collection"));
        assert!(matches!(entries.subentry("Collection/"), Some(RemarkableEntry::Collection {name, entries: _}) if name == "Collection"));
        assert!(matches!(entries.subentry("Collection/Project"), Some(RemarkableEntry::Item {name: _, id, modified: _}) if id == "4711"));
    }
}