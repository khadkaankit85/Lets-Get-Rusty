#[allow(unused_variables)]
#[allow(dead_code)]
pub fn slices_in_rust() {
    let sentence = String::from("Humans are stupid creatures");
    let first_word = get_first_word(&sentence);
    let name = "Angkit";
}
fn get_first_word(sentence: &String) -> usize {
    let bytes = sentence.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    sentence.len()
}
