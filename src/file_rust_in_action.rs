//! Simulting files one step at a time.

/// State of file
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

static mut ERRNO: i32 = 0;

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

/// A dummy type representing file
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl File {
    fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        Self {
            name: String::from(name),
            data: data.clone(),
            state: FileState::Closed,
        }
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            Err(format!("File must be opened for reading: {}", self.name))
        } else {
            let mut tmp = self.data.clone();
            let tmp_len = tmp.len();
            save_to.reserve(tmp_len);
            save_to.append(&mut tmp);
            Ok(tmp_len)
        }
    }
}

fn open(mut file: File) -> Result<File, String> {
    file.state = FileState::Open;
    Ok(file)
}

fn close(mut file: File) -> Result<File, String> {
    file.state = FileState::Closed;
    Ok(file)
}

fn main() {
    let f_data = vec![114, 117, 115, 116, 33];
    let f = File::new_with_data("f.txt", &f_data);
    let mut buf: Vec<u8> = vec![];

    let f = open(f).unwrap();
    unsafe {
        if ERRNO != 0 {
            panic!("Error occurred while reading the file.");
        }
    }

    let read_len = f.read(&mut buf).unwrap();
    unsafe {
        if ERRNO != 0 {
            panic!("Error occurred while reading the file.");
        }
    }

    close(f).unwrap();
    unsafe {
        if ERRNO != 0 {
            panic!("Error occurred while closing the file.");
        }
    }
    
    let text = String::from_utf8_lossy(&buf);
    println!("Read {} bytes from file.", read_len);
    println!("{}", text);
}