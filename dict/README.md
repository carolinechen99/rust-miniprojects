# Dict

## Overview
A CLI that looks up words in a dictionary. Users enter a word from the command line, and the program returns the definition of that word. The dictionary used is a Rust library containing an offline version of [webster's dictionary](https://docs.rs/webster/latest/webster/).

## Usage
1. cd into the dict directory `dict`
2. To run the program, type `cargo run -- lookup --word <word>`. The program will return the definition of the word.

    For example, to look up the word "rust", type:
    ```
    cargo run -- lookup --word "rust"
    ```

    And the program will return:
    ```
    The reddish yellow coating formed on iron when exposed to moistair, consisting of ferric oxide or hydroxide; hence, by extension,any metallic film of corrosion.
    ```

3. Use the command `cargo run -- lookup --help` for a description of how to run the program