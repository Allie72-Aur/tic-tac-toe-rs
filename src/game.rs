// The rand crate is used for generating random numbers. In this game, it's specifically
// used to determine the CPU's random move.
use rand::Rng;
// The std::io library provides the standard input/output functionality,
// which is necessary for reading user input from the command line.
use std::{io, usize};

/// An enumeration representing possible errors that can occur when a player
/// attempts to make a move.
///
/// This provides a clean way to handle different failure cases.
#[derive(Debug)]
enum PickError {
    /// Indicates that the player's chosen spot on the board is already occupied.
    AreaOccupied,
    /// Signifies that the game board (`moves_map`) has not been initialized yet.
    MovesMapNotInitialized,
    /// Denotes that the player's chosen index is outside the valid range of 0 to 8.
    OutOfBounds,
}

/// An enumeration representing the result of a game state check.
///
/// This is used by the `check` function to communicate the outcome of a turn.
enum CheckResult {
    /// A player or the CPU has won the game.
    Win,
    /// The game has ended in a tie.
    Tie,
    /// The game is still ongoing, and no winner or tie has been determined.
    Contine,
}

/// An enumeration representing the state of a single cell on the Tic-Tac-Toe board.
///
/// It derives several traits for convenience:
/// - `Debug`: Allows the enum to be printed for debugging purposes.
/// - `Clone`, `Copy`: Enables efficient value-based copying, as the enum is a simple type.
/// - `PartialEq`: Allows for direct comparison between `State` variants.
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    /// Represents the human player's mark.
    X,
    /// Represents the CPU's mark.
    O,
    /// Represents an empty, available cell on the board.
    Empty,
}

/// A struct to hold the scores for the ongoing game.
///
/// It derives `Debug` for easy printing of the scores.
#[derive(Debug)]
struct Score {
    /// The number of games won by the player.
    player: u16,
    /// The number of games won by the CPU.
    cpu: u16,
    /// The number of games that have ended in a tie.
    tie: u16,
}

/// The main game struct that encapsulates all the necessary data and logic
/// for a Tic-Tac-Toe game.
///
/// The `pub` keyword makes this struct accessible from other modules,
/// allowing for the creation of a `Game` instance.
#[derive(Debug)]
pub struct Game {
    /// Represents the game board. An `Option` is used because the board is not
    /// initialized until the `start` method is called. Once initialized, it's
    /// a fixed-size array of 9 `State` enums, representing a 3x3 grid.
    moves_map: Option<[State; 9]>,
    /// The score tracker for the game.
    score: Score,
}

impl Game {
    /// The constructor for the `Game` struct.
    ///
    /// Initializes a new game instance with an un-initialized board (`None`)
    /// and a score of 0 for all categories.
    pub fn new() -> Self {
        Game {
            moves_map: None,
            score: Score {
                player: 0,
                cpu: 0,
                tie: 0,
            },
        }
    }

    /// The main entry point for the game.
    ///
    /// This method sets up the game board and runs the primary game loop,
    /// handling turns, input, and game state.
    pub fn start(&mut self) {
        // Initialize the moves_map with an empty board represented by
        // an array of 9 `State::Empty` values.
        self.moves_map = Some([State::Empty; 9]);

        // The main game loop. It continues indefinitely, allowing for multiple
        // rounds of Tic-Tac-Toe until the program is manually terminated.
        loop {
            println!("Choose index(0 to 8):");
            // Display the current state of the board and the scores.
            self.print_info();
            let mut input = String::new();
            // Read the player's input from the console.
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // Convert the input string to an integer, trimming the newline character.
            // This will panic if the input is not a valid number.
            let number: usize = input.trim().parse().expect("Please enter a valid number");
            println!("You entered: {}", number);

            // Attempt to place the player's mark on the board.
            match self.pick_player(number) {
                // If the move was successful, check for a win or tie.
                Ok(()) => match self.check(State::X) {
                    CheckResult::Win => {
                        println!("** You win! **");
                        self.increase_score(1);
                        self.reset();
                        continue;
                    }
                    CheckResult::Tie => {
                        println!("** Tie! **");
                        self.increase_score(0);
                        self.reset();
                        continue;
                    }
                    CheckResult::Contine => {
                        println!("** Cpu turn **");
                    }
                },
                // Handle various errors from `pick_player`.
                Err(PickError::AreaOccupied) => {
                    println!("That area is already occupied!");
                    continue; // Skip to the next iteration of the loop, prompting the player again.
                }
                Err(PickError::OutOfBounds) => {
                    println!("Invalid index!\nMust be between 0 and 8");
                    continue; // Skip to the next iteration.
                }
                Err(PickError::MovesMapNotInitialized) => println!("The game has not started!"),
            };

            // If the player's turn didn't end the game, it's the CPU's turn.
            self.pick_cpu();
            // Check for a CPU win or tie.
            match self.check(State::O) {
                CheckResult::Win => {
                    println!("** Cpu wins! **");
                    self.increase_score(2);
                    self.reset();
                    continue;
                }
                CheckResult::Tie => {
                    println!("** Tie! **");
                    self.increase_score(0);
                    self.reset();
                    continue;
                }
                CheckResult::Contine => {
                    println!("** Your turn **");
                }
            }
        }
    }

