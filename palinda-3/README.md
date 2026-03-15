# Parallel Performance

For the final task, we investigate the performance gains we can achieve by enabling parallelism and building programs that encourage concurrency as much as possible.

## 💀 Deadline

This work should be completed before the exercise on **Thu 01 April** (DD1328) and **Fri 02 April** (DD1396).

## 👩‍🏫 Instructions

For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-25/course-instructions#assignments).

## 📝 Preparation

Read and answer all questions in [Task 3: Parallel Performance](https://qbl.sys.kth.se/sections/dd1396_parallel_and_concurrent_mqniy/container/parallel_performance)

- Make sure to read the ±feedback when you answer
- You can try the questions multiple times with no penalty (eventually correct is the goal)

You can also review the lecture slides:

- [Concurrency Patterns and Mutexes](https://docs.google.com/presentation/d/193b7qpcEW3WdqmFGlA-4j9HL0FN0FdTtjYLuMg7cG7g/edit#slide=id.p)
- [Group Synchronisation and Parallel Go](https://docs.google.com/presentation/d/15kkxEB998IV6hYOX_E-hacRMHelsNqtMgl45M8kSEOg/edit#slide=id.p)

And read the companion literature:

- [Mutual exclusion](http://yourbasic.org/golang/mutex-explained/)
- [Efficient parallel computation](http://yourbasic.org/golang/efficient-parallel-computation/)

Finally, if you have not pushed an task submission before using Git/Github, then watch our [handy guide](https://www.youtube.com/watch?v=Sp5AASmX4no&list=PLZtN6QLX2rBA_gL6zs-qijIDihx-p2tO8).

## ✅ Learning Goals

- Understanding synchronisation in concurrent Go programs
- Modifying existing code to take advantage of concurrency and parallelism

## 🚨 Troubleshooting Guide

If you have any questions or problems, follow this procedure: <br/>

1. Look at this week's [posted issues](https://gits-15.sys.kth.se/inda-25/help/issues). Are other students asking about your problem?
2. If not, post a question yourself by creating a [New Issue](https://gits-15.sys.kth.se/inda-25/help/issues/new). Add a descriptive title, beginning with "Task *x*: *summary of problem here*"
3. Ask a TA in person during the [weekly lab](https://queue.csc.kth.se/Queue/INDA). Check your schedule to see when the next lab is.

We encourage you to discuss with your course friends, but **do not share answers**! Similarly, use of AI services  🤖 are great to *help explain things*, but please **do not submit AI-generated solutions** - you must be both responsible for your own solutions and be able to explain them under examination. If in doubt, refer to the **AI Policy on Canvas**.

## 🗺️ Make a Plan

- Check your feedback on last week's issue tracker for a review and tips!
- Open issues for your plan in the [issue tracker](/../../issues).
- Commit at least once per issue.
- Reference issues from commit messages using `Fixes #N` to automatically close them.
- Read more about it [here](https://gits-15.sys.kth.se/inda-25/course-instructions/blob/main/ice-guide.md).

## 🏛 Assignment

### Task 1 - Matching Behaviour

Take a look at the program [matching.go](src/matching.go). Explain what happens and why it happens if you make the following changes. Try first to reason about it, and then test your hypothesis by changing and running the program.

- What happens if you remove the `go-command` from the `Seek` call in the `main` function?
- What happens if you switch the declaration `wg := new(sync.WaitGroup)` to `var wg sync.WaitGroup` and the parameter `wg *sync.WaitGroup` to `wg sync.WaitGroup`?
- What happens if you remove the buffer on the channel match?
- What happens if you remove the default-case from the case-statement in the `main` function?

Hint: Think about the order of the instructions and what happens with arrays of different lengths.

[Open an issue for Task 1](../../issues/new?title=Task%201%3A%20Matching%20Behaviour)

### Task 2 - Fractal Images

The file [julia.go](src/julia.go) contains a program that creates Julia Set
fractal images and writes them to file. You can watch a quick explainer on the Julia Set and its relationship to the Mandelbrot Set
[here](https://www.youtube.com/watch?v=mg4bp7G0D3s), to get an idea of the type
of images that are produced.

To start with read through the code to get an idea of how it works. Some things
to note:

- `var Funcs []ComplexFunc = []ComplexFunc{` on line 18 is an array of functions.
Note how Go lets us iterate through this array and pass each function as an
argument to the `CreatePng` function in the `Main` function (see: [Wikipedia: First-class function](https://en.wikipedia.org/wiki/First-class_function)).
- In the `CreatePng` function it is the `Julia` function that creates the image and it does this by iterating through each pixel of the image to paint the final
image. The actual calculation whether to paint a pixel occurs in the `Iterate`
function in the _blink and you miss it_ line 73: `z = f(z)`, using the function from the `Funcs` array that has been passed through several functions.

The program works, but is pretty slow. In order to get a sense of how long the
program takes, you can time it using a stop watch, or more conveniently using
the `time` keyword in Bash:

```bash
time go run julia.go
```

Example output:

```bash
go run julia.go  11.10s user 0.60s system 97% cpu 11.974 total
```

The output provides several metrics, such as time spent in user mode, system mode, and CPU utilization, ending with the total elapsed time (wall-clock time). For this assignment you should focus on the "total" time for your measure of runtime, in the example output that would be `11.974` seconds.

Your assignment is to find different ways to divide the computations so that
they run in parallel on all available CPUs. Use the ideas from the example in
the [efficient parallel
computation](http://yourbasic.org/golang/efficient-parallel-computation/)
section of the course literature.

Please note that simply making everything concurrent with more and more Goroutines does not necessarily get you faster performance after a certain point (see: [Wikipedia: Amdahl's Law](https://en.wikipedia.org/wiki/Amdahl%27s_law)), so think carefully how you split the problem up (hint: each individual pixel does not needs its own Goroutine).

> **Assistant's note:** In more recent versions of Golang (since 1.5), the runtime will default to use as many operating system threads as it is allowed. To see differences in behaviour, refer to the [GOMAXPROCS](https://pkg.go.dev/runtime#GOMAXPROCS) function and vary the value.

Make sure you add a comment in [julia.go](src/julia.go) with your **original
runtime** and your **improved runtime**.

[Open an issue for Task 2](../../issues/new?title=Task%202%3A%20Fractal%20Images)

### Task 3 - MapReduce

In the final task, you will be applying the MapReduce model for improving a word frequency program. Part of the task is to research the MapReduce model and apply it to this task, however the full MapReduce model is to complex for an introductory course and we are satisfied with a simplified version of the model.

The simplified MapReduce model would consist of three main concepts that are at the core of what MapReduce is all about.

1. **Splitting** the task into smaller chunks to be run in parallel, this is often referred to as *partitioning* and can be done over a single source or by treating each source as a different partition to begin with.
2. The **Map** function converts the input of a partition into key value pairs, such as a key for each word in a text, and a value for the number of times it occurred. Many map functions can thus run at once over different partitions of the input.
3. **Reduce** accepts the output of all Map function instances and combines their individual results into a single result for the entire task.

> **Assistant's note:** While we are satisfied with a simplified MapReduce model the interested reader can take a look at the original Google Research article presenting the full MapReduce programming model. [MapReduce: Simplified Data Processing on Large Clusters](https://static.googleusercontent.com/media/research.google.com/sv//archive/mapreduce-osdi04.pdf)

A word frequency analysis of a document will return a summary of the word counts for all unique words in the document. Whilst this can be solved efficiently using a map data structure in a sequential program, the performance can be improved by parallelising the program.

> **Assistant's note:** Typically we ignore case when counting the frequency so 'Hello', 'HELLO' and 'hello' are all counted as 'hello', so remember to convert all strings to lower case. We also ignore punctuation marks, so these must be removed as well.

Read the code in [singleworker/words.go](src/singleworker/words.go) and complete the missing parts:

- Implementation for the `WordCount` function
- Reading a text file into a string in the `main` function
- Filtering duplicate versions of words in different cases, removing punctuation marks, etc
- Check that the unittest passes
- Log the runtime performance in the table below

Once you are satisfied with the singleworker, move into [mapreduce/words.go](src/mapreduce/words.go) and parallelise the program in order to improve the performance.

> **Assistant's note:** Part of the challenge here is identifying the different tasks that can be made parallel. This can happen at multiple levels so pay attention for opportunities to increase the performance.

- Update the `WordCount` function with the Map and Reduce tasks, using goroutines to parallelise and a channel to gather partial results
- Check that the unittest passes
- Find the optimal amount of goroutines before you encounter diminishing returns in performance improvements
- Log the runtime performance in the table below

|Variant       | Runtime (ms) |
| ------------ | ------------:|
| singleworker |          xxx |
| mapreduce    |          yyy |

> **Assistant's note:** Experiment with different amount of goroutines for maximum performance using numerical values,
> but you can also further try using system variables
> or multiples of system variables. Such system info can be reached through the runtime package,
> for example the number of available logical CPUs through `runtime.NumCPU()` which you can read more about [here](https://pkg.go.dev/runtime#NumCPU).

And with that, you are on your way to Google-scale problems ;-)

[Open an issue for Task 3](../../issues/new?title=Task%203%3A%20MapReduce)

## 🙏 Acknowledgments

This task was designed by:

- Simon Larsén
- Anton Lyxell
- Vincent Nord Lundström
- Jimmy Tran
- Stefan Nilsson
- Ric Glassey
