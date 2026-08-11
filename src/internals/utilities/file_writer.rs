use std::io::Write;

pub fn write_to_file_os(content: String, file_path: &str) {
    let check_file = std::fs::metadata(file_path);
    let locate_file = match check_file {
        Ok(_) => {
            std::fs::File::create(file_path)
        }
        Err(_) => {
            println!("File already exists, only rewriting");
            std::fs::File::create_new(file_path)
        }
    };
    match locate_file {
        Ok(mut file) => {
            match  file.write_all(content.as_bytes()) {
                Ok(_) => {
                    println!("Success to write!")
                },
                Err(err) => {
                    println!("Error ar writting to location : {:?}",err)
                }
            }
        }
        Err(err) => {
            println!("Error at writting to file Log : {:?}",err)
        }
    }
}
