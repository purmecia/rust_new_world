# Rust Chatbot using OpenAI

This is a local chatbot implemented in Rust using OpenAI's GPT-3.5 Turbo model through the OpenAI API.

## Prerequisites

1. Rust: You need to have Rust installed on your machine. If you don't have it, follow the instructions [here](https://www.rust-lang.org/tools/install).

2. API Key: You need an API key to access the OpenAI GPT-3.5 Turbo model through Hugging Face. Get the API key from [OpenAI](https://platform.openai.com/signup/) 

## Setup

1. Clone the repository
2. Set up an environment variable for your OpenAI API key. Replace your_api_key with your actual API key:

```
export OPENAI_API_KEY=your_api_key
```

For Windows, you can use:

```
$env:OPENAI_API_KEY="your_api_key"
```

## Run the Chatbot
Build and run the chatbot using Cargo:
```
cargo run
```

The chatbot is now ready to use. Type your message and press Enter to see the response. To exit the chatbot, type quit or exit.

## Example
```
Chatbot is ready. Type a message:
Hello!
Bot: Hi there! How can I assist you today?
Who you are?
Bot: I am an AI language model created by OpenAI.
What you can do?

1. Volunteer your time for a cause you believe in.
2. Educate yourself on important issues and share your knowledge with others.
3. Reduce your carbon footprint by using public transport or walking instead of driving.
5. Practice kindness and empathy towards others, including those who may be different from you.
6. Donate to charities or organizations that support causes you care about.
7. Speak up against injustice and discrimination.
8. Use your vote to support politicians and policies that align with your values.
10. Take care of your physical and mental health to be the best version of yourself possible.
```