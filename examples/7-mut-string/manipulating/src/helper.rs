pub fn longest(sentence: &str) -> String {
    let mut longest = "";
    for word in sentence.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest.to_string()
}