# DD1337 Week 7

Heavily edited by: Benjamin Widman

## Getting started with C

### Install GCC toolchain
> Note: This is already installed in most Linux distros, so if you are using something like Ubuntu (hint to WSL users) you can skip this.
1) Install [MingW](https://www.mingw-w64.org/). *Windows-users are recommended to just use WSL*. 
2) Test your installation by entering `gcc --help` in your terminal or command prompt(*host pleb *host).

Congrats dear programmer. You can now compile your C files by entering the following command. (`$` indicates a terminal prompt)
```
$ gcc program.c -o name_of_executable
```
To run: 
```
$ ./name_of_executable
```

### Prepare for your assignment

1) Create a repository named `<KTH_ID>-clang`.
2) Clone your newly created repository.
3) Create one `.c` file per subassignment. Name then descriptively.  

For help with code setup, begin by copying the contents of `./kattis_template/main.c` into your `.c` file.

If you're not familiar with the C language I recommend reading up on it online, for example [GeeksForGekks](https://www.geeksforgeeks.org/c-programming-language/).

## Assignment 1 - Kattis (optional)

As a warmup, solve [A Different Problem](https://open.kattis.com/problems/different), which is now on Open Kattis and not KTH's Kattis (i.e. not locked behind courses😄).

See `./minimal_scalar_product` for a Kattis solution example in C.

**Don't forget** to include screenshots of Kattis to prove that your solutions work.

## Assignment 2.1 - RSA key generation

The main part of this task will be to implement the [RSA public-key cryptosystem](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) to encrypt files with a public encryption key such that they're unreadable, and incredibly difficult to crack, without the private decryption key. The first, and biggest, part will be to create these two keys.

Create a program called `keygen.c` which creates one file containing the public key and another containing the private key. E.g. `public.key` and `private.key`.

- To understand RSA, watch this short video: https://www.youtube.com/watch?v=wcbH4t5SJpg
- Or Eddie Woo's two videos:
  - Intro: https://www.youtube.com/watch?v=4zahvcJ9glg
  - Generating keys: https://www.youtube.com/watch?v=oOcTVTpUsPQ

So the important values we have are:

| Value | Description |
| --- | --- |
| p = large prime | Random, but p != q. |
| q = large prime | Random, but p != q. |
| n = p * q | The modulo. Public, but it's incredibly hard to find the two prime factors. |
| r = (p-1)(q-1) | Modulo to find d. Actually Euler's phi function of n, but don't mind that until diskmatten. |
| e = 7 | The encryption exponent. Public. Can be other bigger prime numbers but we keep it small to not run into overflow. |
| d = e^-1 (mod r) | The decryption exponent. Private. |

The public key is `(e, n)` while the private key is `(d, n)` (n is included in the private key here for simplicity). How these are stored in the files is up to you, whether it's raw data or plain text separated into two lines.

There are multiple ways you can find prime numbers. One example is the [sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) (note there's pseudo code for it) or a faster but much harder to implement [sieve of Atkin](https://en.wikipedia.org/wiki/Sieve_of_Atkin).

When selecting two random prime numbers out of the ones you've found with a sieve, make sure you initialize rand with the current time as the seed to avoid always getting the same output when running the program:

```c
#include <time.h>
#include <stdlib.h>

int main() {
    srand(time(NULL));
    int p = rand();
    int q = rand();

    return 0;
}
```

For simplicity sake I suggest limiting the value of `n` (thus how big the p & q you choose are) to fit within a C datatype, like `int` or better `unsigned long long int` (nothing some evil type casting can't handle muahaha). Using prime numbers around the range 500-1000 *should* not cause any overflow, but please test around. However, if you want to you can extend your key size to 2048 or 4096 and in that case increase `e` to 65537, for top-class security.

> Note: `e^-1` is not `1/e` but the inverse of `e` in modulo `r`. If you remember from basmatten it's the number which when multiplied with `e` equals 1 in modulo `r`. I.e: ed (mod r) = 1.

`d` can either be found using the [extended Euclidean algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm) or more simply looping and testing every multiple of `e` to see if it is equal to 1 in modulo r.

## Assignment 2.2 - RSA encryption

Create a program called `encrypt.c` which takes the name of a text file as a command line argument and encrypts the content into a new file prefixed with "encrypted": `encrypted_<previous name>.txt`.
```
$ ./encrypt text.txt
```
Which outputs: `encrypted_text.txt`

It's supposed to read in the public key `(e, n)` from the previously generated file and then encrypt using: `m^e (mod n)`

`m` is the message we want to encrypt. When we are encrypting a string of text we need to run this for every character or byte.

### How to get CLI arguments

Command line arguments are retrieved from `argv` when using the following function header:
```c
int main(int argc, char **argv) {}
```
- `int argc` is the argument count, i.e. the number of arguments supplied. This is useful for either error checking that the correct number of arguments where supplied or looping through the arguments when there can be a varied amount of them.
- `char **argv` are the argument values and is a C-style string array since `char*` is a string and arrays are just pointers, hence the double pointer.

## Assignment 2.3 - RSA decryption

Create a new program called `decrypt.c` which takes the same command line arguments as `encrypt.c` but instead reverses the encryption of the supplied text file. The result shall be in a new file with either the prefix "encrypted" removed or with a new prefix "decrypted" for fun growing names🤪.

It reads in the private key (d, n) from the previously generated file and then decrypts using: `c^d (mod n)`

`c` is the cipher text, i.e. the encrypted message.

## Questions

You don't have to write the answers to these but think about them and know the answers during our next lesson.

### Data control

Observe the following code:

```c
int length = 0;
scanf("%d", &length); 

int* vector = malloc(length * sizeof(int));
for (int i = 0; i < length; i++) 
    scanf("%d", &vector[i]);

free(vector);
```

Know the answer of the following question:
- What is happening line for line?

### Data types are like whaaat?

Observe the following function:

```c
int x;

printf("Nothing: %d\nGive x a value: ", x);

scanf("%d", x);

printf("\nYour value is: %d\n", x);
```

Know the answer of the following question:
- What is printed?
