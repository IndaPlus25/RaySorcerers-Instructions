# Introduction to Go
Welcome to DD1396. In this task we will become acquainted with the Go programming language as well as touching upon its builtin concurrency features - goroutines and channels.

### 💀 Deadline
This work should be completed before the exercise on **Friday 22nd March**.

### 👩‍🏫 Instructions
For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-23/course-instructions#assignments).

### 📝 Preparation
This year we are trying **question-based learning**. 

In Canvas there is a single activity called `Task Preparation` with a link to online material. You **must attempt all the questions** for the relevant task. You also must have accepted the section invitation via email to be able to access the material.

You can also review the lecture slides: 
- [Parallel and Concurrent Programming](https://docs.google.com/presentation/d/1vb4F1Bu_ElXI6nR48lhfbN3O6iSo_3LUez8XShioHLU/edit#slide=id.p)
- [Overview of Go](https://docs.google.com/presentation/d/1t59UE1mWf-VIxr1hHkojHJ2U-vlGVFi5oFh1ufIeqSA/edit#slide=id.p)

And read the companion literature:
- [Go for Java Programmers](http://yourbasic.org/golang/go-java-tutorial/)
- Fundamentals of Concurrent Programming:
    - [Goroutines](http://yourbasic.org/golang/goroutines-explained/)
    - [Channels](https://yourbasic.org/golang/channels-explained/)

Finally, if you have not pushed an task submission before using Git/Github, then watch our [handy guide](https://www.youtube.com/watch?v=Sp5AASmX4no&list=PLZtN6QLX2rBA_gL6zs-qijIDihx-p2tO8).

### ✅ Learning Goals
* Setup a functioning Go environment
* Compare and contrast Go with either Java or Python
* Use goroutines and channels to achieve concurrency

### 🚨 Troubleshooting Guide
If you have any questions or problems, follow this procedure: <br/>

1. Look at this week's [posted issues](https://gits-15.sys.kth.se/inda-23/help/issues). Are other students asking about your problem?
2. If not, post a question yourself by creating a [New Issue](https://gits-15.sys.kth.se/inda-23/help/issues/new). Add a descriptive title, beginning with "Task *x*: *summary of problem here*"
3. Ask a TA in person during the [weekly lab](https://queue.csc.kth.se/Queue/INDA). Check your schedule to see when the next lab is.

We encourage you to discuss with your course friends, but **do not share answers**! Similarily, use of AI services  🤖 are great to *help explain things*, but please **do not submit AI-generated solutions** - you must be both responsible for your own solutions and be able to explain them under examination.

### 🏛 Assignment

#### Task 1 - Go Environment

The first task is to determine that you have a functioning Go environment on
the computer that you are working from.

- On a KTH computer - Go should be installed and ready to use
- On your own computer - Goto the [downloads page](https://golang.org/dl/) for
  and follow the installation instructions for your preferred operating system.

#### Task 2 - A Tour of Go

In this task we shall follow the online exercises hosted on
[A Tour of Go](http://tour.golang.org/welcome/1). Start at the beginning and
read through the tutorial. You are expected to submit solutions for the
following exercises:

- [Loops and Functions](http://tour.golang.org/flowcontrol/8)
    - Submit your solution in [src/gotour_exercises/loops.go](src/gotour_exercises/loops.go)
- [Slices](http://tour.golang.org/moretypes/18)
    - Submit your solution in [src/gotour_exercises/slices.go](src/gotour_exercises/slices.go)
- [Maps](http://tour.golang.org/moretypes/23)
    - Submit your solution in [src/gotour_exercises/maps.go](src/gotour_exercises/maps.go)
- [Fibonacci Closure](http://tour.golang.org/moretypes/26)
    - Submit your solution in [src/gotour_exercises/fibo.go](src/gotour_exercises/fibo.go)

Remember to format your code. Go has a unapologetic tool built-in that will
reformat your code according to a set of style rules made by the designers of
the language. To run the format utility, use the following command for all
submissions:

    $ go fmt

> **Assistant's requirement:** All Go code you submit should be formatted with
> `go fmt`. Code formatted in any other way is not acceptable.

> **Assistant's note:** `go fmt` uses tabs for indentation. You don't really
> need to worry about it as long as you make sure to run `go fmt` before
> committing, but it may be good to know.

#### Task 3 - Alarm Clock

In this task you will explore time functions using Go. Create a file called
`alarmclock.go` and write a function `Remind(text string, delay time.Duration)`
that prints output on the following form:

    The time is hh.mm.ss: <text>

Where `hh.mm.ss` should be replaced by the current hour, minute and second, and
`<text>` should be replaced by the `text` argument. **The output should be
printed forever**, separated by `delay` time units. So, for example, the call
`Remind("Hello, world!", 2*time.Second)` should print out `Hello, world!` every
two seconds, forever.

Now, write a complete program (i.e. a `main` function in `alarmclock.go`) that
runs indefinitely and prints the following reminders:

* every 10 seconds: `The time is hh.mm.ss: Time to eat`
* every 30 seconds: `The time is hh.mm.ss: Time to work`
* every 60 seconds: `The time is hh.mm.ss: Time to sleep`

To prevent the main program from exiting early, the following statement can be
used:

```Go
select {}
```

Another solution is to simply run the last call to `Remind` in the main
Goroutine, instead of starting a new Goroutine for it (yes, this is a hint as
to how you should approach writing the program, you need Goroutines).

In order to access time related functions, you should investigate the
[time package](https://golang.org/pkg/time/), and discover how to get the
current time, delay program execution and format time strings. Remember to
format your code.

> **Assistant's note:** Formatting time strings in Go is easy (as in it's a
> single function call), but it's not obvious how to do it. Look at
> [`time.Format`](https://golang.org/pkg/time/#Time.Format), read it's
> documentation, and look through the example carefully.

#### Task 4 - Two Part Sum

In this task, you will write _and test_ a function to sum an array
concurrently. When you are done with this section, make sure that you have
written and committed:

* At least two new tests for the `ConcurrentSum` function.
* Implemented `ConcurrentSum` such that it passes all tests.

##### Task 4.1 - Go `testing` framework
Start out by reading
[Chp 12 of the Golang book](https://www.golang-book.com/books/intro/12) for a
brief introduction to the `testing` framework. You are also encouraged to try
the examples, but you don't need to submit them.

> **Assistant's note:** There are a few non-obvious subtleties in the Go
> `testing` framework.
>
> * Test files must end with `_test.go`.
> * Test functions must be named on the form `TestFunc`, where you replace
>   `Func` with whatever is appropriate. _Note that the capitalization is
>   important, for example `Testfunc` and `testFunc` won't work!_
>   - You can read more
>     [in the `testing` package docs](https://golang.org/pkg/testing/)
> * For basic usage, you just type `go test`, with no other arguments. It will
>   find all of the `*_test.go` files in the current directory.
> * All Go code in the current directory must be compilable _together_. This
>   means, for example, that you can't have multiple files with `main`
>   functions in a directory where you try to run `go test`.

`cd` into `src/twopartsum/`
and run `go test`. It should find the
[`twopartsum_test.go`](src/twopartsum/twopartsum_test.go) file and run the
single test contained in it. You should get a failure message like this:

```
--- FAIL: TestSumConcurrentCorrectlySumsNonEmptyArray (0.00s)
    twopartsum_test.go:15: expected 55, was -1
FAIL
exit status 1
FAIL	_/path/to/palinda-1/src/twopartsum	0.001s
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
This task was designed by:               <br>
- Simon Larsén                             <br>
- Anton Lyxell                             <br>
- Stefan Nilsson                           <br>
- Ric Glassey                              <br>