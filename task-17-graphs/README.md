# DD1338 Week 17

Author: Benjamin Widman & Dmitry Chirin

## Data Structures

You think you know lists, but you don't (or maybe you do, who knows?). Prove it, in style.

### Prepare for your assignment

1) Create a repository named `<KTH_ID>-graphs`.
2) Clone your newly created repository and start coding.

To make your Kattis-experience easier, see `./kattis_template/src/main.rs`. If you're mot all förmodan using Java, make use of [this utility class](https://open.kattis.com/download/Kattio.java?1a0093=) to simplify input-output fast enough to satisfy the judge.

## Assignment

For this week you have two assignments:
1) Use a binary search tree to sort an array, and
2) solve two Kattis problems involving graphs.

### Tree structure

You are required to write an implementation of a [binary search tree (BST)](https://en.wikipedia.org/wiki/Binary_search_tree) structure and then use it to sort arrays, as a way to prove its correctness. 

You could also _(optionally)_ try implementing a [self-balancing BST](https://en.wikipedia.org/wiki/Self-balancing_binary_search_tree) (instead or in addition to BST), such as an [AVL tree](https://en.wikipedia.org/wiki/AVL_tree) or a [red–black tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree). 

A template with tests can be found in [bst-template](bst_template).

### Graph problems

Now you're going to solve two Kattis problems involving traversing graphs with the help of [depth-first search (DFS)](https://en.wikipedia.org/wiki/Depth-first_search) and [breadth-first search (BFS)](https://en.wikipedia.org/wiki/Breadth-first_search). Include screenshots of your Kattis submissions to prove your solutions. See `../task-02-rustIntro/minimal_scalar_product` for a Rust Kattis solution example.

Solve both of the following problems:
- DFS: [Where's My Internet??](https://open.kattis.com/problems/wheresmyinternet)
- BFS: [Grid](https://open.kattis.com/problems/grid)

_(optional fun)_:
- [Fenwick Tree](https://open.kattis.com/problems/fenwick)
- [Kötid](https://po.kattis.com/problems/kotid)

### Questions

Be prepared to answer the following questions during the next övning.

#### Complexity

What is the time complexity of the insertion, look-up and deletion functions of non-balanced vs. balanced trees? Non-balanced trees have different best and worst case complexities, why?

See [Wikipedia](https://en.wikipedia.org/wiki/Self-balancing_binary_search_tree) if you don't know what a balanced binary search tree is.

#### Unusual data structures

There are more data structures than lists, stacks, queues, trees and graphs. Learn about a new (maybe unusual) data structure and its implementations. Be prepared to teach the group about it.
