### Appendix I. Install Rust

In order to start using Rust you have to install and configure its tooling on your machinery.

- The Rust compiler (`rustc`), standard library and tooling (e.g. `cargo`) are all managed using `rustup`.
  - You can learn how to get started by going to: <https://www.rust-lang.org/learn/get-started>
  - Once you finished with this you'll have Rust ready on your machine out of the box. Each time there is a new version of Rust you can update it, simply by running `rustup update`

With this installed you should now be able to run the following command:

```
cargo version
```

Which will output something like:

```
cargo 1.67.1 (8ecd4f20a 2023-01-10)
```

You can learn to create, compile and run your first Rust program at "[First Steps with Cargo - The Cargo Book](https://www.rust-lang.org/learn/get-started)".

> If you are looking for a fun bigger project to help you learn Rust,
> you might find it fun to buy your own cargo-like tool: <https://blog.mgattozzi.dev/freight-part-1/>

#### [Linting and styling](#linting-and-styling)

- Install `clippy` using `rustup component add clippy`
  - This way you can use the command `cargo clippy` to help guarantee the quality of your code
- Install `rustfmt` using `rustup component add rustfmt`
  - This way you can use `cargo fmt` to format your code (best to also configure this in your IDE to be done each time you save the file)

No custom linting configurations or agreements are required beyond this. If you use these two tools, you're golden. These tools also work nicely with any IDE you might use. Within your CI environment these same tools will be used to check that your code adheres to its standards.

Clippy can be seen as a testing ground to see what lints are useful and which ones aren't. The ones that are often get shipped into the official compiler a while later. Some lints are also ignored by default for whatever reasons. You can find all lints (AFAIK) at <https://rust-lang.github.io/rust-clippy/stable/index.html>, should you want to enable even more lints.

Some useful reasons why you might stil want to enable custom lints are:

- to enable nightly features;
- to prevent the usage (in your own codebase) of `unsafe` code.

For reference you can check the <https://github.com/plabayo/tower-async> mono repository's `lib.rs` files,
where I enable such opt-in lints myself, which itself is based on the original `tower-rs` code.
