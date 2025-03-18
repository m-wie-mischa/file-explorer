use rfd::FileDialog;

fn main() {
    let files = FileDialog::new()
        .set_title("Select Image")
        .add_filter("image", &["png", "jpg", "jpeg"])
        .set_directory("/")
        .pick_file();
    println!("{:?}", files)
}