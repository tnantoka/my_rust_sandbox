mod index;
mod object;

use index::Entry;
use index::Index;
use object::blob::Blob;
use object::commit;
use object::commit::Commit;
use object::tree;
use object::tree::Tree;
use object::GitObject;

use chrono::{Local, TimeZone, Utc};
use libflate::zlib::{Decoder, Encoder};
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::os::macos::fs::MetadataExt;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let sub_cmd = args.get(1).unwrap().clone();
    match sub_cmd.as_str() {
        "cat-file" => {
            let obj = cat_file_p(args.get(2).unwrap().clone())?;
            println!("{}", obj);
            Ok(())
        }
        "hash-object" => {
            let blob = hash_object(args.get(2).unwrap().clone())?;
            println!("{}", hex::encode(blob.hash()));
            Ok(())
        }
        "ls-files-stage" => {
            let index = read_index().and_then(|x| ls_files_stage(&x))?;
            println!("{}", index);
            Ok(())
        }
        "add" => add(args.get(2).unwrap().clone()),
        "commit" => commit(args.get(2).unwrap().clone()),
        _ => Ok(()),
    }
}

pub fn cat_file_p(hash: String) -> io::Result<GitObject> {
    let (sub_dir, file) = hash.split_at(2);
    let path = format!(".git/objects/{}/{}", sub_dir, file);

    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut d = Decoder::new(&buf[..])?;
    let mut buf = Vec::new();
    d.read_to_end(&mut buf)?;

    GitObject::new(&buf).ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))
}

pub fn hash_object(path: String) -> io::Result<Blob> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    Blob::from_bytes(&buf).ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))
}

pub fn read_index() -> io::Result<Vec<u8>> {
    let path = std::env::current_dir().map(|x| x.join(".git/index"))?;
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    Ok(bytes)
}

pub fn ls_files_stage(bytes: &[u8]) -> io::Result<Index> {
    Index::from_bytes(bytes).ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))
}

pub fn update_index(hash: &[u8], file_name: String) -> io::Result<Index> {
    let bytes = read_index()
        .unwrap_or_else(|_| [*b"DIRC", 0x0002u32.to_be_bytes(), 0x0000u32.to_be_bytes()].concat());
    let index = ls_files_stage(&bytes)?;

    let metadata = std::env::current_dir().and_then(|x| x.join(&file_name).metadata())?;
    let entry = Entry::new(
        Utc.timestamp(metadata.st_ctime(), metadata.st_ctime_nsec() as u32),
        Utc.timestamp(metadata.st_mtime(), metadata.st_mtime_nsec() as u32),
        metadata.st_dev() as u32,
        metadata.st_ino() as u32,
        metadata.st_mode(),
        metadata.st_uid(),
        metadata.st_gid(),
        metadata.st_size() as u32,
        Vec::from(hash),
        file_name,
    );

    let mut entries: Vec<Entry> = index
        .entries
        .into_iter()
        .filter(|x| x.name != entry.name && x.hash != entry.hash)
        .collect();
    entries.push(entry);

    Ok(Index::new(entries))
}

pub fn write_object(object: &GitObject) -> io::Result<()> {
    let hash = hex::encode(object.hash());
    let (sub_dir, file) = hash.split_at(2);

    let path = std::env::current_dir()?;
    let path = path.join(".git/objects").join(sub_dir);

    if path.metadata().is_err() {
        fs::create_dir_all(&path)?;
    }

    let path = path.join(file);

    let mut encoder = Encoder::new(Vec::new())?;
    encoder.write_all(&object.as_bytes())?;
    let bytes = encoder.finish().into_result()?;

    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    file.flush()?;

    Ok(())
}

pub fn write_index(index: &Index) -> io::Result<()> {
    let mut file = File::create(".git/index")?;
    file.write_all(&index.as_bytes())?;
    file.flush()?;

    Ok(())
}

fn add(file_name: String) -> io::Result<()> {
    let path = std::env::current_dir().map(|x| x.join(&file_name))?;
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let blob = Blob::from_bytes(&bytes)
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))
        .map(GitObject::Blob)?;
    write_object(&blob)?;

    let index = update_index(&blob.hash(), file_name)?;
    write_index(&index)?;

    Ok(())
}

pub fn write_tree() -> io::Result<Tree> {
    let bytes = read_index()?;
    let index = ls_files_stage(&bytes)?;

    let contents = index
        .entries
        .iter()
        .map(|x| tree::File::new(100644, x.name.clone(), &x.hash))
        .collect::<Vec<_>>();

    Ok(Tree::new(contents))
}

pub fn commit_tree(
    name: String,
    email: String,
    tree_hash: String,
    message: String,
) -> io::Result<Commit> {
    let parent = head_ref().and_then(read_ref).ok();
    let ts = Utc::now();
    let offset = {
        let local = Local::now();
        *local.offset()
    };
    let author = commit::User::new(
        name,
        email,
        offset.from_utc_datetime(&ts.naive_utc()),
    );
    let commit = Commit::new(tree_hash, parent, author.clone(), author, message);

    Ok(commit)
}

pub fn head_ref() -> io::Result<PathBuf> {
    let path = std::env::current_dir().map(|x| x.join(".git/HEAD"))?;
    let mut file = File::open(path)?;
    let mut refs = String::new();
    file.read_to_string(&mut refs)?;

    let (prefix, path) = refs.split_at(5);

    if prefix != "ref: " {
        return Err(io::Error::from(io::ErrorKind::InvalidData));
    }

    Ok(PathBuf::from(path.trim()))
}

pub fn read_ref(path: PathBuf) -> io::Result<String> {
    let path = std::env::current_dir().map(|x| x.join(".git").join(path))?;
    let mut file = File::open(path)?;
    let mut hash = String::new();
    file.read_to_string(&mut hash)?;

    Ok(hash.trim().to_string())
}

pub fn update_ref(path: PathBuf, hash: &[u8]) -> io::Result<()> {
    write_ref(path, hash)
}

pub fn write_ref(path: PathBuf, hash: &[u8]) -> io::Result<()> {
    let path = std::env::current_dir().map(|x| x.join(".git").join(path))?;
    let mut file = File::create(path)?;
    file.write_all(hex::encode(hash).as_bytes())?;
    file.flush()?;

    Ok(())
}

fn commit(message: String) -> io::Result<()> {
    let tree = write_tree().map(GitObject::Tree)?;
    write_object(&tree)?;

    let tree_hash = tree.hash();
    let commit = commit_tree(
        "tnantoka".to_string(),
        "tnantoka@bornneet.com".to_string(),
        hex::encode(tree_hash),
        message,
    )
    .map(GitObject::Commit)?;
    write_object(&commit)?;

    update_ref(head_ref()?, &commit.hash())?;

    Ok(())
}
