## Learn Async Rust

There is no standard / official Asynchronous runtime. This in contrast to actual CPU threads [which you can create from within the std library](https://doc.rust-lang.org/std/thread/fn.spawn.html).

The most popular and recommended by many Async Runtime is [the Tokio runtime](https://tokio.rs/). Which brings us to the next chapter:

- To start with your Async Learning Journey you might want to start with: [Getting Started - Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/);
- Afterwards you can finally start going through the Tokio [learning journey Tutorial](https://tokio.rs/tokio/tutorial) 

Please also join the Tokio Discord community as part of your async Journey, a place where you can ask any questions you want as well: <https://discord.com/invite/tokio>. This server also has individual channels for the different projects in the Tokio ecosystem such as Hyper, Axum, Tower and more. The maintainers of the different projects are also very active in the server and are happy to help you out, but of course please do not take this for granted. Be respectful and if possible contribute back to the community as well.

The following article is a very nice introduction to Rust async: <https://ibraheem.ca/posts/too-many-web-servers/>.

Extra Learning Resources:

- <https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust/>;
- <https://bitbashing.io/async-rust.html>: great article to make you think about why Async rust might feel so wrong.
  - A big problem is that most people use Tokio in multithreade mode which puts extra restrictions (Send + 'Static),
    which makes Async feel extra difficult.
  - It's also important that people, next to having experience with Tokio and getting a good understanding of Async rust and all it entails,
    to also discover, play and use other async runtimes.
    - <https://github.com/smol-rs/smol> is a great alternative Async runtime that you might want to try out;
- Learn more about Async Rust and what the current situation is/was in 2023: <https://corrode.dev/blog/async/>
- Get an understanding about issues you might encounter at Rust regarding Async:
  - Deadlocks
  - Cancellation safety
    - <https://docs.google.com/presentation/d/1CObHaEu1OnXhTEFs3RmuUQHJj_Uw9HOqWHqmnod6YXc/mobilepresent?slide=id.g228c626047c_0_222>
    - <https://google.github.io/comprehensive-rust/async/pitfalls/cancellation.html>

At <https://without.boats/blog/why-async-rust/> you can reat more thoughts on Async rust, its history and its implementation.

As an extra, and perhaps slightly sidetracked, you may also want to read and develop alongside the following articles:

- [The HTTP crash course nobody asked for](https://fasterthanli.me/articles/the-http-crash-course-nobody-asked-for)
- [Understanding Rust futures by going way too deep](https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep)
- To get to get a deeper understanding about the concepts of pinning and the like you might find the following articles helpful:
  - [Put a pin on That](https://archive.is/pHfCn)
    - This article refers also to [Pin, Unpin and why Rust needs them](https://archive.is/LH91o)
    - as well as [Pin and Suffering](https://archive.is/32RlT)
  - All of the above are great articles to get you quickly up to speeds with the concept, and helpful as a repeater or because you have no desire or time to read some of the books that talk about this topic and that I recommend in other parts of this guide;

In order to help you understand how Async code works (e.g. what about the Async keyword and How do futures really work. And how do you define traits with futures. And How do you implement them), you might want to read through the articles linked in the [Tower](#tower) section, as seeing how to implement your own _Tower_ middleware might answer many of such questions. Gaining a deeper theoretical but still pragmatic enough understanding behind it is probably better done by reading some of the books listed in this guide.

> ⓘ Note: Tokio also provides support to [the new Linux kernel io_uring concept](https://unixism.net/loti/), a new powerful way to allow async programming on Linux. Support for it can be found at: [GitHub - tokio-rs/tokio-uring: An io_uring backed runtime for Rust](https://github.com/tokio-rs/tokio-uring)
> 
> Synacktiv made an IO network scanner using io_uring as can be seen at [GitHub - synacktiv/io_uring_scanner: io_uring based network scanner written in Rust](https://github.com/synacktiv/io_uring_scanner), using tokio's low level userspace bindings to io_uring.

To get a better idea about how futures work and the executor which polls them, you might want to read this article: <https://bertptrs.nl/2023/04/27/how-does-async-rust-work.html>.

The blog series as found at <https://hegdenu.net/posts/understanding-async-await-1/> can be another great reference to help you understand the entire `async/await` part of Rust:

- [part 1: why doesn’t my task do anything if I don’t await it?](https://hegdenu.net/posts/understanding-async-await-1/)
- [part 2: how does a pending future get woken?](https://hegdenu.net/posts/understanding-async-await-2/)
- [part 3: why shouldn’t I hold a mutex guard across an await point?](https://hegdenu.net/posts/understanding-async-await-3/)
- [part 4: why would I ever want to write a future manually?](https://hegdenu.net/posts/understanding-async-await-4/#why-would-i-ever-want-to-write-a-future-manually)

Sooner or later you'll also start to see the conterversy surrounding Async, while most do agree it's a a great thing to be and have,
and also acknowledge the hard work that is put into it, but not about all implementation details people agree.
One such detail is about whether we really need `async fn` syntax, an interesting take about that can be read at
<https://seanmonstar.com/post/66832922686/was-async-fn-a-mistake>. Like always follow interesting blogs,
and news resources such as ["This week in Rust"](https://this-week-in-rust.org/).

There are currently not yet async iterators. An interesting blog series that can help you
get a grasp of what async iterators might look like or to get you start thinking on it
is the following series by "Yoshua Wuyts":

1. Async Iteration I: Semantics (2020): <https://blog.yoshuawuyts.com/async-iteration/>
2. Async Iteration II: Async Iterator Crate (2022): <https://blog.yoshuawuyts.com/async-iterator-crate/>
3. Async Iteration III: The Async Iterator Trait (2023): <https://blog.yoshuawuyts.com/async-iterator-trait/>
