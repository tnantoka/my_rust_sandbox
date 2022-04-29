pub mod blob;
pub mod commit;
pub mod tree;

use blob::Blob;
use commit::Commit;
use tree::Tree;

use std::fmt;

pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl GitObject {
    pub fn new(bytes: &[u8]) -> Option<Self> {
        let mut iter = bytes.splitn(2, |&byte| byte == b'\0');

        let obj_type = iter
            .next()
            .and_then(|x| String::from_utf8(x.to_vec()).ok())
            .and_then(|x| ObjectType::from(&x))?;

        match obj_type {
            ObjectType::Blob => Blob::from_bytes(bytes).map(GitObject::Blob),
            ObjectType::Tree => Tree::from_bytes(bytes).map(GitObject::Tree),
            ObjectType::Commit => Commit::from_bytes(bytes).map(GitObject::Commit),
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        match self {
            Self::Blob(obj) => obj.hash(),
            Self::Tree(obj) => obj.hash(),
            Self::Commit(obj) => obj.hash(),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Blob(obj) => obj.as_bytes(),
            Self::Tree(obj) => obj.as_bytes(),
            Self::Commit(obj) => obj.as_bytes(),
        }
    }
}

impl fmt::Display for GitObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Blob(obj) => obj.fmt(f),
            Self::Tree(obj) => obj.fmt(f),
            Self::Commit(obj) => obj.fmt(f),
        }
    }
}

pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl ObjectType {
    pub fn from(s: &str) -> Option<Self> {
        let mut header = s.split_whitespace();

        match header.next()? {
            "blob" => Some(ObjectType::Blob),
            "tree" => Some(ObjectType::Tree),
            "commit" => Some(ObjectType::Commit),
            _ => None,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    #[allow(clippy::inherent_to_string)]
    #[allow(dead_code)]
    pub fn to_string(self) -> String {
        match self {
            ObjectType::Blob => String::from("blob"),
            ObjectType::Tree => String::from("tree"),
            ObjectType::Commit => String::from("commit"),
        }
    }
}
