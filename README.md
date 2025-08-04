# **Tic-Tac-Toe in Rust**

A simple, classic Tic-Tac-Toe game implemented in Rust. This is a command-line interface (CLI) game where a player can compete against a simple, random-moving CPU.

## **Features**

* **Command-Line Interface**: Play directly from your terminal.  
* **Player vs. CPU**: Challenge a computer opponent.  
* **Score Tracking**: The game keeps a running score of player wins, CPU wins, and ties.  
* **Simple Game Logic**: Uses a straightforward approach to detect wins and ties.

## **Getting Started**

### **Prerequisites**

To run this project, you need to have the **Rust programming language** and **Cargo**, its package manager, installed on your system.

You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### **Building and Running**

1. **Clone the repository:**  
```bash
   git clone https://github.com/rogue-87/tic-tac-toe-rs.git  
   cd tic-tac-toe-rs
```
2. **Run the game with Cargo:**  
```bash
   cargo run
```
## **How to Play**

The game will prompt you to choose a number from 0 to 8 to place your 'X' on the 3x3 grid. The grid layout corresponds to the indices like this:

0 | 1 | 2  
\--+---+--  
3 | 4 | 5  
\--+---+--  
6 | 7 | 8

After each of your turns, the CPU will make its move, and the board will be displayed. The game continues until a player wins or the board is full, resulting in a tie. You can play multiple rounds, and the score will be tracked.

## **Game Logic**

The game logic is implemented within the Game struct in src/main.rs. Key aspects of the implementation include:

* **Board Representation**: A 9-element array of State enums (X, O, Empty) is used to represent the 3x3 game board.  
* **CPU AI**: The CPU opponent uses a simple random number generator to pick an available spot on the board.  
* **Win and Tie Detection**: The check method systematically scans the board for all possible winning combinations (horizontal, vertical, and diagonal lines). If no winning condition is met and the board is full, a tie is declared.

## **Contributing**

If you have any suggestions or improvements, feel free to open an issue or submit a pull request.