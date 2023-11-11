use std::fs::DirEntry;

pub struct FSObjectIdentity
{
    pub name: String,
    pub path: String
}

impl FSObjectIdentity
{
    pub fn from_dir_entry(entry: &DirEntry) -> Self
    {
        FSObjectIdentity {
            name: entry.file_name().into_string().unwrap(),
            path: entry.path().into_os_string().into_string().unwrap()
        }
    }
}