    /// Increments the score based on the outcome of a round.
    ///
    /// The `turn` parameter is used to determine which score to update:
    /// - `0`: Tie
    /// - `1`: Player win
    /// - `2`: CPU win
    fn increase_score(&mut self, turn: u8) {
        match turn {
            0 => self.score.tie += 1,
            1 => self.score.player += 1,
            2 => self.score.cpu += 1,
            _ => (),
        }
    }

    /// Resets the game board for a new round without clearing the score.
    fn reset(&mut self) {
        self.moves_map = Some([State::Empty; 9]);
    }

    /// Checks if the board is completely filled.
    ///
    /// This is a utility function used to detect a tie condition.
    fn is_full(&self) -> bool {
        match self.moves_map {
            // If the board exists, iterate over all its cells and check if any are `State::Empty`.
            // `all(|&v| v != State::Empty)` returns true if no cells are empty.
            Some(moves) => moves.iter().all(|&v| v != State::Empty),
            // If the board doesn't exist, it's not full.
            None => false,
        }
    }

    /// Prints the current state of the game board and the scores to the console.
    fn print_info(&self) {
        match &self.moves_map {
            // If the board exists, print it.
            Some(moves) => {
                for (i, &val) in moves.iter().enumerate() {
                    // Choose the symbol to print based on the cell's state.
                    let symbol = match val {
                        State::X => "X",
                        State::O => "O",
                        State::Empty => ".",
                    };
                    // Print the symbol with padding.
                    print!("{:3}", symbol);
                    // Print a new line every 3 symbols to create a 3x3 grid.
                    if (i + 1) % 3 == 0 {
                        println!();
                    }
                }
            }
            // If the board doesn't exist, inform the user.
            None => println!("No moves yet!"),
        };
        // Print the current scores.
        println!("{:?}", &self.score)
    }

    /// Makes a move for the CPU.
    ///
    /// The CPU's move is chosen randomly from the available empty spots.
    fn pick_cpu(&mut self) {
        // Enters a loop that continues until a valid move is made.
        loop {
            // Create a thread-local random number generator.
            let mut rng = rand::thread_rng();
            // Generate a random index between 0 and 8.
            let index: usize = rng.gen_range(0..=8);
            
            // This is a safety check to prevent an infinite loop if the board is full
            // before the CPU's turn. The `check` method already handles the tie condition.
            if self.is_full() {
                return;
            }

            // If the board exists and the randomly chosen index is empty,
            // place the CPU's mark and exit the loop.
            if let Some(map) = &mut self.moves_map {
                if map[index] == State::Empty {
                    map[index] = State::O;
                    break;
                }
            }
        }
    }

    /// Attempts to make a move for the human player.
    ///
    /// This function handles validation of the player's input and returns
    /// a `Result` indicating success or failure.
    fn pick_player(&mut self, index: usize) -> Result<(), PickError> {
        // Use a range match to check if the index is valid.
        match index {
            0..=8 => {
                // If the board exists, proceed with the move.
                if let Some(map) = &mut self.moves_map {
                    // Check if the chosen spot is empty.
                    if map[index] == State::Empty {
                        // Place the player's mark and return success.
                        map[index] = State::X;
                        Ok(())
                    } else {
                        // The spot is occupied, return the appropriate error.
                        Err(PickError::AreaOccupied)
                    }
                } else {
                    // The board is not initialized, return the corresponding error.
                    Err(PickError::MovesMapNotInitialized)
                }
            }
            // The index is out of the valid range, return the error.
            _ => Err(PickError::OutOfBounds),
        }
    }

    /// Checks the current state of the game board for a win or a tie.
    ///
    /// This function contains the core game logic for determining the outcome of a turn.
    fn check(&mut self, state: State) -> CheckResult {
        // Only perform the check if the board is initialized.
        if let Some(map) = self.moves_map {
            // The following logic checks for a win by scanning all possible winning combinations.
            // The board is a 1-dimensional array, so we use pointer-like logic to check rows, columns, and diagonals.

            // --- Check for COLUMN wins (Vertical) ---
            let (mut ptr1, mut ptr2, mut ptr3) = (0, 3, 6);
            for _ in 0..=2 {
                if map[ptr1] == state && map[ptr2] == state && map[ptr3] == state {
                    return CheckResult::Win;
                }
                // Move to the next column.
                ptr1 += 1;
                ptr2 += 1;
                ptr3 += 1;
            }

            // --- Check for ROW wins (Horizontal) ---
            (ptr1, ptr2, ptr3) = (0, 1, 2);
            for _ in 0..=2 {
                if map[ptr1] == state && map[ptr2] == state && map[ptr3] == state {
                    return CheckResult::Win;
                }
                // Move to the next row.
                ptr1 += 3;
                ptr2 += 3;
                ptr3 += 3;
            }

            // --- Check for DIAGONAL wins ---
            // Primary diagonal (top-left to bottom-right).
            (ptr1, ptr2, ptr3) = (0, 4, 8);
            if map[ptr1] == state && map[ptr2] == state && map[ptr3] == state {
                return CheckResult::Win;
            }
            // Secondary diagonal (top-right to bottom-left).
            (ptr1, ptr2, ptr3) = (2, 4, 6);
            if map[ptr1] == state && map[ptr2] == state && map[ptr3] == state {
                return CheckResult::Win;
            }

            // --- Check for a TIE ---
            // If no win condition was met, check if the board is full.
            if self.is_full() {
                return CheckResult::Tie;
            }
        }
        // If none of the above conditions were met, the game continues.
        return CheckResult::Contine;
    }
}
