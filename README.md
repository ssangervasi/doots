# Telefactor: Doots

This repo is the starting point for a game of [Telefactor](https://github.com/telefactor/telefactor) written in [Rust](https://doc.rust-lang.org/book/).

The subjects for this game will be the "AI" algorithms that can play the game automatically. Sourcerers are encouraged to make these algorithms smarter. Examiners are encouraged to find test cases where the algorithm could have played better.

The code under `src/game` and `src/players` should be left unmodified. The "AI" algorithms under `src/ai` and their tests under `tests/ai_test` will be the subject of the game.

## The Game

The base app implements a mini game engine and CLI for playing Dots and Boxes ([Wikipedia](https://en.wikipedia.org/wiki/Dots_and_boxes)).

For example, two AI players (using the "boox" algorithm) can be pitted against each other on a 5x5 board:

```sh
cargo run -- -1 boox -2 boox -s 5
```

And here is a snippet that shows Player Two filling a box and taking an extra turn:

```
Turn #3: Boox One
Player One drew: (1, 0)·─·(1, 1)


   0  1  2  3  4
 0 ╶──┐  ·  ·  ·
      │
 1 ╶──┘  ·  ·  ·

 2 ·  ·  ·  ·  ·

 3 ·  ·  ·  ·  ·

 4 ·  ·  ·  ·  ·

Turn #4: Boox Two
Player Two drew: (0, 0)·─·(1, 0)
Player Two finished a box!


   0  1  2  3  4
 0 ┌──┐  ·  ·  ·
   │2 │
 1 └──┘  ·  ·  ·

 2 ·  ·  ·  ·  ·

 3 ·  ·  ·  ·  ·

 4 ·  ·  ·  ·  ·

Streak 1! Boox Two
Player Two drew: (1, 1)·─·(2, 1)


   0  1  2  3  4
 0 ┌──┐  ·  ·  ·
   │2 │
 1 └──┤  ·  ·  ·
      │
 2 ·  ╵  ·  ·  ·

 3 ·  ·  ·  ·  ·

 4 ·  ·  ·  ·  ·

```

And, finally, Player Two eventually wins:

```
Turn #60: Boox Two
Player Two drew: (5, 4)·─·(5, 5)
Player Two finished a box!


   0  1  2  3  4  5
 0 ┌──┬──┬──┬──┬──┐
   │2 │2 │2 │2 │2 │
 1 ├──┼──┼──┼──┼──┤
   │2 │1 │2 │1 │2 │
 2 ├──┼──┼──┼──┼──┤
   │2 │1 │1 │2 │1 │
 3 ├──┼──┼──┼──┼──┤
   │2 │2 │2 │2 │1 │
 4 ├──┼──┼──┼──┼──┤
   │2 │1 │1 │2 │2 │
 5 └──┴──┴──┴──┴──┘

· ───────────────────────────────────────── ·
│                 GAME OVER                 │
│ Player Two (Boox Two) wins with 17 boxes! │
· ───────────────────────────────────────── ·
```

## Install
Cargo is Rust's package manager:
https://doc.rust-lang.org/cargo/getting-started/installation.html

`cargo` will automatically check for packages that need to be installed and compiled. For example:

```
$ cargo init
$ cargo run
   Compiling doots v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/doots`
Hello, world!
```

Just few crates are installed to start:
- Code formatting with RustFmt is good: https://github.com/rust-lang/rustfmt#running-rustfmt-from-your-editor
- Spectral for nicer assertions: https://docs.rs/spectral/0.6.0/spectral/?search=equal

## Tips

Telefactor is all about figuring out what the code does and how to test it. Here is some advice for reading and writing those tests...

### Grouping tests

Take this example test file with lots of "horse" and "cow" prefixes:

```rust
use spectral::boolean::BooleanAssertions;
use spectral::{assert_that, asserting};

use animal::{Cow, Horse}

#[test]
fn cow_says_moo {
   assert_that!(Cow().speak()).is_equal_to("moo")
}

#[test]
fn cow_is_not_a_steed {
   assert_that!(Cow().is_steed()).is_false()
}

#[test]
fn horse_says_neigh {
   assert_that!(Horse().speak()).is_equal_to("neigh")
}

#[test]
fn horse_is_a_noble_steed {
   assert_that!(Horse().is_steed()).is_true()
}

```

It can be turned into this module-grouped test file:

```rust
use spectral::boolean::BooleanAssertions;
use spectral::{assert_that, asserting};

use animal::{Cow, Horse}

mod test_cow {
    use super::*;

	#[test]
	fn says_moo {
	   assert_that!(Cow().speak()).is_equal_to("moo")
	}

	#[test]
	fn is_not_a_steed {
	   assert_that!(Cow().is_steed()).is_false()
	}
}


mod test_horse {
    use super::*;

	#[test]
	fn says_neigh {
	   assert_that!(Horse().speak()).is_equal_to("neigh")
	}

	#[test]
	fn is_a_noble_steed {
	   assert_that!(Horse().is_steed()).is_true()
	}
}
```

Note:
 - The `use super::*` is a convenience so that we don't have to re-import the
   stuff from the top of the file. We simply take everything the parent has.
   If you wanted to only import `Horse` within the horse block, you could
   remove it from the top of the file.
 - There is no naming magic going on with "test_horse". Any function tagged
   with `#[test]` will be picked up from nested modules.