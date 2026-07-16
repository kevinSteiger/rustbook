fn main() {
    println!("Hello, world!");


    let word1 = pig("hello");
    println!("pig version of hello is {}", word1);

    let word2 = pig("first");
    println!("pig version of first is {}", word2);

    let base_word = String::from("Yello");
    let word3 = pig(&base_word);
    println!("pig version of {} is {}", base_word, word3);

}

fn pig(word: &str) -> String {
    let vowels = "aeiou";

    let mut retVal = String::new();

    if vowels.contains(&word[0..1]){
        retVal = word.to_owned() + "hay"
    } else{
        retVal = word[1..].to_owned() + &word[0..1] + "ay"
    }

    retVal
}
