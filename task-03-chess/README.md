# DD1337 Week 3-4
Author: Viola Söderlund 
Modified by: Isak Larsson

## Chess Project

With that quick introduction out of the way, we are going to dive deeper into Rust and write a game-engine! The task for this and next week is to implement the logic for chess.

The implementation will be in the form of a library, that can be used by a GUI. Next task you will have to write GUIs for eachother's games, so keep that in mind when you write your API. 

**You must**
* Use rust 🦀
* Implement the required features
* Write tests for each of your functions that proves their functionality
* Document your code completely, *remember that someone else has to use it*
  * Write an additional documentation in a README.md if your API differs from the one detailed below, documenting each of your *public* functions and structs/enums/etc.

## Chess

The complete rules for chess can be found [here](https://en.wikipedia.org/wiki/Chess#Rules).

**Required features**
* Complete movesets for all pieces (except castling & en passant)
* Check
* Turn indicator (whose turn it is)
* Promotion

**Optional features**
* Castling
* En passant
* Checkmate
* Stalemate

You may implement it however you want, but for your comrades sake it is recommended that you create a struct called `Game` with the following public functions: 

| **Function**                                                                  | **Description**                                                                                                                                                                               |
| ----------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `pub fn new() -> Game`                                                        | Initialises a new board with pieces.                                                                                                                                                          |
| `pub fn make_move(&mut self, from: String, to: String) -> Option<GameState>`  | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game.                                                                     |
| `pub fn set_promotion(&mut self, piece: String) -> ()`                        | Set the piece type that a peasant becames following a promotion.                                                                                                                              |
| `pub fn get_game_state(&self) -> GameState`                                   | Get the current game state.                                                                                                                                                                   |
| `pub fn get_possible_moves(&self, position: String) -> Optional<Vec<String>>` | If a piece is standing on the given tile, return all possible new positions of that piece. Don't forget to the rules for check. _(optional)_ Don't forget to include en passent and castling. |

Positions are given as strings with the format `"<file><rank>"`. Ex. B4

Your program should also export an enumerator `GameState` with the values:
- `InProgress`, 
- `Check`,
- `GameOver`, 
- _(optional)_ `Checkmate` and
- _(optional)_ `DeadPosition`.


> Note: You are ofccourse allowed to change any details of this interface, it might even be necessary to do so depending on your implementation.  
#### Expansion

A GUI application could also make use of enumerables such as `Colour` and `PieceType`. You may also like to make changes to the above API depeding on your board representation. If your library API do not reflect the documentation above, write your own complementary documentation in your repository's `README.md` file.

### Prepare assignment

1) Create a repository named `<KTH_ID>-chess` under the `INDAPlus23` organisation and clone it.
2) Navigate into your newly created repository and initialise a Rust library crate with the following command.
```bash
cargo init --lib
```

See the template crate for help with code setup.

### Testing

Since your crate is of type library, we cannot simply test it by running it. Instead, test your application with Rust unit tests. 

The grading on this assignment is based on how well the tests demonstrates the full functionality and game mechanics of your chess engine. The tests are expected to not fail, and print a representation of the board in the case of move demonstrations. _Test at least all of your implemented functionality from the lists above to prove their functionality._

Run your unit tests with comments by entering the following command in your terminal:
```
cargo test -- --nocapture --test-threads=1
```
### Documentation

In addition to unit tests, all your public structures, functions, constants, and enumerables **must** have well written documentation comments.  
This can be done in multiple ways, one of which is the famous `doc`-comment.

You can read more at https://doc.rust-lang.org/reference/comments.html

> Note: I don't expect proffesional looking documentation, but atleast show some effort to try to document your code
