# Introduction to Go

Welcome to DD1396. In this task we will become acquainted with the Go programming language as well as touching upon its builtin concurrency features - goroutines and channels.

### 💀 Deadline

This work should be completed before the exercise on **Friday 21st March**.

### 👩‍🏫 Instructions

For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-24/course-instructions#assignments).

### 📝 Task Preparation

Answering questions is good for you, even if the subject is completely new!

Read and answer all questions in [Task 1: Introduction to Go](https://qbl.sys.kth.se/sections/dd1396_parallel_and_concurrent_kcj76/container/introduction_to_go)

- Make sure to read the ±feedback when you answer
- You can try the questions multiple times with no penalty (eventually correct is the goal)

You should also review the lecture slides:

- [Parallel and Concurrent Programming](https://docs.google.com/presentation/d/1vb4F1Bu_ElXI6nR48lhfbN3O6iSo_3LUez8XShioHLU/edit#slide=id.p)
- [Overview of Go](https://docs.google.com/presentation/d/1t59UE1mWf-VIxr1hHkojHJ2U-vlGVFi5oFh1ufIeqSA/edit#slide=id.p)

And browse the companion literature:

- [Go for Java Programmers](http://yourbasic.org/golang/go-java-tutorial/)
- Fundamentals of Concurrent Programming:
  - [Goroutines](http://yourbasic.org/golang/goroutines-explained/)
  - [Channels](https://yourbasic.org/golang/channels-explained/)

Finally, if you have not pushed an task submission before using Git/Github, then watch our [handy guide](https://www.youtube.com/watch?v=Sp5AASmX4no&list=PLZtN6QLX2rBA_gL6zs-qijIDihx-p2tO8).

### ✅ Learning Goals

- Setup a functioning Go environment
- Compare and contrast Go with either Java or Python
- Use goroutines and channels to achieve concurrency

### 🚨 Troubleshooting Guide

If you have any questions or problems, follow this procedure:

1. Look at this week's [posted issues](https://gits-15.sys.kth.se/inda-24/help/issues). Are other students asking about your problem?
2. If not, post a question yourself by creating a [New Issue](https://gits-15.sys.kth.se/inda-24/help/issues/new). Add a descriptive title, beginning with "Task *x*: *summary of problem here*"
3. Ask a TA in person during the [weekly lab](https://queue.csc.kth.se/Queue/INDA). Check your schedule to see when the next lab is.

We encourage you to discuss with your course friends, but **do not share answers**! Similarily, use of AI services  🤖 are great to *help explain things*, but please **do not submit AI-generated solutions** - you must be both responsible for your own solutions and be able to explain them under examination.

### 🏛 Assignment

#### Task 1 - Go Environment

The first task is to determine that you have a functioning Go environment on
the computer that you are working from.

- On a KTH computer - Go should be installed and ready to use
- On your own computer - Goto the [downloads page](https://go.dev/dl/) for
  and follow the installation instructions for your preferred operating system.

#### Task 2 - Go Fundamentals

In this task we shall familiarize ourselves with some programming concepts that you know from previous INDA courses, but in Go! It is recommended that you first check out [A Tour of Go](https://go.dev/tour/) for bite-sized tutorials that will cover the fundamentals of Go, including everything you need to solve the following tasks.

> **Assistant's note:** The exercises in this fundamentals tasks are modified versions of a few select exercises found in A Tour of Go.

##### Task 2.1 Functions and Loops

In [src/basics/loops.go](src/basics/loops.go) you will find a code skeleton for which you are to implement a numerical method for solving a simple function.

> **Assistant's note:** This math is taught later in the numerical methods course, however if you are interested, here is a brief explanation of the method.
>
> Given a function $f(x) = y$, we can use the [Newton-Raphson](https://en.wikipedia.org/wiki/Newton%27s_method) method to solve for $x$ for any given output. This is done by continuously making a linear approximation of the function and moving towards the answer along this approximation, until the rate at which we move is small enough to conclude that we have a good enough approximate answer.

In the function `Solver` you will need to set an initial guess `z` for the value that will be our answer, the function will the refine this answer for each iteration.

The function we will solve for is $f(x) = x^3$, for this function the better guess is derived by:

```go
z = z - ((z*z*z - x) / (3*z*z))
```

Your task is specifically to create a loop inside the `Solver` that will run this guess-improving line of code, for a given amount of iterations. Do you arrive at an answer after just 1 loop, 10 or 20? Are there any other creative stopping conditions you can implement to make sure that the loop finishes only when the answer is sufficiently close to being correct?

You can compare your answer to the correct answer by running the Go function `math.Cbrt()` to compute the cubic root of a number.

##### Task 2.2 Slices

In [src/basics/slices.go](src/basics/slices.go) you will find a code skeleton where you will implement and loop through slices.

In Go an array has a fixed length much like many other programming languages. But there is also a powerful feature called slices. A slice is a flexibly sized view of an array, and uses the following syntax.

```go
fibonacci = [8]int{0, 1, 1, 2, 3, 5, 8, 13}
// Slices uses the syntax [low:high]
// low indicates the index of the first element, where the slice starts
// high indicates the cutoff element, the index of the first element that
// will not be included in the slice
fibonacci[2:5] // [1, 2, 3]
// If no low is specified it is equal to zero
fibonacci[:4] // [0, 1, 1, 2]
// If no low is specified it is equal to keeping all elements
// after and including the low index
fibonacci[4:] // [ 3, 5, 8, 13]
```

It is also possible to make a slice of a given length using the following syntax.

```go
s := make([]int, 5) // [0, 0, 0, 0, 0]
```

Your task is to create, inside the function `Pic` a slice of length `dy`, where every slice contains a slice of length `dx`, in which you will store an 8-bit unsigned integer based on any formula you want.

You need to loop over the `dx` and `dy` ranges to allocate each value and `[]uint8` element inside the larger `[][]uint8` that is your return type.

Some interesting formulas to try are: `x^y`, `(x + y) / 2`, `x*y` but you are free to construct whatever you want.

When you run your function, a .png image will be created automatically for you, representing the greyscale color values of the formula you have used.

##### Task 2.3 Maps

In [src/basics/fibo.go](src/basics/fibo.go) you will find a code skeleton where you will implement the fibonacci closure.

Implement a word counter in `WordCount` in  that counts every occurrence of a word and returns a map with each unique word and the amount of times it occurred in the text. You do not need to take uppercase and lowercase into account, for example *The* and *the* can be treated as different words.

> **Hint:** It can be useful to look into the function `strings.Fields`.

##### Task 2.4 Function closures

In [src/basics/maps.go](src/basics/maps.go)

A function may be a function closure, a higher-order function that returns binds an environment to a value while also returning a function.

```go
// Defines a closure
func closure() func(int) int {
  sum := 0
  return func (x int) int {
    sum += x
    return sum
  }
}

// Bind closure to a variable
summation = closure()
summation(5)  // 5
summation(5)  // 10
summation(20) // 30
```

Your task is to define the content of the closure `Fibonacci` that will calculate the next number in the fibonacci sequence for each call to the function. When running the program skeleton, if implemented correctly, your output should be fibonacci numbers 0 through to 34.

Remember to format your code. Go has a unapologetic tool built-in that will
reformat your code according to a set of style rules made by the designers of
the language. To run the format utility, use the following command for all
submissions:

```bash
$> go fmt
```

> **Assistant's requirement:** All Go code you submit should be formatted with
> `go fmt`. Code formatted in any other way is not acceptable.
>
> **Assistant's note:** `go fmt` uses tabs for indentation. You don't really
> need to worry about it as long as you make sure to run `go fmt` before
> committing, but it may be good to know.

#### Task 3 - Alarm Clock

In this task you will explore time functions using Go. Create a file called
`alarmclock.go` and write a function `Remind(text string, delay time.Duration)`
that prints output on the following form:

```bash
The time is hh.mm.ss: <text>
```

Where `hh.mm.ss` should be replaced by the current hour, minute and second, and
`<text>` should be replaced by the `text` argument. **The output should be
printed forever**, separated by `delay` time units. So, for example, the call
`Remind("Hello, world!", 2*time.Second)` should print out `Hello, world!` every
two seconds, forever.

Now, write a complete program (i.e. a `main` function in `alarmclock.go`) that
runs indefinitely and prints the following reminders:

- every 10 seconds: `The time is hh.mm.ss: Time to eat`
- every 30 seconds: `The time is hh.mm.ss: Time to work`
- every 60 seconds: `The time is hh.mm.ss: Time to sleep`

To prevent the main program from exiting early, the following statement can be
used:

```Go
select {}
```

> **Assistant's note:** Inserting a `select{}` statement indicates that go should wait for some
> communication to be sent over some channel. If this is left empty, Go will wait forever instead,
> preventing the function from exiting. This is not considered good practice, but will serve its
> purpose until we have properly introduced channels and how to communicate between Goroutines.

Another solution is to simply run the last call to `Remind` in the main
Goroutine, instead of starting a new Goroutine for it (yes, this is a hint as
to how you should approach writing the program, you need Goroutines).

In order to access time related functions, you should investigate the
[time package](https://pkg.go.dev/time), and discover how to get the
current time, delay program execution and format time strings. Remember to
format your code.

> **Assistant's note:** Formatting time strings in Go is easy (as in it's a
> single function call), but it's not obvious how to do it. Look at
> [`time.Format`](https://pkg.go.dev/time#Time.Format), read it's
> documentation, and look through the example carefully.

#### Task 4 - Two Part Sum

In this task, you will write *and test* a function to sum an array
concurrently. When you are done with this section, make sure that you have
written and committed:

- At least two new tests for the `ConcurrentSum` function.
- Implemented `ConcurrentSum` such that it passes all tests.

##### Task 4.1 - Go `testing` framework

Start out by reading
[Chp 9 of the Golang book](https://learning.oreilly.com/library/view/introducing-go/9781491941997/ch09.html) for a
brief introduction to the `testing` framework. The book is available for free to any KTH student through O'Riley,
if you sign in with your KTH email. You are also encouraged to try the examples, but you don't need to submit them.

> **Assistant's note:** There are a few non-obvious subtleties in the Go
> `testing` framework.
>
> - Test files must end with `_test.go`.
> - Test functions must be named on the form `TestFunc`, where you replace
>   `Func` with whatever is appropriate. _Note that the capitalization is
>   important, for example `Testfunc` and `testFunc` won't work!
>   - You can read more
>     [in the `testing` package docs](https://pkg.go.dev/testing)
> - For basic usage, you just type `go test`, with no other arguments. It will
>   find all of the `*_test.go` files in the current directory.
> - You can also run `go test ./folder` to run tests in a specified directory.
> - All Go code in the current directory must be compilable *together*. This
>   means, for example, that you can't have multiple files with `main`
>   functions in a directory where you try to run `go test`.
>
> **Assistant's note:** We use a TableDrivenTests structure which is a way to write tests
> that is encouraged by Go. You can read more about it at the [Go-Wiki: TableDrivenTests](https://go.dev/wiki/TableDrivenTests).
> This pattern is also used in the Golang book but not expressed explicitly.

`cd` into `src/twopartsum/`
and run `go test`, or simply run `go test ./src/twopartsum/` from the repo root. It should find the
[`twopartsum_test.go`](src/twopartsum/twopartsum_test.go) file and run the
single test contained in it. You should get a failure message like this:

```bash
--- FAIL: TestConcurrentsum (0.00s)
    --- FAIL: TestConcurrentsum/sum_even_array (0.00s)
        twopartsum_test.go:25: Test: "sum even array" failed: expected 55, got -1
FAIL
exit status 1
FAIL    palinda-1/src/twopartsum        0.004s
```

Now, **write at least two additional tests** in
[`src/twopartsum/twopartsum_test.go`](src/twopartsum/twopartsum_test.go) and
make sure that they fail properly before moving on to task 4.2.

##### Task 4.2 - Implementing the concurrent sum function

Now that the testing is out of the way, you can get down to implementing the
`ConcurrentSum` function. It adds all of the numbers in an array by splitting
the array in half and then having two Go routines take care of a half each.
Partial results are then sent over a channel. You should implement
`ConcurrentSum`, and its helper function `sum`.

```Go
package main

// sum the numbers in a and send the result on res.
func sum(a []int, res chan<- int) {
    // TODO sum a
    // TODO send result on res
}

// concurrently sum the array a.
func ConcurrentSum(a []int) int {
    n := len(a)
    ch := make(chan int)
    go sum(a[:n/2], ch)
    go sum(a[n/2:], ch)

    // TODO Get the subtotals from the channel and return their sum
    return -1
}
```

Implement your solution in
[`src/twopartsum/twopartsum.go`](src/twopartsum/twopartsum.go).

### 🙏 Acknowledgments

This task was designed by:

- Simon Larsén
- Anton Lyxell
- Stefan Nilsson
- Ric Glassey
- Vincent Nord Lundström
