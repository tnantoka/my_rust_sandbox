use sha1::{Digest, Sha1};
use std::fmt;

pub struct Tree {
    pub contents: Vec<File>,
}

impl Tree {
    pub fn new(contents: Vec<File>) -> Self {
        Self { contents }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let contents: Vec<File> = Vec::new();
        let mut iter = bytes.split(|&b| b == b'\0');

        let mut header = iter.next()?;
        let contents = iter.try_fold(contents, |mut acc, x| {
            let (hash, next_header) = x.split_at(20);
            let file = File::from_bytes(header, hash)?;

            acc.push(file);
            header = next_header;
            Some(acc)
        })?;

        Some(Self { contents })
    }

    pub fn hash(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        Vec::from(Sha1::digest(&bytes).as_slice())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let content: Vec<u8> = self.contents.iter().flat_map(|x| x.as_bytes()).collect();
        let header = format!("tree {}\0", content.len());

        [header.as_bytes(), content.as_slice()].concat()
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            (&self.contents)
                .iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub struct File {
    pub mode: usize,
    pub name: String,
    pub hash: Vec<u8>,
}

impl File {
    pub fn new(mode: usize, name: String, hash: &[u8]) -> Self {
        Self {
            mode,
            name,
            hash: hash.to_vec(),
        }
    }

    pub fn from_bytes(header: &[u8], hash: &[u8]) -> Option<Self> {
        let split_header = String::from_utf8(header.to_vec()).ok()?;

        let mut iter = split_header.split_whitespace();

        let mode = iter.next().and_then(|x| x.parse::<usize>().ok())?;
        let name = iter.next()?;

        Some(Self::new(mode, String::from(name), hash))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("{} {}\0", self.mode, self.name);
        [header.as_bytes(), &self.hash].concat()
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>06} ??? {}\t{}",
            self.mode,
            hex::encode(&self.hash),
            self.name
        )
    }
}
