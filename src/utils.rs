use std::fs::File;
use std::io::Write;

pub fn write_to_file(path: &str, content: &str) {
    let mut of = File::create(path).expect("making file brokie somehow");
    write!(of, "{content}").expect("writing to file brokie somehow");
}

pub fn make_dir(path: &str) {
    std::fs::create_dir(path).expect("Couldn't create dir :( brokie");
}

pub fn _cwd() -> String {
    std::env::current_dir().expect("brokie, getting cwd brokie (somehow)").into_os_string().into_string().expect("text brokie i guess???")
}
