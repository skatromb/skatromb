use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

/// Get all file paths
fn get_file_paths(directory: &str) -> impl Iterator<Item = PathBuf> {
    let entries = fs::read_dir(directory).unwrap();

    entries
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
}

/// Make file to write as multithreaded version
fn make_mutex_file(file_name: &str) -> Arc<Mutex<File>> {
    let file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)
        .unwrap();

    Arc::new(Mutex::new(file))
}

/// Read files and write to one file in threads
fn reduce_in_threads(file_paths: impl Iterator<Item = PathBuf>, to_file: Arc<Mutex<File>>) {
    let thread_handles = file_paths.map(|path| {
        let to_file = to_file.clone();

        thread::spawn(move || {
            let contents = fs::read_to_string(path).unwrap();

            to_file
                .lock()
                .unwrap()
                .write_all(contents.as_bytes())
                .unwrap();
        })
    });

    thread_handles
        .into_iter()
        .for_each(|handler| handler.join().unwrap());
}

fn main() {
    let paths = get_file_paths("files");

    let to_file = make_mutex_file("combined.json");

    reduce_in_threads(paths, to_file);

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
