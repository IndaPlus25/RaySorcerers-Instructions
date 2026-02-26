If written answers are required, you can add them to this file. Just copy the
relevant questions from the root of the repo, preferably in
[Markdown](https://guides.github.com/features/mastering-markdown/) format :)

#### Task 1

##### Buggy Code 1
1. What is wrong:
2. How it was fixed:

##### Buggy Code 2
1. What is wrong:
2. How it was fixed:

#### Task 2

| Question | What I expected | What happened | Why I believe this happened |
|-|-|-|-|
| What happens if you do X? |  Program would still work as before | Program ended up in a deadlock | Because of reasons 🤷 |
| What happens if you switch the order of the statements `wgp.Wait()` and `close(ch)` in the end of the `main` function? | ... | ... | ... |
| What happens if you move the `close(ch)` from the `main` function and instead close the channel in the end of the function `Produce`?  | ... | ... | ... |
| What happens if you remove the statement `close(ch)` completely?  | ... | ... | ... |
| What happens if you increase the number of consumers from 2 to 4?  | ... | ... | ... |
| Can you be sure that all strings are printed before the program stops?  | ... | ... | ... |