# DD1338 Week 10

Hope it was fun designing your own ISA's, assemblers and interpreters! Now our IndaPlus course continues on!

## The Nature of Code

You know how, there's a small chance that we're all living in a simulation? Well, let's find out how it could even be possible.

With all the BAS-MATTE, LINALG, and Fysik 1-2, you are ready to write a piece of the Matrix... well, a very small piece of the Matrix.

_The Nature of Code_ is a 14 week course written by [Daniel Shiffman](https://www.youtube.com/c/TheCodingTrain/featured). It covers the basics of physics simulation, including particle systems and flocking behaviour. Your assignment for the next week is to follow and implement one chapter of _The Nature of Code_. Yes, this does include the front-end samples.

Book: https://natureofcode.com/

### Prepare Assignment

1) Create a repository named `<KTH_ID>-simulation` and clone it. 
2) Navigate into your newly created repository and start writing.
    - Implement the logic of a chapter of choice, though it's not required to do the entire chapter.
    - Visualise the logic, either like the book samples, or something more creative.
    - Consider writing unit tests as practice.

**Note**: You may implement your solution using any language. The only drawback is that the book assumes that you run OOP. Obviously, this is not the case (for me). However, you should be able to do your own interpretations.

#### Rust Game Engines

There are several Rust game engines to choose from. Shortlist example: https://blog.logrocket.com/5-rust-game-engines-consider-next-project/

See `./examples` for three examples in Piston, courtesy of Viola.

- _GGEZ_: Interface with game loop through implementation of `EventHandler`. May not work on every obscure Linux configuration.
- _Piston_: Events as a stream iterator. You write the event loop. Doesn't handle text with grace.

### Your README

Because your solution can be implemented using any language, write in a README file of how to build and run your application (that includes how to run any unit tests and your frontend application).

Also write about what chapter you have implemented and which parts.
