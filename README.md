# Hangman Game in Rust

This is a simple implementation of the classic Hangman game in Rust. The game allows players to guess letters to uncover a hidden word before they run out of attempts.

## Features

- Word Selection: Randomly selects a word from a predefined list.
- Guessing Mechanism: Players can guess letters to reveal the hidden word.
- Visual Feedback: Displays the current state of the hangman and the guessed letters.
- Win/Loss Condition: Recognizes when the player wins by guessing the word correctly or loses by running out of attempts.

## Requirements

- Rust programming language (version X.X.X)
- Cargo build system

## Installation

Clone the repository:

```bash
Copy code
git clone https://github.com/your_username/hangman-rust.git
```
Navigate into the project directory:

```bash
Copy code
cd hangman-rust
```
Build the project:

```bash
Copy code
cargo build --release
```

## Usage

Run the executable after building:

```bash
cargo run --release
```

Follow the prompts to play the game. Guess letters to uncover the hidden word within the allowed attempts.