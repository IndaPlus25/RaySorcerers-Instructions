# DD1338 Week 14
Edited by: Benjamin Widman

So long for-loops, hello recursion!
This week we are going to learn a language from the functional paradigm: Haskell.

## Task
Solve this Kattis-problem: https://kth.kattis.com/courses/DD1366/progp26/assignments/da9oo8/problems/kth.progp.warmup

(Register on the course first: https://kth.kattis.com/courses/DD1366/progp26/register)

## Prepare assignment
1. As usual create a repo with the name `<kth-id>-haskell`.
2. Install Haskell via [GHCup](https://www.haskell.org/ghcup/) *or* simply install `ghc` from your package manager on Linux or MacOS.
3. Write solutions to the Kattis problems and screenshot that you've passed the judge.

> How-to ghc: `ghc` is the Haskell compiler but the installation should also come with `ghci`, which you can use to interactively run Haskell code. While inside `ghci` you can load a file with `:l <file name>.hs` and then interactively run functions inside that file.

## Reading material
Up to chapter 6 of the "Haskell book": https://learnyouahaskell.github.io/chapters.html

## Questions
Know the answers to these questions and be prepared to be able to answer them.

1. Why can you write expressions like this one in Haskell?
```haskell
let evenNumbers = [ 2 * n | n <- [1..]]
print (take 1000000 evenNumbers)
```

2. Will this code work?
```haskell
addOne :: [Int] -> [Int]
addOne (l:ls) = (l+1):(addOne ls)
```

3. What is a pure function? Give an example of a pure and a non-pure function.
