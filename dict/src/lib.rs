/* A command line tool that returns the definition of a word from a dictionary. */

/* Accept a word as a command line argument.
If the word exists in webster dictionary, returns its defination.
Else, returns an error message.
*/

pub fn get_definition(word: &str) -> String {
    // Get the definition of a word from a dictionary.
    let def = webster::dictionary(word);
    // If the def is equal to None, returns an error message.
    if def.is_none() {
        format!("{word} is not a word.")
    }
    // If the def is not equal to None, returns the definition.
    else {
        let result = webster::dictionary(word).unwrap().to_string();
        result
    }
}
