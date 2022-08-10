#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut temp = self.data.clone();
        let read_length = temp.len();
        save_to.reserve(read_length);
        save_to.append(&mut temp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    // from_utf8_lossy_test();
    let mut f5 = File::new("5.txt");
    let mut buffer: Vec<u8> = vec![];
    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);
}

fn from_utf8_lossy_test() {
    /// 유효하지 않은 문자들을 포함하고 있는 bytes slice 자료형을 string으로 변환시키는 함수.
    /// String은 u8 bytes자료형으로 구성되어져있다. bytes slice(&[u8]) 자료형은 bytes로 구성되어져있다.
    /// 이 함수는 위 두개간의 변환을 도와준다.
    /// 문자열들(string)은 유효한 UTF-8이어야 합니다. 하지만 bytes slice자료형 안에는 유효한 문자열들(string)만 있는것이 아니므로
    /// 이 함수(from_utf8_lossy)는 변환하는 중에 유효하지 않은 UTF-8 시퀀스 들을 U+FFFD REPLACECTION CHARACTER(�)로 교체합니다.
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);
    assert_eq!("💖", sparkle_heart);

    let input = b"Hello \xF0\x90\x80World";
    let output = String::from_utf8_lossy(input);
    assert_eq!("Hello �World", output);
}
