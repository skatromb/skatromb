use std::io::{Write};
use std::thread;
use std::fs;

fn main() {
    let entries = fs::read_dir("files").unwrap();

    let paths = entries
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file());

    let threaded_contents = paths
        .map(|path|
            thread::spawn(||
                fs::read_to_string(path).unwrap()
            )
        );

    let contents = threaded_contents
        .into_iter()
        .map(|handler| handler.join().unwrap());

    let mut file = fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open("combined.jsonl")
        .unwrap();

    contents.for_each(|content| {
        file.write_all(content.as_bytes()).unwrap();
    });

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
