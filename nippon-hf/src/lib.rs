/* A libray that utilizes hugging face's transformers library to perform
 * natural language processing tasks.
 */

use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::fs::File;
use std::io::Read;

// build a function that reads a file in Japanese and translates it to English
pub fn translate_jp_to_en() {
    // read the file from the path specified by user using command line arguments
    let mut file = File::open("nippon-hf/src/jp.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    // build a translation model
    let model = TranslationModelBuilder::new()
        .with_model(Language::Japanese, Language::English)
        .build()
        .unwrap();

    // translate the contents of the file
    let output = model.translate(&[&contents]).unwrap();

    // print the output
    println!("{:?}", output);
}
