extern crate walkdir;
extern crate colored;
extern crate argparse;
extern crate fstream;

use colored::*;
use walkdir::{WalkDir, DirEntry};
use std::path::Path;
use argparse::{ArgumentParser, Store};
use std::ffi::OsStr;



fn main() {
    let pyroot: &Path = Path::new("/Users/a.poulton/miniconda3/lib/python3.7/site-packages");
    let mut module = ".".to_string();
    let mut query = "query".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Recursive string locater in files");
        ap.refer(&mut module)
            .add_option(&["-m", "--module"], Store, "Python module to search")
            .required();
        ap.refer(&mut query)
            .add_option(&["-q", "--query"], Store, "Query string to find")
            .required();
        ap.parse_args_or_exit();
    }
    let filepath = pyroot.join(module);
    let filepath = filepath.as_path().to_str().unwrap();
    list_files(&filepath, &query);
}

// fn is_py_file(entry: DirEntry)-> bool{
//     let is_py = entry.file_name()
//         .to_str()
//         .map(|s| s.ends_with(".py"))
//         .unwrap_or(false);
//     println!("{:?}   {:?}", entry.path(), is_py);
//     is_py
// }

fn list_files(path: &str, query: &str)-> (){
    // let mut files = Vec::new();
    for (_f_no, file) in WalkDir::new(path)
        .into_iter()
        .enumerate(){
            let filename: &DirEntry = &file.unwrap();
            let filename: &Path = filename.path();
            match filename.extension() {
                Some(name) => {
                    if name == "py"{
                        search_file(&filename, query)
                    }
                }
                None => ()
                
            }

        }
}

fn search_file(path: &Path, query: &str)->(){
    match fstream::contains(path, query) {
        Some(b) => {
            if b {
                match fstream::read_lines(path) {
                    Some(lines) => {
                        for (_pos, line) in &mut lines.iter().enumerate(){
                            if line.contains(query){
                                if line.contains("def "){
                                    println!("{}:{}", path.to_str().unwrap().red(), _pos.to_string().red());
                                    break;
                                } else{
                                    println!("{}:{}", path.to_str().unwrap(), (_pos+1).to_string());
                                }
                                
                            }
                        }
                    }
                    None => println!("Error reading file")

                }
            }
        }
        None => println!("Error searching file: {}", path.to_str().unwrap())
    }

}
