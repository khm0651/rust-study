use std::fmt::Result;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum FileState {
    Open,
    Close,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FileState::Open => {
                write!(f, "OPEN")
            }
            FileState::Close => {
                write!(f, "CLOSE")
            }
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Close,
        }
    }
}

fn main() {
    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
}
