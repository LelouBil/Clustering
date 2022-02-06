use std::cmp::Ordering;
use std::collections::{BTreeSet};
use std::fmt::{Debug, Display, Formatter, write};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;

pub fn load_file(data: &str) -> std::io::Result<Vec<Instance>> {
    let mut nosqldata = File::open(data)?;

    let mut buf = String::new();
    nosqldata.read_to_string(&mut buf)?;

    Ok(buf.lines()
        .enumerate()
        .map(|(id,line)| {
            Instance {
                proprietes: line.trim()
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect(),
                id
            }
        }).collect())
}


#[derive(Debug)]
pub struct Instance {
    pub proprietes: BTreeSet<String>,
    id : usize,
}

impl Display for Instance{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"[{}]",self.id);
        self.proprietes.fmt(f)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Instance {}
impl Hash for Instance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}


