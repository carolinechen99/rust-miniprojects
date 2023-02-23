/* A library that uses hugging face models to translate text from Japanese to English and vice versa. */
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};

// Translate from English to Japanese using from the hugging face model.
// Take command line input as a string.
pub fn translate_to_japanese(text: &str) -> anyhow::Result<()> {
    let model = TranslationModelBuilder::new()
        //required by this bound in `rust_bert::pipelines::translation::TranslationModelBuilder::with_source_languages`
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Japanese])
        .create_model()?;
    // create output variable, where text is translated from English to Japanese
    // pass text to model.translate()
    let output = model.translate(&[text], None, Language::Japanese)?;
    for translation in output {
        println!("{translation}");
    }
    // return output
    Ok(())

}

// Translate from Japanese to English using from the hugging face model.
// Take command line input as a string.
pub fn translate_to_english(text: &str) -> anyhow::Result<()> {
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::Japanese])
        .with_target_languages(vec![Language::English])
        .create_model()?;
    // create output variable, where text is translated from Japanese to English
    // pass text to model.translate()
    let output = model.translate(&[text], None, Language::English)?;
    for translation in output {
        println!("{translation}");
    }
    // return output
    Ok(())
}
