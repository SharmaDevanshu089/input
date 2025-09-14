# from-user

**A tiny, highly simplified Rust input helper**

**Version:** 0.2.1

A minimal library that provides two helpers for reading user input from stdin:
- `single_line()` — returns the raw line as a `String`.
- `number()` — tries to read a numeric line and return it as `i32` (retries on invalid input).

This crate is intentionally small and easy to read — good for learning or small CLI experiments.

---

## Quick start

Add the crate to your `Cargo.toml` (when published on crates.io):

```toml
[dependencies]
from-user = "0.2"
```

> Note: when using the crate in Rust code, the crate name uses an underscore in `use` paths (hyphens become underscores):

```rust
use from_user::{single_line, number};

fn main() {
    let s = single_line();
    println!("you typed: {}", s);

    let n = number();
    println!("number: {}", n);
}
```

If you are testing locally without publishing, add it by path in your binary project's `Cargo.toml`:

```toml
[dependencies]
from-user = { path = "../from-user" }
```

---

## API

### `pub fn single_line() -> String`
Reads one line from `stdin` and returns it as a `String` (including trailing newline if present). It currently prints a short message on `read_line` errors and returns whatever it could read.

**Example:**

```rust
let s = from_user::single_line();
println!("raw: {:?}", s);
```


### `pub fn number() -> i32`
Reads a line from `stdin`, validates characters, and attempts to parse the trimmed input into `i32`.
- On parse failure, it prints a message and **retries** by calling `number()` again.
- Intended behavior: keep asking until a valid integer is entered, then return it.

**Example:**

```rust
let n = from_user::number();
println!("got int: {}", n);
```


---

## Known issues & quick fixes

This crate is simple by design and the current `number()` implementation has a few rough edges. Below are short, easy-to-understand fixes you can apply as tiny edits.

~1. **Loop condition is reversed — causes the loop not to run at all.**~

```diff
- while loop_variable > word_length {
+ while loop_variable < word_length {
```

~2. **Character check logic is inverted.**~
- Current code treats *digits* as invalid. Fix by negating the check.

```diff
- if char_to_check.is_ascii_digit() {
-     println!("Please enter a Numerics only");
-     return number();
- }
+ if !char_to_check.is_ascii_digit() {
+     println!("Please enter numerics only");
+     return number();
+ }
```

~3. **`loop_variable` never increments → infinite loop.**~

Add this at the end of the loop body:

```diff
+ loop_variable += 1;
```

4. **`chars().nth()` can return `None` (out of bounds).**
- Your code used `.expect(SRS)` which will panic if something goes wrong. The fixes above (loop condition and increment) avoid the out-of-bounds case. If you prefer safer code, use pattern matching on the `Option<char>`.

5. **Parsing fallback (already present) — retry on parse error.**
- The crate currently uses a `match` on `parse()` and calls `number()` again on `Err(_)` — that is fine for simplicity. Be aware that heavy recursion on many bad inputs could grow the call stack; for interactive use this is usually not a problem.


---

## Design notes

- The crate is intentionally simple: it focuses on blocking stdin reads and straightforward retries.
- It uses recursion for retry behavior because it is easy to follow. If you plan to ship to many users or expect adversarial input, consider replacing recursion with a `loop` to avoid stack growth.

---

## Contributing

- Open an issue or a pull request if you want to improve the validation, add non-blocking variants, or add tests.
- If you submit fixes, please keep changes small and document them in the PR description.

---

## License & author

From your `Cargo.toml`:
- **Author:** Devanshu Sharma <sharma.devanshu089@gmail.com>
- **License:** `MIT OR Apache-2.0`

---