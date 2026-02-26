# Concurrency Conundrums
In this task we will try to dive into the difficulties that concurrency creates; both in debugging existing code and developing new code.

### 💀 Deadline
This work should be completed before the exercise on **Friday 12th April**.

### 👩‍🏫 Instructions
For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-23/course-instructions#assignments).

### 📝 Preparation
In Canvas there is a single activity called `Task Preparation` with a link to online material. You **must attempt all the questions** for the relevant task. You also must have accepted the section invitation via email to be able to access the material.

You can also review the lecture slides: 
- [Channels & Coordination](https://docs.google.com/presentation/d/1I46lo9wX3BSZwLD6AXMPcS1Rto5peC16hciNgwRqYqs/edit#slide=id.p)
- [Data races, Deadlocks & Dining Philosophers](https://docs.google.com/presentation/d/14Y6YIHBsGDgNzhILRfu7Qubl0jrzoInc4sex_pCUNqY/edit#slide=id.p)

And read the companion literature:
  - [Select](http://yourbasic.org/golang/select-explained/)
  - [Data races](http://yourbasic.org/golang/data-races-explained/)
  - [Detecting data races](http://yourbasic.org/golang/detect-data-races/)
  - [Deadlock](http://yourbasic.org/golang/detect-deadlock/)
  - [Waiting and Wait groups](https://yourbasic.org/golang/wait-for-goroutines-waitgroup/)

Finally, if you have not pushed an task submission before using Git/Github, then watch our [handy guide](https://www.youtube.com/watch?v=Sp5AASmX4no&list=PLZtN6QLX2rBA_gL6zs-qijIDihx-p2tO8).

### ✅ Learning Goals
* Detecting data races and deadlocks as concurrent bugs
* Debugging concurrency bugs in existing code
* Developing a concurrent solution

### 🚨 Troubleshooting Guide
If you have any questions or problems, follow this procedure: <br/>

1. Look at this week's [posted issues](https://gits-15.sys.kth.se/inda-23/help/issues). Are other students asking about your problem?
2. If not, post a question yourself by creating a [New Issue](https://gits-15.sys.kth.se/inda-23/help/issues/new). Add a descriptive title, beginning with "Task *x*: *summary of problem here*"
3. Ask a TA in person during the [weekly lab](https://queue.csc.kth.se/Queue/INDA). Check your schedule to see when the next lab is.

We encourage you to discuss with your course friends, but **do not share answers**! Similarily, use of AI services  🤖 are great to *help explain things*, but please **do not submit AI-generated solutions** - you must be both responsible for your own solutions and be able to explain them under examination.

### 🏛 Assignment

#### Task 1 - Debugging Concurrent Programs
In this task, you will be provided with two buggy programs. For each program,
you should:

1. Explain what is wrong with the code.
2. Fix the bug.
3. Explain why your solution fixes the bug.

##### Buggy program 1
[src/bug01.go](src/bug01.go)

##### Buggy program 2
[src/bug02.go](src/bug02.go)

Please provide your answers [here](docs/README.md).

> **Assistant's requirement:** You are not allowed to move or remove the
> time.Sleep(10 * time.Millisecond) statement in the Print function. A
> deterministic solution (e.g. using a channel or WaitGroup) is required and
> removing the sleep is only a quick fix.

#### Task 2 - Many Senders; Many Receivers
The program [many2many.go](src/many2many.go) contains four
producers that together send 32 strings over a channel. At the
other end there are two consumers that receive the strings.
Describe what happens, and explain why it happens, if you make the
following changes in the program. Try first to reason your way
through, and then test your hypothesis by changing and running the
program.

* What happens if you switch the order of the statements
  `wgp.Wait()` and `close(ch)` in the end of the `main` function?
* What happens if you move the `close(ch)` from the `main` function
  and instead close the channel in the end of the function
  `Produce`?
* What happens if you remove the statement `close(ch)` completely?
* What happens if you increase the number of consumers from 2 to 4?
* Can you be sure that all strings are printed before the program
  stops?

Please provide your answers [here](docs/README.md).

Finally, modify the code by adding a new WaitGroup that waits for
all consumers to finish.

> **Assistant's requirement:** The syntax for waiting and wait groups is fairly 
> simple - see the preparation link above in case you have not already read it 
> through before attempting the task...who would even contemplate doing that?


#### Task 3 - Pythia, the Oracle of Delphi

The code in [oracle.go](src/oracle.go) contains the outline for a program that
will answer 'questions' (like a chatbot, but worse).  Complete the `Oracle`
function.  You should not modify the `main` function or other function
signatures. Note that answers should not appear immediately; instead there
should be a random delay or **pause for thought**.  Also, the Oracle will still
print **helpful predictions** even if there are not any questions, after a
random amount of time. You may structure your solution into multiple functions.

Your program should contain two channels: One channel for questions,
and one for answers and predictions.  In the `Oracle` function you
should start three infinite go-routines.

* A go-routine that receives all questions, and for each incoming
  question, creates a separate go-routine that answers that question
* A go-routine that generates predictions
* A go-routine that receives all answers and predictions, and prints
  then to stdout

Whilst the `Oracle` function is the most important of the
assignment, you should also make the following improvements:

* Parse input for specific keywords ("What is the meaning of life?") and create a response ("Ah, life! ...")
* Prints answers and predictions one character at a time with a short delay
* Your own creative chat-bot-like idea :-)

### 🙏 Acknowledgments
This task was designed by:               <br>
- Simon Larsén                             <br>
- Anton Lyxell                             <br>
- Stefan Nilsson                           <br>
- Ric Glassey                              <br>