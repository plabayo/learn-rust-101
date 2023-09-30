### Appendix VIII. Debugging Rust

Debugging Rust code can be seen on three different levels:

1. Write Rustic code, making full use of the language and its expressive type system;
2. Have good logging and metrics in place for your production infrastructure;
3. Know your way around a debugger.

Underappreciated is point (1). Ensuring that impossible state and behavior is in fact impossible
at compile team and thus guaranteed by the compiler is a big class of errors that you simply
can no longer make and thus never have to debug. Using maker types and enums can help you a lot here.

Rust is also a typed language so ensuring to use the proper types and use them consistently,
will also help avoid a big category of bugs. Similarly, you probably do not really ever want to
unwrap/expect any of your library code and only do so at the very top level, as to avoid nasty edge cases at runtime.

That being said, sometimes things do go wrong. And debugging in production is not always fun.
In fact, trying to reproduce rare scenarios locally is even less fun. For that you really
want to ensure you know the right tools and crates to help you deal with point (2).

- If you use [Tokio](https://tokio.rs) then the [`tracing` crate](https://tracing.rs/tracing/) and its mini ecosystem will be your friend, to help you define logs, metrics and more.
  - The more data you have for your production infrastructure the easier it should be to spot potential issues that you didn't foresee but that you do clearly see in your logs and/or metrics;
  - If you really want to dive deep and also learn about its macros, a blog article like <https://dietcode.io/p/tracing-macros/> might help you get into that;
  - A blog article like <https://broch.tech/posts/rust-tracing-opentelemetry/> might help you get started with Tracing in Rust,
    and introduce you to "Open Telemetry" in general.
- If you do not use _Tokio_, then a crate like [log](https://crates.io/crates/log) is your best bet, and it will ensure that no matter what application makes use of your crate, will be able to consume your logs the way they want :)

> A detailed overview of logging libraries that are available can be found in <https://www.shuttle.rs/blog/2023/09/20/logging-in-rust>.

Finally when shit really hits the fan, and you do not see what's wrong from metrics and logs (alone), you might need to grab a debugger. The book <https://rustc-dev-guide.rust-lang.org/debugging-support-in-rustc.html> might be able to give you a good starting point. Articles such as <https://dev.to/rogertorres/debugging-rust-with-vs-code-11dj> might help you get started on debugging in case you are using VSCode for Rust, which is what plenty of people seem to use these days. There's also a Gist about it for VSCode at <https://gist.github.com/xanathar/c7c83e6d53b72dd4464f695607012629>.
