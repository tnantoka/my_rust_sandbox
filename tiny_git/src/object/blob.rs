use sha1::{Digest, Sha1};
use std::fmt;

pub struct Blob {
    pub size: usize,
    pub content: String,
}

impl Blob {
    pub fn new(content: String) -> Self {
        Self {
            size: content.len(),
            content,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let content = String::from_utf8(bytes.to_vec());

        match content {
            Ok(content) => Some(Self::new(content)),
            Err(_) => None,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("blob {}\0", self.size);
        let store = format!("{}{}", header, self);

        Vec::from(store.as_bytes())
    }

    pub fn hash(&self) -> Vec<u8> {
        Vec::from(Sha1::digest(&self.as_bytes()).as_slice())
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
