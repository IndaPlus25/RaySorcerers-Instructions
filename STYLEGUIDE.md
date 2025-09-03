# Styleguide for submissions

This is a small styleguide that should act like a checklist for your submitted solutions. It contains some best-practice for submitting code for course-assingments, but it is also very useful for all code that you write that someone needs to read (including you). 

I should note that most of these are my opinions, but since I'm the one who has to read your assignments it would be nice if you followed atleast some of them :)
## 1. Include a README
It doesn't matter what kind of project you are writing, a README is a must-have for every git-repo. It can serve multiple purposes, here are some examples on what to include (if necessary):

1. Build instructions
2. Known bugs or errors in your code
3. Usage examples, if it's a CLI-tool or similar
4. Which subassignments you have done
5. File descriptions (if you have like 10 modules)

It is especially important if you know that you have not finished some parts of the task, or it isn't working as intended, that you write this in the README. I want to be sure that it isn't an issue with my machine. 
## 2. Use a .gitignore
Compiled binaries and executables are only useable on the type of machine that compiled them. An `.exe` won't be useful for linux-users. Furthermore having binaries in the working-tree is generally annoying (for you) as they change everytime you compile your program, and it is hardly useful to know how the bytecode changes between versions. 

An easy way to make sure that you keep the correct files out of your repo is to specify a `.gitignore` template for the language you are using when you create the repo in github.

## 3. Remove commented code and test-prints

Your code will be much more readable if there aren't blocks of unused code laying around. 

I feel like I don't have to explain why you should remove test-prints.

If your code is a kattis-submission, it should be in the state that you sent to kattis. For example if you have tested your program by reading a test-file, remember to change it back to reading from `stdin`, like you would in a kattis submission.

If you want to use your own testdata, but don't want to read directly from a file you can use piping:
```
./exec < testfile.txt
```
This writes (pipes) the contents of `testfile.txt` to `stdin`.
## 4. Use reasonable filenames

It is a bit annoying having to go through like 11 files to find the one that is relevant. If you have written a parser for say BBVV, name it `BBVVparser`, or `parser`. Don't use abberviations like `BBVVpar`, or in the case of interpreters `BBVVint`. If you must use abbreviations or names that are not immediatly recognisable, include a description at the top of the file. 

Example:
```rust
/*
A parser for BBVV
Author: Isak Larsson (2022) 
*/
```

> Note: I feel like this doesn't need to be said but don't use joke-names for files (unless it's clear what the file does)


## 5. Name your repo according to the instructions
If the repo for the instructions is named `task-9`, but the instructions specify that you should name it `<kth-name>-assembly`, follow the instructions.

We TAs use automated tools for grading, and differently named repos throw a wrench in the workflow. 

## 6. Include screenshots of Kattis-submissions
This should be included in the instructions, but sometimes I forget to write it. 

If you didn't pass the tests, either include a screenshot of the failed submission or/and tell me in the README.