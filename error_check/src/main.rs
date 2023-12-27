use std::fs::File;
use std::io::Read;

fn main() {}

fn read_file(path: &str) -> Result<String, String> {
    match File::open(path).map_err(|err| format!("error open file {}", err)) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file
                .read_to_string(&mut contents)
                .map_err(|err| format!("error reading file {}", err))
            {
                Ok(_) => Ok(contents),
                Err(e) => return Err(e),
            }
        }
        Err(e) => return Err(e),
    }
}
