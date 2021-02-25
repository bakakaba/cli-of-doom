use std::fs;

pub fn list_files() {
    for path in fs::read_dir(".").unwrap() {
        let meta = fs::metadata(path.unwrap().path());
        println!("{:?}", meta.unwrap());
    }
}
