### Threading

Please do remember that Async programming is only beneficial to optimize the idle time of your threads away.
For tasks where you already near 100% utilize your threads (e.g. computation heavy algorithms) something
like async will not help you and will in fact make your codebase a lot more complex and difficult to reason about.

Should you have computation-heavy algorithms which you want to parallelize you can achieve that by making use
of [threading](https://doc.rust-lang.org/std/thread/) alone. A crate such as <https://docs.rs/rayon/latest/rayon/> might help you a lot with this as well.
