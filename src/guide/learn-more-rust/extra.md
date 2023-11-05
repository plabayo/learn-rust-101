## Learn More Rust: Extra

Once you went through the resources in the previous two chapters you might want to start working on a real project to start applying and solidifying your knowledge. A classic way to do this is by porting a small specialized codebase from one language to Rust, keeping in mind to not 1-to-1 map the lines of code but to instead focus on porting the logic with the "Rust way to do things" in mind.

Should you want some inspiration or a more guided approach, here are some resources that could help you out with this extra optional, but recommended step:

- [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/): build a cli tool such as grep in rust, using test driven development (TDD), all to learn Rust
- [Rust Application Books - The Little Book of Rust Books](https://lborb.github.io/book/applications.html): lists a lot of different books, including ones that allow you to develop small projects using Rust, usually in a well guided fashion;
  - You might find <https://archive.is/acQA2> an interesting companion article as it gives some (common) ideas that you can apply to the CLI tool you're building;
  - [Introduction - PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/) is an especially fun small project. It allows you to apply the knowledge that you learned above in a very narrow program, that is for once not network related. At the end you'll have a cli tool that allows you to encode and decode "hidden" messages in a PNG image;

> Was it always your dream to make your own Game? And do you also want to learn Rust?
>
> <https://bfnightly.bracketproductions.com/webbuild.html> might be the free online guide for you!
>
> In this tutorial you go through the entire process of building your own game and compile
> it to WASM so that you can play it directly from any modern browser!
>
> If you like this tutorial and other material by Herbert Wolverson
> you can also go over to <https://pragprog.com/titles/hwrust/hands-on-rust/> and buy his book,
> available as paperback and ebook.

Most pragmatic commercial Rust books cover Macros only as much as you need to know about them:

- What are macros?
- How do they work?
- How to read them?
- And some books might even teach you about `cargo expand` (the tool that will help you with reading macros);

When the day comes that you do feel ready to learn about Macros, here a list of extensive resources all about macros:

- [Introduction - MacroKata](https://tfpk.github.io/macrokata/index.html): Macro's are in general not very covered in the resources above, as it is not that common that you have to write your own macros. They can however be convenient in certain situations. This book can be seen as a bootstrap tutorial to get you started with one kind of Rust macros.
  - If you are interested in (also) learning procedural macros you can perhaps consult the following resources:
    - [GitHub - dtolnay/proc-macro-workshop: Learn to write Rust procedural macros [Rust Latam conference, Montevideo Uruguay, March 2019]](https://github.com/dtolnay/proc-macro-workshop)
    - [Procedural Macros in Rust (part 1)](https://www.youtube.com/watch?v=geovSK3wMB8)
    - Check out "[Diesel is a Safe, Extensible ORM and Query Builder for Rust](https://diesel.rs/)" if you want to see a very cool example of macros (besides all the derives, formatting and printing you might already have been doing)
    - [json in serde_json - Rust](https://docs.rs/serde_json/latest/serde_json/macro.json.html) is another neat example as it allows you to quickly define a Json value directly in Rust
      - Taking a couple of hours is also a life well spent reading through this codebase as it will teach you a lot of nice things about Rust, especially given how simple and scoped the problem of json serialization is. It will really show you and teach you how serde works and why it works;
    - Most web frameworks (if not all) in Rust also heavily make use of Macro's to give you a very neat and clean API
  - And of course do not forget to consult the "classic" macro Rust book: [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)
  - If you want to get a quick intro and overview of Macros this video can help a lot as well: [Rust's Witchcraft](https://www.youtube.com/watch?v=MWRPYBoCEaY)

> 🔗 [Rust Power Tools (MEAP)](https://www.manning.com/books/rust-power-tools) (By Manning Publications Co.)
>
> A great book to help you with supercharging your code with macros—the real power tools of the Rust programming language!
>
> This book focuses mostly on Procedural macros, but does give a quick intro of declarative macros as well (`macro_rules!`).
>
> Rust Power Tools is a comprehensive guide to creating macros in Rust. You’ll start your journey with declarative macros, then quickly move on to the powerful procedural macros to build your own domain-specific language. Learn how to create public fields, work with custom attributes, integrate your macros with other crates, write effective tests to ensure your macros are reliable and bug-free, and even share your macros with other developers.

If you have a lot of free time at your hand and you want to build something really cool that is totally useless, here are some more ideas:

- Build your own Gameboy (classic) emulator: [Introduction - DMG-01: How to Emulate a Game Boy](https://rylev.github.io/DMG-01/public/book/introduction.html)
- Build your own CHIP-8 (8-bit computer) interpreter: [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) (no Rust guidance, directly from the CHIP-8 specification instead, you can do it, I believe in you)
- Build your own Ray Tracer (not a Rust tutorial, but can easily be implemented in Rust as well): [Ray Tracing in One Weekend Series](https://raytracing.github.io/) (fun if you always had the dream to start dabbling into the world of Graphics Programming)
  - <https://www.superperfundo.dev/?tag=ray-tracer-challenge> might be a useful blog series if you want a more guided approach on how you might make a Ray Tracer in Rust;
  - <https://archive.is/t6Ifp> is an interesting article in this area as well, where it introduces you to a language called PO;

Seriously though, do not consider the above recommended or mandatory in any way. If you however really like to develop stuff and you do like to do it extensively in your free time, then, and only then, I do believe that the above are a great way to really solidify your current Rust knowledge and give yourself a great (pragmatic) foundation.

At this point you might also be ready to start reading alternative — community driven — learning resources on foundational Rust knowledge. With such content you do however always need to be careful as they might contain mistakes, not well rounded or closed opinions. An example of possible valuable learning resources are the following blog series about the Rust type system:

- part 1 (<https://sanjuvi.github.io/Blog/posts/Rust-type-system-part-1/>): ownership and move semantics, aliasing and mutation, lifetime and region based resource management;
- part 2 (<https://sanjuvi.github.io/Blog/posts/Rust-Type-System-Part-2>/): type as sets, algebraic data types, patterns in Rust using Algebraic types, traits, generic associated types (GAT), operator overloading, use cases;
- part 3 (<https://sanjuvi.github.io/Blog/posts/Rust-Type-System-Part-3>/): container types, interior mutability, concurrency, deadlocks without threads.

More parts to come as well.

For the brave among you with the time for it, learn to build your own Operating System (OS) using Rust: <https://os.phil-opp.com/>

### Code like a pro in Rust

At this point of your Rust learning journey you've come a far way already, relatively speaking. Perhaps you want to shortcut, at least for now.

If so, [the still to be published "Code like a pro in Rust" book](https://www.manning.com/books/code-like-a-pro-in-rust) might be enough for you to quickly get from beginner to advanced Rust programmer.

### A Crust of Rust

Once you get at this point you are around the level of a beginner or intermediate Rust programmer. At this point you could already start to tackle stuff like ["Rust for Rustaceans" of step (6)](/guide/study-using-the-rust-for-rustaceans-idiomatic-programming-for-experienced-developers-book.md). However it might be a bit much.

Therefore while you go through the next couple of sections it could be helpful to now and then (e.g. while washing dishes or instead of Netflix) an education Rust code video. In the appendix at the bottom of this page there a couple of such suggestions.

Particular to this context you might want to save [the "Crust of Rust" playlist](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa). Each video will go over a specific topic, e.g. Lifetimes, macros, Atomics, … No need to watch them one by one, feel free to jump to them in whatever orders and only those that interest you (which is the same tip that I can give for the "Rust for Rustaceans" book).

### API Design

Designing APIs is a big part of being a Rust programmer, any programmer really. Once you start to get comfortable enough in Rust or start building your own public crate, it is a topic you'll want to invest in.

A good starting point is the [API Guidelines](https://rust-lang.github.io/api-guidelines/about.html) which are a set of guidelines that are meant to help you design good APIs. They are not meant to be followed blindly, but rather to be used as a reference to help you make the right decisions.

A next and continuous step is to read blog posts and watch videos about API design. The Rust newsletter often has an article listed in its weekly edition. But other platforms such as HackerNews will have interesting Rust articles pop up as well. <https://sabrinajewson.org/blog/errors> is for example a great article talking about one might to give errors as much love as the other parts of the API, and also talks about why crates like `thiserror` might not be that great. All in all, similar to other parts of your continuous Rust learning journey, you'll want to remain critical and open to new ideas and see what works for you and what not.

If you are however still in the camp of `thiserror`, you might be able to make good use of blog posts such as <https://determinate.systems/posts/instrumenting-axum> that guide you through adding instrumentation to your Axum based web service,
using `thiserror` among other excellent crates such as `tracing`. A great read, especially if you're not that experienced with instrumentation
that goes beyon the basic single line logs.
