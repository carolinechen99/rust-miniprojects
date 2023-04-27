# Sentiment

## Overview

## Usage

1. cd into the sentiment directory `sentiment`
2. To run the program, type `cargo run -- analyze --sentence <sentence>`. The program will return the sentiment of the sentence.

   For example, to analyze the sentence "I love Rust", type:

   ```bash
   cargo run -- analyze --sentence "I love Rust"
   ```

   And the program will return:

   ```bash
    Sentiment score: 3
    comparative score: 1
    Positive words: ["love"]
    Negative words: []
   ```

3. Use the command `cargo run -- analyze --help` for a description of how to run the program
