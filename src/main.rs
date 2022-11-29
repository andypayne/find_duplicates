use std::{fs, io};
use std::collections::HashMap;
use walkdir::WalkDir;
use md5::{Md5, Digest};
use base16ct;
use clap::Parser;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::json;


pub struct FileEntry {
    pub size: u64,
    pub files: Vec<String>,
}


impl FileEntry {
    fn new() -> FileEntry {
        FileEntry {
            size: 0,
            files: Vec::new(),
        }
    }
}


impl Serialize for FileEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FileEntry", 3)?;
        s.serialize_field("size", &self.size)?;
        s.serialize_field("files", &self.files)?;
        s.end()
    }
}


#[derive(Parser, Default, Debug)]
#[clap(about="Find duplicate files in a directory tree.")]
struct Args {
    /// Output json to a file
    #[arg(short = 'j', long = "json")]
    json_file: Option<String>,
    /// Include all files (instead of only duplicates)
    #[arg(short = 'a', long = "all")]
    include_all: bool,
    /// The base path to search
    path: String,
}


/// Return the MD5 digest of a file as a string.
fn hash_file(entry: &walkdir::DirEntry) -> std::io::Result<String> {
    let mut file = fs::File::open(entry.path())?;
    let mut hasher = Md5::new();
    let _n = io::copy(&mut file, &mut hasher);
    let hash = hasher.finalize();
    let mut buf = [0u8; 32];
    let hex_hash: &str = base16ct::lower::encode_str(&hash, &mut buf).unwrap();
    return Ok(hex_hash.to_string());
}


/// `pluralize` is an extremely naive pluralizer.
fn pluralize(count: usize) -> String {
    match count {
        1 => return String::from(""),
        _ => return String::from("s")
    }
}


fn output_results(
    files: HashMap<String, FileEntry>,
    json_file: Option<String>,
    include_all: bool
) {
    let total_count = files.len();
    let dup_files: HashMap<String, FileEntry> = files.into_iter()
        .filter(|(_, v)| include_all || v.files.len() > 1)
        .collect();
    println!("Total files: {}", total_count);
    if !include_all {
        println!("Duplicate entries: {}", dup_files.len());
    }
    match json_file {
        Some(json_file_p) => {
            println!("JSON output - {}", json_file_p);
            let files_json = json!(dup_files);
            fs::write(
                json_file_p,
                files_json.to_string()
            ).unwrap();
        },
        None => {
            for (hash, file_entry) in &dup_files {
                println!("{}: {} bytes, {} instance{}",
                    hash,
                    file_entry.size,
                    file_entry.files.len(),
                    pluralize(file_entry.files.len())
                );
                for file in &file_entry.files {
                    println!("- {}", file);
                }
            }
        },
    };
}


fn main() -> std::io::Result<()> {
    let args = Args::parse();
    //println!("path: {:?}", args.path);
    //println!("json_file: {:?}", args.json_file);
    //println!("include_all: {:?}", args.include_all);

    let mut file_entries: HashMap<String, FileEntry> = HashMap::new();
    for entry in WalkDir::new(args.path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let hash_res = hash_file(&entry);
        let hash = match hash_res {
            Ok(hash_str) => hash_str,
            Err(error) => panic!("Error hashing: {:?}", error),
        };
        let file_path = entry.path().to_string_lossy();
        let file_size: u64 = fs::metadata(file_path.to_string())?.len();
        let fe = file_entries.entry(hash).or_insert_with(|| FileEntry::new());
        fe.size = file_size;
        fe.files.push(file_path.to_string());
    }

    output_results(file_entries, args.json_file, args.include_all);
    Ok(())
}
