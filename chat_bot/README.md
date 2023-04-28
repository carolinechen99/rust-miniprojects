# Chat Bot

## Overview

This is a chat bot written in Rust using OpenAI's GPT-4 API. It is a simple command line application that allows you to chat with the bot. The bot will respond to your messages and you can continue the conversation as long as you want.

## Usage

If you do not have openai API key, you can go to [openai.com](https://openai.com/) and sign up for a free account. Once you have an account, you can get your API key from the dashboard.

Once you have your API key, you can run the following command to start the chat bot:

For Windows:

```bash
setx OPENAI_API_KEY "<your-api-key>"
```

For Linux and macOS:

```bash
export OPENAI_API_KEY=<your-api-key>
```

Replace `<your-api-key>` with your API key.

Once you have set your API key, you can run the following command to start the chat bot:

```bash
cargo run
```

## References

a. [OpenAI API](https://beta.openai.com/docs/introduction)

b. [assimilate-openai](https://github.com/nogibjj/assimilate-openai/tree/main/chatbot)
