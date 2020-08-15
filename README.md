# doots
Telefactor: Doots

## Install
Cargo is Rust's package manager:
https://doc.rust-lang.org/cargo/getting-started/installation.html

`cargo` is like `npm` for Rust. For example:

```
$ cargo init
$ cargo run
   Compiling doots v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/doots`
Hello, world!
```

Code formatting with RustFmt is good:
https://github.com/rust-lang/rustfmt#running-rustfmt-from-your-editor

Nicer assertions:
https://docs.rs/spectral/0.6.0/spectral/?search=equal

## Tips

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