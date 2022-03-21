use crate::utility::copy_file_contents_to_another_file;

pub fn run() {
    copy_file_contents_to_another_file(
        &String::from("./templates/editorconfig/.editorconfig"),
        &String::from(".editorconfig"),
    )
    .unwrap();
}
