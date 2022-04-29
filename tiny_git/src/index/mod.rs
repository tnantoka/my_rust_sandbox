use chrono::{DateTime, TimeZone, Utc};
use sha1::{Digest, Sha1};
use std::fmt;

pub struct Index {
    pub entries: Vec<Entry>,
}

impl Index {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self { entries }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if &bytes[0..4] != b"DIRC" {
            return None;
        }

        if hex_to_num(&bytes[4..8]) != 2 {
            return None;
        }

        let entry_num = hex_to_num(&bytes[8..12]);
        let entries = (0..entry_num)
            .try_fold((0, Vec::new()), |(offs, mut vec), _| {
                let entry = Entry::from_bytes(&bytes[(12 + offs)..])?;
                let size = entry.size();
                vec.push(entry);
                Some((offs + size, vec))
            })
            .map(|(_, entries)| entries)?;

        Some(Self::new(entries))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header = [
            *b"DIRC",
            [0x00, 0x00, 0x00, 0x02],
            (self.entries.len() as u32).to_be_bytes(),
        ]
        .concat();

        let entries = self
            .entries
            .iter()
            .flat_map(|x| x.as_bytes())
            .collect::<Vec<_>>();

        let content = [header, entries].concat();
        let hash = Vec::from(Sha1::digest(&content).as_slice());

        [content, hash].concat()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.entries.iter().try_for_each(|e| writeln!(f, "{}", e))
    }
}

pub struct Entry {
    pub c_time: DateTime<Utc>,
    pub m_time: DateTime<Utc>,
    pub dev: u32,
    pub inode: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u32,
    pub hash: Vec<u8>,
    pub name: String,
}

impl Entry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        c_time: DateTime<Utc>,
        m_time: DateTime<Utc>,
        dev: u32,
        inode: u32,
        mode: u32,
        uid: u32,
        gid: u32,
        size: u32,
        hash: Vec<u8>,
        name: String,
    ) -> Self {
        Self {
            c_time,
            m_time,
            dev,
            inode,
            mode,
            uid,
            gid,
            size,
            hash,
            name,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let c_time = hex_to_num(&bytes[0..4]);
        let c_time_nano = hex_to_num(&bytes[4..8]);
        let m_time = hex_to_num(&bytes[8..12]);
        let m_time_nano = hex_to_num(&bytes[12..16]);
        let dev = hex_to_num(&bytes[16..20]);
        let inode = hex_to_num(&bytes[20..24]);
        let mode = hex_to_num(&bytes[24..28]);
        let uid = hex_to_num(&bytes[28..32]);
        let gid = hex_to_num(&bytes[32..36]);
        let size = hex_to_num(&bytes[36..40]);
        let hash = Vec::from(&bytes[40..60]);
        let name_size = hex_to_num(&bytes[60..62]);
        let name = String::from_utf8(Vec::from(&bytes[62..(62 + name_size as usize)])).ok()?;

        let entry = Self {
            c_time: Utc.timestamp(c_time.into(), c_time_nano),
            m_time: Utc.timestamp(m_time.into(), m_time_nano),
            dev,
            inode,
            mode,
            uid,
            gid,
            size,
            hash,
            name,
        };

        Some(entry)
    }

    pub fn size(&self) -> usize {
        let size = 62 + self.name.len();
        size + (8 - size % 8)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let ctime = self.c_time.timestamp() as u32;
        let ctime_nano = self.c_time.timestamp_subsec_nanos();
        let mtime = self.m_time.timestamp() as u32;
        let mtime_nano = self.m_time.timestamp_subsec_nanos();

        let meta = [
            ctime, ctime_nano, mtime, mtime_nano, self.dev, self.inode, self.mode, self.uid,
            self.gid, self.size,
        ]
        .iter()
        .flat_map(|&x| Vec::from(x.to_be_bytes()))
        .collect::<Vec<_>>();

        let name_size = self.name.len() as u16;
        let name = self.name.as_bytes();

        let len = 62 + name_size as usize;

        let padding = (0..(8 - len % 8)).map(|_| b'\0').collect::<Vec<u8>>();

        [
            meta,
            self.hash.clone(),
            Vec::from(name_size.to_be_bytes()),
            name.to_vec(),
            padding,
        ]
        .concat()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} 0\t{}",
            num_to_mode(self.mode as u16),
            hex::encode(&self.hash),
            self.name
        )
    }
}

fn hex_to_num(hex: &[u8]) -> u32 {
    hex.iter()
        .rev()
        .fold((0u32, 1u32), |(sum, offs), &x| {
            (sum + (x as u32 * offs), offs << 8)
        })
        .0
}

fn num_to_mode(val: u16) -> String {
    let file_type = val >> 13;
    let (user, group, other) = {
        let permission = val & 0x01ff;
        let user = (permission & 0x01c0) >> 6;
        let group = (permission & 0x0038) >> 3;
        let other = permission & 0x0007;

        (user, group, other)
    };

    format!("{:03b}{}{}{}", file_type, user, group, other)
}
