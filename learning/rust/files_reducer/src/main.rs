use std::io::{Write};
use std::thread;
use std::fs;
use std::path::PathBuf;

fn get_file_paths(directory: &str) -> impl Iterator<Item = PathBuf> {
    let entries = fs::read_dir(directory).unwrap();

    entries
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
}

fn read_in_threads(file_paths: impl Iterator<Item = PathBuf>) -> impl Iterator<Item = String> {
    let threaded_contents = file_paths
        .map(|path|
            thread::spawn(||
                fs::read_to_string(path).unwrap()
            )
        );

    let contents = threaded_contents
        .into_iter()
        .map(|handler| handler.join().unwrap());

    contents
}

fn write_file(contents: impl Iterator<Item = String>) {
    let mut file = fs::File::options()
        .create(true).write(true).truncate(true)
        .open("combined.jsonl")
        .unwrap();
    
    contents.for_each(|content| {
        file.write_all(content.as_bytes()).unwrap();
    });
    
}

fn main() {
    let paths = get_file_paths("files");
    
    let contents = read_in_threads(paths);
    
    write_file(contents);
    

    // Alternative simple version
    // for entry in entries {
    //     let entry = entry?;
    //     let path = entry.path();
    //
    //     if path.is_file() {
    //         let content = fs::read_to_string(&path)?;
    //         file.write_all(content.as_bytes())?;
    //     }
    // }
}
