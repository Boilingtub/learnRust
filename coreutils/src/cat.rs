use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        for a in 1..args.len() {
            println!("{}",cat(&args[a])); 
        }
    }
    else {
        println!("no arguments");
        let file_to_open = usr_input();
        println!("{}",cat(&file_to_open));
    }
}

fn usr_input() -> String {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    input_text.trim().to_string()
}

fn cat(file_path:&str) -> String {
    let path = Path::new(file_path);

    if path.exists() {
        fs::read_to_string(file_path).expect("Can read file")
    }
    else {
            println!("File Does not exist!");
            "".to_string()
    }    
}

#[allow(dead_code)]
use std::io::prelude::*;
fn create_file(file_path:&str, write_content:&str) -> std::io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(write_content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_file(file_path:&str, write_content:&str) -> std::io::Result<()> {
        let mut file = fs::File::create(file_path)?;
        file.write_all(write_content.as_bytes())?;
        Ok(())
    }


    #[test]
    fn cat_test() {
        let _ = create_file("cat_test_file","This\nFile contains\nSome Cat test");
        let cat_result = cat("cat_test_file");
        assert_eq!(cat_result,"This\nFile contains\nSome Cat test");
    }
}
