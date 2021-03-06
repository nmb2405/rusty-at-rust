/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/

fn main() {
    let test_str1 = String::from("did you ever hear the tragedy of darth plagueis the wise");
    println!("{}", piglatin(&test_str1[..]));
}
fn convert(word: &str) -> String {
    /*
    Main function which converts each word to piglatin
    */
    let first_letter = word.chars().nth(0);
    match first_letter {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => {
            format!("{}-{} ", word, "hay")
        },
        Some(c) => {
            /*
            I was wondering why the first line below did not work and why the second line did. After asking on discord, I was told that,
            rust implicitly borrows (does not move). It does so via match foo { ref bar => ... } which requires that the foo (word[1..] here) be Sized.
            To explicitly borrow we have to use &
    
    
            My next question was why would rust not know the size of the slice at compile time to which someone answered,
            that one could specify the length it needs to be sliced to at runtime which makes sense.
            */
            format!("{}-{}{} ", &word[1..], c, "ay")
        },
        None => {
            String::from("")
        }

    }
}
fn piglatin(sentence: &str) -> String {
    /*
    This function splits the sentence into multiple words and calls the main convert function on each word.
    */
    let mut pigstring = String::new();
    for word in sentence.split(" ") {
        let converted = convert(&word[..]);
        pigstring.push_str(&converted);
    }
    pigstring.trim().to_string()
}

#[test]
fn test_piglatin() {
    let test_str1 = String::from("did you ever hear the tragedy of darth plagueis the wise");
    let result_str1 = String::from("id-day ou-yay ever-hay ear-hay he-tay ragedy-tay of-hay arth-day lagueis-pay he-tay ise-way");

    assert_eq!(result_str1, piglatin(&test_str1[..]));
}
#[test]
fn test_convert() {
    let test_str1 = String::from("apple");
    let result_str1 = String::from("apple-hay");
    assert_eq!(result_str1, piglatin(&test_str1[..]));

    let test_str2 = String::from("first");
    let result_str2 = String::from("irst-fay");
    assert_eq!(result_str2, piglatin(&test_str2[..]));
}
#[test]
fn test_convert_empty(){
    let test_str3 = String::from("");
    let result_str3 = String::from("");
    assert_eq!(result_str3, piglatin(&test_str3[..]));
}