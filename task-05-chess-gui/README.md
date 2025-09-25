# DD1337 Week 5

## Chess GUI Project

Great job creating your first ever chess-engine! Now that you are a certified backend-developer, it's time to find out how much you'll like (or dislike) being a front-end developer ;) This weeks task is to create a GUI for a chess-engine.

To write a front-end in Rust, you have two choices:
* The [Good Game Easily library (GGEZ)](https://ggez.rs/). For help, you're referred to the [library documentation](https://docs.rs/ggez/0.6.1/ggez/) and to the code repository's [very good examples](https://github.com/ggez/ggez/tree/master/examples).
* The modular [Piston](https://www.piston.rs/) library. [Examples](https://github.com/PistonDevelopers/piston-examples/tree/master/examples). [Documentation](https://docs.rs/piston/0.53.1/piston/)
  
You can of course use another library if you want to.

Take inspiration from existing chess games or go crazy and write something unorthodox!

### Requirements
**Must haves**
* You must be able to play a game of chess
* You must be able to restart the game
* The GUI must use someone else's engine

**Nice to haves**
* Put your own spin on it! Have fun! 

### Prepare assignment

1) Create a repository named `<KTH_ID>-chess-gui` under the `INDAPlus23` organisation and clone it.
2) Navigate into your newly created repository and initialise a Rust application crate (like you did the first week).

See the template crate for help with code setup.

#### Linux

For the most based of users, additional dependencies may have to be installed for GGEZ to interact properly with all hardware.

The following is known to solve most issues:
```
sudo apt install libasound2-dev libudev-dev pkg-config
```

Unfortunately, GGEZ and WSL has been proven to be especially tricky. For example, if you are experiencing `'Failed to build context.: AudioError("Could not initialize sound system using default output device (for some reason)")'`, it is mentally sane to disenable the audio.
```rust
ContextBuilder::new()./*...*/.modules(conf::ModuleConf::default().audio(false));
```

#### Chess GUI templates

Navigate in your command prompt/terminal to `./task-5/chess-gui-templates/<piston/ggez-template>`. Run the application to show a chess board with the game state shown in text. 

The `resources` directory contains image files for all chess pieces, as well as the application icon file. The chess piece image files are loaded into image structures; a gift from me to you. Switch out the image files if you prefer to render the pieces in a different style. 

To switch from the uncomplete `chess-template` engine repository as a dependency, to one of your comrades' state of the art creations, change the target URL in the `Cargo.toml` file.
```toml
chess_template = { git = "https://github.com/INDAPlus21/chess-template.git" }
```

