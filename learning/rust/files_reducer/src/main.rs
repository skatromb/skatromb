use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/// Random string generator
fn _get_random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Get all file paths
fn get_file_paths(directory: &str) -> impl Iterator<Item = PathBuf> {
    let entries = fs::read_dir(directory).unwrap();

    entries
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
}

/// Make file to write as multithreaded version
fn make_mutex_file(file_name: &str) -> Arc<Mutex<File>> {
    let file = File::options()
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
    const FROM_DIRECTORY: &str = "files_heavy";
    const FILE_NAME: &str = "combined.json";

    let paths = get_file_paths(FROM_DIRECTORY);

    let time = Instant::now();

    {
        let file_to = make_mutex_file(FILE_NAME);

        reduce_in_threads(paths, file_to);
    }

    // Alternative simple version
    // {
    //     let mut file_to = fs::File::options()
    //         .create(true)
    //         .write(true)
    //         .truncate(true)
    //         .open(FILE_NAME)
    //         .unwrap();
    //
    //     for path in paths {
    //         if path.is_file() {
    //             let content = fs::read_to_string(&path).unwrap();
    //             file_to.write_all(content.as_bytes()).unwrap();
    //         }
    //     }
    // }

    println!("Time: {}ms", time.elapsed().as_millis());
}
