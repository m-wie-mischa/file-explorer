use rfd::FileDialog;

fn main() {
    let files = FileDialog::new()
        .set_title("Me like Cookie!")
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();
    println!("{:?}", files)
}