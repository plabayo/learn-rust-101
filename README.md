[![Repository Banner Image](./img/banner.png)](https://rust-lang.guide)

[![GitHub Repo stars](https://img.shields.io/github/stars/plabayo/learn-rust-101?style=social)](https://github.com/plabayo/learn-rust-101) [![GitHub](https://img.shields.io/github/license/plabayo/learn-rust-101)](https://github.com/plabayo/learn-rust-101/blob/main/LICENSE) [![available to hire](https://img.shields.io/badge/Glen-available%20to%20hire%20as%20consultant-green)](mailto:glen@plabayo.tech)

[rust]: https://www.rust-lang.org/

> A guide to aid you in your journey of becoming a Rustacean (Rust developer). See the [Contributing](./CONTRIBUTING.md) and [Code of Conduct](./CODE_OF_CONDUCT.md) for more information about how to contribute to this repository.

> This guide was featured [on the front page of HackerNews on the 8th of April 2023](<https://news.ycombinator.com/item?id=35489029>).

[Rust][rust] is a modern systems programming language with safety in mind as one of its core goals and strengths. Systems programming is programming within a resource contrained environment. However, as a lot of our services run now in paid-for-usage cloud environments, we can also consider them as resource constrained environments. This is why Rust is a great fit for more use cases then people might realize.

As it is a modern language and has taken the lessons from many other languages before it, it is also surprisingly pleasant to use once you get the hang of it. Its type system also allows expressive code that can help you exclude a great categories of bugs beyond the benefits that static typing can bring.

The goal of this guide is to introduce Rust to you as an individual or an entire organization. Should this not be sufficient you can also contact me for 1-on-1 coaching or workshops for your organization, by emailing me at [glen@plabayo.tech](mailto:glen@plabayo.tech).

## Rust 101 Course

The origins of this guide can be found in the preparation of a semester long course I gave to a group of employees at [OTA Insight Ltd.](https://www.ota-insight.com/). The recordings of this course are available on YouTube at: <https://www.youtube.com/playlist?list=PLQgXEsLXFxpVyLddG8FFXfNQEiodTzAjj>.

> This course is still actively being thought and the videos will be uploaded weekly when I find time to upload the recordings.

## Learning Rust in Milestones

```
                                   ┌───────────────────────────┐
                                   │                           │
                           ┌───────┴──────────┐                │
┌───────────┬─────────────►│2. Learn More Rust├─────┐          │
│           │              └────────┬─────────┘     │          │
│     ┌─────┴───────┐               │   ▲           │          │
│     │1. Learn Rust│               │   │           │          │
│     └──┬──┬───────┘               ▼   │           │          │
│        │  │              ┌────────────┴──────┐    │          │
│        │  └─────────────►│3. Learn Async Rust│    │          │
│        │                 └─────────┬─────────┘    │          │
│        │                           │              │          │
│        │                           │              │          │
│        ▼                           ▼              ▼          │
│     ┌────────────────────────────────────────────────────┐   │
│     │4. Study Using the "Zero to Production in Rust" book│◄──┤
│     └──────────────────────────────┬─────────────────────┘   │
│                                ▲   │                         │
│                                │   │                         │
│                                │   ▼                         │
│  ┌─────────────────────────────┴─────────────────────────┐   │
│  │5. Contribute for the first time to an existing project│◄──┘
│  └──┬─────────────────────────────┬──────────────────────┘
│     │                             │
│     │                             ▼
│     │    ┌───────────────────────────────────────────────┐
│     │    │ 6. Study using the "Rust for Rustaceans" Book │
│     │    └────────────────────────┬──────────────────────┘
│     │                          ▲  │
│     ▼                          │  ▼
│ ┌──────────────────────────────┴────────────────────────────┐
└─┤7. Contribute an advanced feature or start your own project│
  └──────────────────────────────────┬────────────────────────┘
                                     │
                                     ▼
                       ┌──────────────────────┐
                       │ 8. Continue to Learn │
                       └──────────────────────┘

      Continiously Learn, Take Your Time
      Enjoy the road :)
```

The above paths are just a suggestion and also should make clear that there are many ways and approaches to using this guide.
Most important is that you do enjoy the journey and that you learn consistently (e.g. ~ X hours per week). How you'll walk the path and how often you revisit things will depend on your learning style and capabilities. There are however no wrong ways to go. E.g. maybe you want to dive into some code yourself as soon as possible and only then will start covering foundations. Or perhaps you first wanna feel very comfortable to only then start really coding yourself once you think you understand the foundations well. All up to you really.

Only thing I would really want to hammer on is to do a lot of coding yourself. The best way to learn is to do. So don't be afraid to make mistakes and to try things out. You'll learn a lot more that way. It will be a very confronting journey but it will be worth it. If all you do is read and read you'll never really get a feel for the language and you'll never really get a feel for what you can do with it. So please, do a lot of coding yourself. Plus at times we can trick our brain into thinking we understand something when we don't.

Also even if you start to reach advanced topics (e.g. Rust for Rustaceans), it is still very important to keep doing the basics. You'll be surprised how quickly you can forget things. So keep doing the basics and keep doing the advanced topics. It is a never ending journey.

If you ever feel stuck, or want a guide, teacher or mentor. Feel free to reach out to Glen at [glen@plabayo.tech](mailto:glen@plabayo.tech) to book a 1-on-1 session or a workshop for your organization. It can be a one time thing or we can meet regularly to help you along the way.

## Sponsors

Support this project by becoming a [sponsor](https://github.com/sponsors/plabayo).

## Index

- [About](#about)
- [1. Learn Rust](#1-learn-rust)
- [2. Learn More Rust](#2-learn-more-rust)
  - [Learn More Rust: Extra](#learn-more-rust-extra)
  - [Code like a pro in Rust](#code-like-a-pro-in-rust)
  - [A Crust of Rust](#a-crust-of-rust)
  - [API Design](#api-design)
- [3. Learn Async Rust](#3-learn-async-rust)
  - [Tower](#tower)
  - [Rust Atomics and Locks](#rust-atomics-and-locks)
- [4. Study using the "Zero to Production in Rust" book](#4-study-using-the-zero-to-production-in-rust-book)
- [5. Contribute for the first time to an existing project](#5-contribute-for-the-first-time-to-an-existing-project)
- [6. Study using the "Rust for Rustaceans: Idiomatic Programming for Experienced Developers" book](#6-study-using-the-rust-for-rustaceans-idiomatic-programming-for-experienced-developers-book)
- [7. Contribute an advanced feature to an existing project or start a project from scratch](#7-contribute-an-advanced-feature-to-an-existing-project-or-start-a-project-from-scratch)
- [Next Steps](#next-steps)
- Appendix:
  - [Appendix I. Install Rust](#appendix-i-install-rust)
    - [Linting and styling](#linting-and-styling)
  - [Appendix II. WebAssembly (WASM)](#appendix-ii-webassembly-wasm)
    - [WASM Learning Resources](#wasm-learning-resources)
  - [Appendix III. Native Apps](#appendix-iii-native-apps)
    - [Tauri](#tauri)
  - [Appendix IV. Rust in the background](#appendix-iv-rust-in-the-background)
  - [Appendix V. Python / Javascript developers](#appendix-v-python--javascript-developers)
  - [Appendix VI. More Material](#appendix-vi-more-material)
    - [Keep up to date](#keep-up-to-date)
    - [Lists](#lists)
    - [Read](#read)
    - [Educational](#educational)
  - [Appendix VII. Community Chat](#appendix-vii-community-chat)

## About

[Glen De Cauwsemaecker](https://www.glendc.com/) created this Educational track to help you get started with [Rust][rust] from an absolute beginner all the way until you're an expert in it. He is open to mentor people, lead workshops and more. If you're a team lead that wants to introduce Rust to their team, feel free to talk to him. Glen started with Rust around 2015, when Rust was still unstable and fast moving. Coming from a C++ background in a system programming job the benefits were immediately clear to him. In the meanwhile things have changed a lot and many companies have been starting to adopt Rust in their toolset:

- Amazon Web Services (AWS) has used Rust since 2017 for its serverless computing offerings, AWS Lambda and AWS Fargate. With that, Rust has gained further inroads. The company has written the Bottlerocket OS and the AWS Nitro System to deliver its Elastic Compute Cloud (EC2) service.
- Cloudflare develops many of its services, including its public DNS, serverless computing, and packet inspection offerings with Rust.
- Dropbox rebuilt its backend warehouse, which manages exabytes of storage, with Rust.
- Google develops parts of Android, such as its Bluetooth module, with Rust. Rust is also used for the crosvm component of Chrome OS and plays an important role in Google's new operating system, Fuchsia.
- Facebook uses Rust to power Facebook's web, mobile, and API services, as well as parts of HHVM, the HipHop virtual machine used by the Hack programming language.
- Microsoft writes components of its Azure platform including a security daemon for its Internet of Things (IoT) service in Rust.
- Mozilla uses Rust to enhance the Firefox web browser, which contains 15 million lines of code. Mozilla's first two Rust-in-Firefox projects, its MP4 metadata parser and text encoder/decoder, led to overall performance and stability improvements.
- GitHub's npm, Inc., uses Rust to deliver "upwards of 1.3 billion package downloads per day."
- Oracle developed a container runtime with Rust to overcome problems with the Go reference implementation.
- Samsung, via its subsidiary SmartThings, uses Rust in its Hub, which is the firmware backend for its Internet of Things (IoT) service.
- [The U.S. Department of Commerce's National Institute of Standards and Technology (NIST) has added Rust to its list of "Safer Languages" as part of its Software Assurance Metrics and Tool Evaluation (SAMATE)](https://foundation.rust-lang.org/news/rust-identified-as-safer-coding-tool-by-nist/).

The language and its ecosystem have also very matured. Concepts like Async have also landed and Async runtimes such as Tokio have become stable and can be used without fear. While it was harder to convince companies to jump on the Rust wagon in the past, by now it should be a lot easier to sell. Is it one language to replace them all? Of course not, but neither should it be overlooked. Are you not sure how Rust might benefit your team for one thing or another? Contact [Glen](https://www.glendc.com/) and figure it out together.

If you are a bit of a history nerd you might also enjoy:

- this article: [How Rust went from a side project to the world's most-loved programming language](https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/)
- or this podcast: [History of Rust with Ben Striegel — Rustacean Station](https://rustacean-station.org/episode/042-ben-striegel/)

There's a [last section on the end of the Rust learning content](#appendix).
This one contains useful references and sources that you can check as part of your
continuous journey as a capable Rust developer.

In case you are a Python or Javascript developer you might find the [Appendix V. Python / Javascript developers](#appendix-v-python--javascript-developers) section useful.

## 1. Learn Rust

In case you're new to the language we suggest you to take a look at [Learn Rust](https://www.rust-lang.org/learn)  (free):

- [The Rust Programming Language - The Rust Programming Language](https://doc.rust-lang.org/book/) gives you a very nice overview;
  - There are plenty of exercises in the book and at the end of your learning journey you'll get to [build your own multi-threaded web server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)!
- [GitHub - rust-lang/rustlings](https://github.com/rust-lang/rustlings): Small exercises to get you used to reading and writing Rust code! is an alternative more pragmatic approach on learning the language;
- [Introduction - Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/) is there for people who like learning from real-world examples;

Sooner or later you might also need help for a project you work on or simply to learn a concept you do not really understand. Not everybody has a senior or friend around that can help them out with this. For all of this and more you want to probably join one or multiple of the community chat servers listed in [Appendix VII. Community Chat](#appendix-vii-community-chat).

> ⓘ **Hardcore** alternative
>
> [Programming Rust, 2nd Edition](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) (O'Reilly Publishing) is a big book and will take you some time to get through. It is not for the faint of heart. However… If you do choose for this route as an alternative to [(1) Learn Rust](#1-learn-rust), you will absolutely not regret it. In fact, your entire rest of the Journey will be a breeze.
>
> The "Programming Rust, 2nd Edition" book is a gem and in case you can handle big dry Technical books such as these it is one that will give you just as much love back as the energy that you put into it. Take your time, get through it, play with it, and enjoy. In fact, it can be easily paired with Rustlings by doing the exercises linked to the content you're reading. Rust by Example can also be added on top of that to see some more code related to the stuff your learning.
>
> Soak it in. It's a hardcore alternative, but if you're up for it, it's there for you to grab.
>
> If you've never done any Systems programming before, this will be an especially helpful book given it will explain a lot of the magic you've encountered in your very protected professional life so far. You're welcome.

Questions you should be able to answer at the end of this step:

1. Rust is a Memory-Safe language. How? Why? In what way is this different from other Memory Safe languages such as Go?
2. Rust is a performant language. Why can it claim this. How?
3. How do you structure data in Rust?
4. Is data passed by Reference or value in Rust? And what does it mean?
5. Is data mutable or immutable by default? How do we make it the other way?
6. What are traits and how are these related to the data models that you structure in Rust?
7. How can you extend the behaviour of external types?
8. Can you extend behaviour of external types using external traits? Why or why not?
9. What is a constant? Where can we define these?
10. What is a static variable? How is it different from a constant? What's special about mutable variables?
11. How does Rust support Async Rust? How do you use it? What are Futures?
12. How do you achieve parallel programming in Rust?
13. What's the difference between async programming and parallel programming? How are they related?
14. What is the borrow checker? What is ownership? How are these concepts related?
15. Data can be moved and it can also be dropped. What do we mean with this? How do we have control over this?
16. How do we achieve shared mutable references to the same data? How can this be done safe?
17. What are lifetimes? How do they work? Can you give some examples of where these are used?
18. What are generics? Why do we use them? 
19. What are Rust Macros? When do we use these? What type of Macros are there?
20. How do you build Rust projects?
21. How do we use dependencies of other developers into our projects? And how do we call such dependencies?
22. What are sum types? What are product types?
23. What is an Enum in Rust and how can we use it? How can we define it?
24. What is pattern matching in Rust? How do we do it? How is it more powerful than a switch?
25. How do we write tests in Rust? What kind of tests are there?
26. What are closures and how are they related to functions? What they have in common? How do they differ?
27. What kind of smart pointers does the Rust std library provide you? What are there use cases, differences, and so on?
28. What is unsafe Rust? How do we use it? When do we use it? What guarantees we still get, even when using unsafe Rust?

### Alternative or Companion resources

[Welcome to Comprehensive Rust 🦀 - Comprehensive Rust 🦀](https://google.github.io/comprehensive-rust/): a small bootcamp course by Google focussed on Android Engineers. Most of it is however applicable to anyone coming to Rust from another language and could be a great alternative or companion for some of the resources above.

## 2. Learn more Rust

- [Introduction - The Cargo Book](https://doc.rust-lang.org/cargo/index.html): Cargo is the Rust package manager. Cargo downloads your Rust package's dependencies, compiles your packages, makes distributable packages, and uploads them to crates.io: [Rust Package Registry](http://crates.io/), the Rust community's package registry;
- [Getting started - Command Line Applications in Rust](https://rust-cli.github.io/book/index.html): Gives you another practical approach in your Rust Learning, by specifically learning how to write CLI applications in it;

Questions you should be able to answer at the end of this step:

1. Make sure to be able to answer all questions of step (1).
2. What is cargo and why does it exist?
3. What kind of extensions exist for Cargo, how do we install them, how do we use them and what are some commonly used ones?
4. What is the Cargo.toml file?
5. What is the Cargo.lock file?
6. What are build scripts? Why do we use these? How do we use these?
7. How can we build code? How do we test code?
8. How can we check code without building the final binaries?
9. How do we make a simple CLI application? How do we pass arguments and flags? How do we handle signals?
10. How can we structure our code and package(s)?
11. What does public and private mean in the context of Rust? At what scope do these operate?

### Learn More Rust: Extra

Once you went through the resources in the previous two chapters you might want to start working on a real project to start applying and solidifying your knowledge. A classic way to do this is by porting a small specialized codebase from one language to Rust, keeping in mind to not 1-to-1 map the lines of code but to instead focus on porting the logic with the "Rust way to do things" in mind.

Should you want some inspiration or a more guided approach, here are some resources that could help you out with this extra optional, but recommended step:

- [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/): build a cli tool such as grep in rust, using test driven development (TDD), all to learn Rust
- [Rust Application Books - The Little Book of Rust Books](https://lborb.github.io/book/applications.html): lists a lot of different books, including ones that allow you to develop small projects using Rust, usually in a well guided fashion;
  - [Introduction - PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/introduction.html) is an especially fun small project. It allows you to apply the knowledge that you learned above in a very narrow program, that is for once not network related. At the end you'll have a cli tool that allows you to encode and decode "hidden" messages in a PNG image;
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

If you have a lot of free time at your hand and you want to build something really cool that is totally useless, here are some more ideas:

- Build your own Gameboy (classic) emulator: [Introduction - DMG-01: How to Emulate a Game Boy](https://rylev.github.io/DMG-01/public/book/introduction.html)
- Build your own CHIP-8 (8-bit computer) interpreter: [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) (no Rust guidance, directly from the CHIP-8 specification instead, you can do it, I believe in you)
- Build your own Ray Tracer (not a Rust tutorial, but can easily be implemented in Rust as well): [Ray Tracing in One Weekend Series](https://raytracing.github.io/) (fun if you always had the dream to start dabbling into the world of Graphics Programming)

Seriously though, do not consider the above recommended or mandatory in any way. If you however really like to develop stuff and you do like to do it extensively in your free time, then, and only then, I do believe that the above are a great way to really solidify your current Rust knowledge and give yourself a great (pragmatic) foundation.

### Code like a pro in Rust

At this point of your Rust learning journey you've come a far way already, relatively speaking. Perhaps you want to shortcut, at least for now.

If so, [the still to be published "Code like a pro in Rust" book](https://www.manning.com/books/code-like-a-pro-in-rust) might be enough for you to quickly get from beginner to advanced Rust programmer.

### A Crust of Rust

Once you get at this point you are around the level of a beginner or intermediate Rust programmer. At this point you could already start to tackle stuff like ["Rust for Rustaceans" of step (6)](#6-study-using-the-rust-for-rustaceans-idiomatic-programming-for-experienced-developers-book). However it might be a bit much.

Therefore while you go through the next couple of sections it could be helpful to now and then (e.g. while washing dishes or instead of Netflix) an education Rust code video. In the appendix at the bottom of this page there a couple of such suggestions.

Particular to this context you might want to save [the "Crust of Rust" playlist](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa). Each video will go over a specific topic, e.g. Lifetimes, macros, Atomics, … No need to watch them one by one, feel free to jump to them in whatever orders and only those that interest you (which is the same tip that I can give for the "Rust for Rustaceans" book).

### API Design

Designing APIs is a big part of being a Rust programmer, any programmer really. Once you start to get comfortable enough in Rust or start building your own public crate, it is a topic you'll want to invest in.

A good starting point is the [API Guidelines](https://rust-lang.github.io/api-guidelines/about.html) which are a set of guidelines that are meant to help you design good APIs. They are not meant to be followed blindly, but rather to be used as a reference to help you make the right decisions.

A next and continuos step is to read blog posts and watch videos about API design. The Rust newsletter often has an article listed in its weekly edition. But other platforms such as HackerNews will have interesting Rust articles pop up as well. <https://sabrinajewson.org/blog/errors> is for example a great article talking about one might to give errors as much love as the other parts of the API, and also talks about why crates like `thiserror` might nog be that great. All in all, similar to other parts of your continuous Rust learning journey, you'll want to remain critical and open to new ideas and see what works for you and what not.

## 3. Learn Async Rust

There is no standard / official Asynchronous runtime. This in contrast to actual CPU threads [which you can create from within the std library](https://doc.rust-lang.org/std/thread/fn.spawn.html).

The most popular and recommended by many Async Runtime is [the Tokio runtime](https://tokio.rs/). Which brings us to the next chapter:

- To start with your Async Learning Journey you might want to start with: [Getting Started - Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/);
- Afterwards you can finally start going through the Tokio [learning journey Tutorial | Tokio - An asynchronous Rust runtime](https://tokio.rs/tokio/tutorial) 

Please also join the Tokio Discord community as part of your async Journey, a place where you can ask any questions you want as well: <https://discord.com/invite/tokio>. This server also has also individual channels for the different projects in the Tokio ecosystem such as Hyper, Axum, Tower and more. The maintainers of the different projects are also very active in the server and are happy to help you out, but of course please do not take this for granted. Be respectful and if possible contribute back to the community as well.

As an extra you may also want to read and develop alongside the following articles:

- [The HTTP crash course nobody asked for](https://fasterthanli.me/articles/the-http-crash-course-nobody-asked-for)
- [Understanding Rust futures by going way too deep](https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep)

In order to help you understand how Async code works (e.g. what about the Async keyword and How do futures really work. And how do you define traits with futures. And How do you implement them), you might want to read through the articles linked in the [Tower](#tower) section, as seeing how to implement your own _Tower_ middleware might answer many of such questions. Gaining a deeper theoretical but still pragmatic enough understanding behind it is probably better done by reading some of the books listed in this guide.

> ⓘ Note: Tokio also provides support to [the new Linux kernel io_uring concept](https://unixism.net/loti/), a new powerful way to allow async programming on Linux. Support for it can be found at: [GitHub - tokio-rs/tokio-uring: An io_uring backed runtime for Rust](https://github.com/tokio-rs/tokio-uring)
> 
> Synacktiv made an IO network scanner using io_uring as can be seen at [GitHub - synacktiv/io_uring_scanner: io_uring based network scanner written in Rust](https://github.com/synacktiv/io_uring_scanner), using tokio's low level userspace bindings to io_uring.

### Questions you should be able to answer at the end of this step:

1. How does Async programming work in Rust, if you didn't understand this already?
2. What types of concurrent programming does Rust support? How many are there in general?
3. What are Futures and how do they work? What function they have? How do they affect you?
4. What is Send and Sync? What is it used for? How does it affect you?
5. What are good use cases of Async code? What kind of tasks aren't a good fit for this?
6. What are Async Executors? What are Async Runtimes?
7. What is Tokio? How do you use it? What does it give you?
8. What are some other libraries in the Tokio ecosystem?
9. What can you build with Tokio?
10. Why would you use Tokio?
11. How do you achieve Rust programming with only standard code (meaning code developed by the Rust core team)?
12. Why does Rust not bundle an Async runtime?
13. What is parallel programming? How do you do it in Rust? How does it relate to Async programming?

### Tower

Once you step into the world of Async Rust, Tokio and Web Services you'll sooner or later come across [Tower](https://github.com/tower-rs/tower), in a similar fashion as [Serde](https://serde.rs/) blew your mind away for your serialization needs. Yes it did, you can admit it, this is a safe space.

> Tower is a library of modular and reusable components for building robust networking clients and servers.

With [Tower](https://github.com/tower-rs/tower) everything is a service. A service takes in a request and outputs either an error or a response.
What the actual types for these request-response pairs are, is up to you and depend mostly on the layer it is used.

In tower it is typical that a service wraps another service, called "layers", as such all your services and middleware will stack on top of each other,
like a... _tower_.

- if you are new to Tower you can start learning how to use it by building your own tower middleware from scratch by following the guide at: <https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md>;
  - if you are not convinced on the usefulness of Tower you perhaps read to [the "inventing your own Service trait" guide](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait).

### Rust Atomics and Locks

Understanding concurrency, the primitives used to do so safely and how it all works "under the hood" (e.g. the OS level) is not critical but it will help you big time the rest of your Async professional career.

The book "[Rust Atomics and Locks](https://marabos.nl/atomics/)" by O'Reilly is an amazingly good book by Mara Bos, an important Rust Contributor. Going through the book will take you some time despite its smaller size, but doing so is very rewarding. As with any technical book it is best to read this book as well as do its exercises and apply its knowledge using one program or another. Learn, Apply, Repeat. Play with it, Understand it.

This book is so good, that even Go, Python or JS developers that will never ever touch Rust, might want to learn Rust just to understand the knowledge that can be found in this Gem of a book. Read it. You won't regret it.

As it is more advanced knowledge it can however be seen as "Extra", so in case you are in a hurry to finish [section (3) of the Rust track](#3-learn-async-rust), feel free to skip it. But even if so, keep its existence in mind and come perhaps back to it when you start to get yourself into trouble.

#### Questions you should be able to answer at the end of this extra step:

1. What primitives are there in general to achieve concurrency?
2. What primitives does the OS give you?
3. What is the difference between threads and async programming?
4. Why is async programming faster?
5. When would you use threads? When would you use async programming?
6. What's a Mutex? How could you implement one?
7. What's a Conditional Variable? How could you implement one?
8. What's a Signal? How could you implement one?
9. What's a Channel? How do you implement it? What types of channels are there?
10. What is Reordering? How can we deal with this? How does it affect us?
11. How can we efficiently make a thread go to "Sleep" and have it woken up again?
12. What's a Spin Lock? How do we build one?
13. What's an atomic? What kind of atomics are there? How do they work under the hood?
14. How does Rust achieve thread safety?
15. What's Memory Ordering? What's the Memory Model? What types are there? Which ones can you use?
16. What is Aquire and Release? What is this an example of?
17. When we talk about a "happens-before relationship". What do we mean with it? Why is it important?
18. What is caching in context of your CPU? How does it affect us? What can we do about it?

## A Note About the remaining part of this guide

The next sections are all optional and are not required to really be productive in Rust. Some of it are alternative resources or extra content to learn from. Others is just miscellaneous content that is not really required to be productive in Rust, but might be interesting to you.

This guide is either way not meant to be read in a single session or series. Take it as yours and just use it as part of your learning journey. Take your time as learning Rust isn't easy, but it is a very rewarding one once you did climb the mountain and are dancing with a double rainbow in the background.

## 4. Study using the "Zero to Production in Rust" book

Go through the entire book, filling any gaps you might still have and do all the exercises as well.

At the end of the book you should be able to present your own GitHub or GitLab hosted projected based on the exercises and projects of the books, where you applied all your knowledge and also put in something of yourself in it.

> ⓘ Backend frameworks like [Rocket](https://rocket.rs/) and [Actix](https://actix.rs/) are pretty commonly used in books. However know that [the Axum framework](https://docs.rs/axum/latest/axum/) is currently probably your best best if you need to choose a backend framework for your next or current Rust project.

### Questions you should be able to answer at the end of this step:

1. How can we write a Full stack app in Rust?
2. How can we deploy and host a Full stack Rust app in the cloud?
3. How can we add telemetry?
4. How can we interact with Databases using Rust?
5. How can we debug production-quality Rust applications?
6. How can we test Rust applications?
7. How do we make Async applications in Rust?
8. How do we achieve TLS capabilities with Rust?
9. How do we make a web server with Rust?
10. The book uses Actix. What are some alternatives these days?

## 5. Contribute for the first time to an existing project

Contribute to an open source project that you like or may want to strongly depend upon in the future.

Many projects have a `good first issue` label, which is a good place to start. You can also look for issues that are labeled `help wanted` or `easy`.

There's also the Rust Mentorship program, which pairs up new contributors with experienced Rustaceans. You can find more information about it at: <https://rustbeginners.github.io/awesome-rust-mentors/>

In [The weekly Rust Newsletter](https://this-week-in-rust.org/) you can find a list of projects that are looking for contributors.

If you are a seasoned programmer already you might also opt to already start making your own new project or parting a scoped project from another language to Rust.

## 6. Study using the "Rust for Rustaceans: Idiomatic Programming for Experienced Developers" book

> <https://nostarch.com/rust-rustaceans>

Go through the entire book, getting a deep(er) understanding of the advanced concepts and inner workings of them.

At the end of the book you should be able to present your own GitHub or GitLab hosted projected based on the exercises and projects of the books, where you applied all your knowledge and also put in something of yourself in it.

### Questions you should be able to answer at the end of this step:

1. Be able to answer all questions from previous sections, in full confidence and in a lot more depth than you could do before.
2. How is memory laid out for your Rust Data structures? Can you control it? If so, how?
3. Explain in full detail so someone new to Rust can understand it what borrowing and lifetimes are, how do they work, why are they important, how do they affect us, how do we encounter it, why do we use it?
4. What primitive types are there in Rust and how are they laid out in memory?
5. What is the Orphan rule? Why is it important?
6. What is polling in the context of Asynchronous programming?
7. List some additional testing tools beyond unit tests? Why does each one of them mather?
8. What is Pin and Unpin?
  - Note: Rust of Crust also has a great video about it should you need some more explanations about this tricky concept;
9. Why is Asynchronous programming more complicated than parallel programming?
10. How can you test your Asynchronous code? And because it is Rust, what kind of Asynchronous problems do we not need to test if we structure our code right? And why not?
11. In the context of concurrent programming, what are actors?
  - Bonus question: why do some people see this as the future?
12. what is FFI? Why do we use it?
13. What is Rust without the STD? How do we use it and why?
14. What are some common patterns found in the wild and that are discussed in the book?
15. How can you continue to learn Rust?

## 7. Contribute an advanced feature to an existing project or start a project from scratch

Start a project for yourself, your dog, your organization or mother. This might be a microservice, library or small tool. Everything is possible.

Another option is to contribute to an open source project that you like or may want to strongly depend upon in the future.

See [section (5) about your first contribution](#5-contribute-for-the-first-time-to-an-existing-project) for some inspiration on where to start.

## Next Steps

If you have completed all the steps above, you have done more then enough and can call yourself a Rustacean. You can now start to contribute to the Rust ecosystem and help others to learn Rust. Perhaps more importantly, you have all the knowledge you need to build your own projects in Rust.

What you'll build depends on your interest or perhaps in the type of industry or company you're working in.

- If you're doing full stack development you can get inspired on what's next by checking out [WASM Learning Resources Appendix](#wasm-learning-resources);
  - For all your web dev needs you might also get inspired on what frameworks to use by checking out a website such as <https://www.arewewebyet.org/>;
  - By coincidence, some of the books and articles listed in this guide also teach you a lot about web development, so if you did indeed complete these guide you're really already well on your way to becoming a full stack developer in Rust, or backend, whatever floats your boat;
- Some call WASM (on the server side) the next thing after docker containers. So perhaps even as a "pure" backened developer you might want to check out [that _same_ appendix](#wasm-learning-resources) to help you get started;
- If you're a game developer (or inspiring to be) you can check out resources such as <https://arewegameyet.rs/#ecosystem> to figure out what Rust technologies are out there in the ecosystem to help you build the next fantastic game;
  - If you're a game developer you might also want to check out the [Rust GameDev Working Group](https://gamedev.rs/) (they also have a monthly news letter);
  - You could also subscribe to the a podcast about GameDev in Rust called [Rust GameDev Podcast](https://rustgamedev.com/);
- If you want to develop Operating Systems I already listed some resources to help you get started with that.
  - Once (and if) Rust is accepted as a second language (next to C) to develop patches for the Linux kernel you can also start contributing to the Linux kernel in Rust. Future dreams;
  - Or you might want to apply to Google and help build [their Fuchsia OS](https://fuchsia.dev/) in Rust. Have fun;
    - It's open source so nobody can stop you to contribute it either way, well except them I guess;
- If you want to develop blockchain technology... Well, not going to guide you in that one, there's a running joke that it's hard to find a job in Rust that's not blockchain tech, so, you should have no problem finding a job in that field if that's what you want to do;
- If you want to develop embedded systems, you can check out the [Rust Embedded Working Groups's Blog](https://blog.rust-embedded.org/). There are also plenty of companies who sometimes appear in articles or podcasts listed in [the More Material appendix](#appendix-vi-more-material), so you can check that also out for yourself.

There's a lot more of course. You probably know best where you're going. A lot of exciting stuff is happening. Plenty of conferences (e.g. Rust Conf) to help you get inspired on it if you're not already. And of course, plenty of people to help you out if you get stuck. So, good luck and have fun!

## Appendix

### Appendix I. Install Rust

In order to start using Rust you have to install and configure its tooling on your machinery.

- The Rust compiler (`rustc`), standard library and tooling (e.g. `cargo`) are all managed using `rustup`.
  - You can learn how to get started by going to: Getting started 
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

#### Linting and styling

- Install `clippy` using `rustup component add clippy`
  - This way you can use the command `cargo clippy` to help guarantee the quality of your code
- Install `rustfmt` using `rustup component add rustfmt`
  - This way you can use `cargo fmt` to format your code (best to also configure this in your IDE to be done each time you save the file)

No custom linting configurations are agreements are required. If you use these two tools, you're golden. These tools also work nicely with any IDE you might use. Within your CI environment these same tools will be used to check that your code adheres to its standards.

### Appendix II. WebAssembly (WASM)

> WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.
>
> <https://webassembly.org/>

WebAssembly on itself is completely isolated from Rust, but it must be said that among the WASM community, the Rust part is probably the biggest and most active one. Compiling Rust to WASM is a true pleasure and allows for browser client code to be written in Rust, be it a small library to be used from JS or an entire web client. WASM projects can also be used to quickly deploy web services (in the backend). These are similar to lambda functions but are even faster to use and in a way also safer.

#### WASM Learning Resources

- RustWasm book: [Introduction - Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
  - This is the officially recommended book to get you started, as can be seen at <https://www.rust-lang.org/what/wasm> 
- [Programming WASM with Rust: Programming WebAssembly with Rust](https://www.oreilly.com/library/view/programming-webassembly-with/9781680506846/)
- Build a web app with Rust (using Yew framework): <https://kerkour.com/web-application-with-rust-and-webassembly>
- Rust to WebAssembly the hard way: [Rust to WebAssembly the hard way — surma.dev](https://surma.dev/things/rust-to-webassembly/index.html)

Frameworks that you might want to check if you want to make a front-end (web) app using Rust:

- Make desktop apps using web tech (think Electron, but better): [GitHub - tauri-apps/tauri: Build smaller, faster, and more secure desktop applications with a web frontend](https://github.com/tauri-apps/tauri);
- Very popular and fast front-end framework for Rust: [GitHub - yewstack/yew: Rust / Wasm framework for building client web apps](https://github.com/yewstack/yew)
- Another front-end framework: [GitHub - chinedufn/percy: Build frontend browser apps with Rust + WebAssembly](https://github.com/chinedufn/percy). Supports server side rendering. 
- Yet another front-end framework: [GitHub - sycamore-rs/sycamore](https://github.com/sycamore-rs/sycamore): A library for creating reactive web apps in Rust and WebAssembly 
- Exciting new and very fast front-end framework: [GitHub - leptos-rs/leptos: Build fast web applications with Rust](https://github.com/leptos-rs/leptos).
- Also a pretty nice front-end framework: [GitHub - framesurge/perseus: A state-driven web development framework for Rust with full support for server-side rendering and static generation](https://github.com/framesurge/perseus).

Run WASM Backend Services in the cloud:

- Cloudflare workers: [Cloudflare Workers®](https://workers.cloudflare.com/)
- Fermyon Cloud (free for now): [Deploy & manage cloud native WebAssembly apps](https://www.fermyon.com/cloud)
- Wasmer's goal is to run any code on any client (think Java's dream, but for real): <https://wasmer.io/>

As you can see, WASM on the backend it is a thing. Wether it's more then just hype is something the future will tell, but either way a lot of interesting stuff is happening and there's big money behind it somehow. So you might as well jump on the car, especially if you missed the Docker train somehow. Happens.

### Appendix III. Native Apps

Native apps are applications written for a specific platform. While that could be any platform, it usually refers to anything that is not the "web" and even more so, these days, it pretty much always refers to Desktop, Mobile or the combination of. Cross platform apps mean that multiple platforms are supported, which could be any set of web or non-web platforms.

#### Tauri

**Tauri** is the toolset you want to check out if you want to build native apps with a Gui in Rust. You can find all information about it on their official website: <https://tauri.app/>

> Build an optimized, secure, and frontend-independent application for multi-platform deployment.

Tauri applications can be compared to Electron, but it uses a very different approach. Multiple rendering engines are supported, with web view like rendering engines being the default. But this is still different then shipping a Node runtime and Chromium with each and one of your apps. Meaning your desktop applications can be as small as a couple of megabytes, compared to the usually bloated 150MB apps. And once running they will also be a lot less resource hungry compared to your topical Electron app.

Mobile applications are not yet officially supported, but they will soon get support as well, as the underlying technology for it is already more or less ready.

In case you want to write applications that work on both the web and a non-web platform you can achieve this by splitting of your business logic from your minimal front-end logic, such that you can write one front-end in any of the available front-end approaches (e.g. leptos or perseus) and another one using Tauri, with most of your code being available as one or more Rust libraries. 

The front-end logic can be written using any web application front-end framework, e.g. Svelte.

You can also opt to go fully native by using plugins such as tauri-egui: <https://github.com/tauri-apps/tauri-egui>

### Appendix IV. Rust in the background

As with any (new) language, there's no point in replacing every existing technology written before, with a new implementation written in Rust. While at times there are benefits such as with [ripgrep](https://github.com/BurntSushi/ripgrep) instead of grep, this is not a generally useful strategy not a desired one.

- In [Appendix I.](#appendix-i-install-rust) we already saw that Rust can be compiled to WASM, and as such you can build an entire WebApp in Rust. However instead of replacing your entire WebApp, it is also just as well possible that you only replace the components of your App where it makes sense, e.g. where you need to do a lot of computations;
- In [Appendix II.](#appendix-ii-webassembly-wasm) we learned about Tauri to build native apps with Rust, and also here noted that it can be done while at the same time building your core frontend logic in the frameworks you are used to;

At times however you might be able to incorporate the benefits of Rust in your existing benefits without having to expose the language to anyone:

- [FNM](https://github.com/Schniz/fnm) is a replacement for NVM. While the latter is more powerful the first is a lot faster, which speeds up terminal start up times by a lot, just to give one benefit;
- [Deno](https://deno.land/) can be used as a replacement for NodeJS, and can not only run javascript but also typescript (for which you would otherwise use something like ts-node). It is not a 1-to-1 replacement, and it will require some effort, but it is already today a faster and more secure alternative;
- [SWC](https://swc.rs/) is a speedy light alternative to babel, with the first outperforming the last with its eyes closed and 1 foot still in the bed;

These give just a small taste of what Rust can mean for Web developers even if they do not know or use Rust themselves directly.

A language is a tool, but a tool is not to be applied to any problem. Rust has its advantages and use cases, it would be foolish not to use it where desirable.

### Appendix V. Python / Javascript Developers

In case you come from Python or Javascript you can alternatively replace the first four steps of this learning guide with a single book. [Rust Web Programming](https://www.packtpub.com/product/rust-web-programming-second-edition/9781803234694) (Packt Publishing) aims to get web developers (including Python developers) get started with Rust and has a very pragmatic approach.

Given how important Async is in our work environment it is still worth to go through section [(3) "Learn Async Rust"](#3-learn-async-rust) to make sure you do really grasp that very well. It is also where a lot of your Rust knowledge starts to come together, so if you get that you know you're on the right track.

Should you after completing this book (even if partly) want to have some more hands on experience you can opt to never the less do section (4) as well and go through [the "Zero to Production in Rust" book](#4-study-using-the-zero-to-production-in-rust-book)

However, in case you are confident you did get all the foundational concepts of Rust and want to just develop stuff already and get it "into your fingers" you might want to opt to after this (and before jumping onto section (5)) read ["Rust In Action"](https://www.manning.com/books/rust-in-action) first. It gives a lot of fun things to build. Alternatively you can check the "Learn More Rust: Extra" part of [section (2) Learn more Rust](#2-learn-more-rust), where we also go over many ideas for you to get more experience and help you solidify your Rust knowledge.

The "Rust in Action" book can also be seen as yet another alternative all together, as it is also aimed at introducing Rust to people who never did any Systems programming, So you could also do it instead of the Rust Web Programming book.

### Appendix VI. More Material

Learning to master a language has a beginning but it doesn't have an ending. You'll want to continue to learn, teach others and keep up to date. In this appendix you'll find material to help you with all this and more.

#### Keep up to date

Official Blogs:

- [The Rust Programming Language Blog](https://blog.rust-lang.org/)
  - e.g. in October of 2022 GAT's (Generic Associated Types) were finally stabilised, after 6+ years of work: [Generic associated types to be stable in Rust 1.65 | Rust Blog](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html)
  - [The "Inside Rust" Blog](https://blog.rust-lang.org/inside-rust/index.html)

Conferences:

- RustConf#2022: <https://rustconf.tilde.io/courses/enrolled/1857336>

Podcasts:

- Rustacean Station is a pretty great podcast with very interesting guests and even more interesting knowledge to share. Listen to it, you won't regret: <https://rustacean-station.org/>
- In order to learn Rust you can also listen to the New Rustacean podcast, which is a podcast about learning Rust: <https://newrustacean.com/>
  - It no longer airs, but its content remains useful and available

Actuality:

- ["This week in Rust"](https://this-week-in-rust.org/) is a weekly newsletter helping you stay up to date with Rust from the lazy comfort of your Mailbox. Easy peasy. They can also be read online in case you prefer a browser instead of the web or do not want to subscribe for w/e reason.
  - Blog articles from the community get shared here as well, some accessible to new comers to Rust, others a bit more advanced
  - They also share VOD's of recent conferences, e.g. the issue in which they share the rust talks from [FOSDEM 2023: This Week in Rust 481  · This Week in Rust](https://this-week-in-rust.org/blog/2023/02/08/this-week-in-rust-481/)
- ["Rust Magazine"](https://rustmagazine.org/): an online magazine dedicated to Rust

#### Lists

community curated crate lists:

- [Crate List - Blessed.rs](https://blessed.rs/crates)

#### Read

Free Rust Resources that we haven't mentioned yet, but are great to read at one point or another:

- The Rust Performance Book: <https://nnethercote.github.io/perf-book/>
- Rust Fuzz Book (Fuzz Testing): <https://rust-fuzz.github.io/book/>
- Idiomatic solutions to common programming tasks: <https://rust-lang-nursery.github.io/rust-cookbook/>
- The little book of Rust books: [Unofficial Rust Books - The Little Book of Rust Books](https://lborb.github.io/book/unofficial.html) (which we also already linked in an earlier section)

If you learn by doing and haven't gotten enough from the resources we linked already in "Learn more Rust", here are some more tips:

- Implement a Redis client and server using Tokio, a very well documented and excellent guide on how to write Asynchronous code in Rust: <https://github.com/tokio-rs/mini-redis/> - for learning purposes only 
- If you want to improve your borrowing and reference skills, implementing a Linked List data structure might be for you: <https://rust-unofficial.github.io/too-many-lists/>
- Or perhaps you want to write your own OS in Rust: <https://os.phil-opp.com/>

A blog you might want to RSS subscribe to is Amos's his blog which is always fun to read, be it sometimes a bit heavy: <https://fasterthanli.me/tags/rust/> 

A nice reference sheet if you need one can be found at: <https://cheats.rs/>

#### Educational

- Crust of Rust video series by "Jon Gjengset", for those that want to see an experienced Rust Developer work through problems: [Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa)
  - You can combine it while learning a bit about networking and implement TCP in Rust yourself: <https://www.youtube.com/playlist?list=PLqbS7AVVErFivDY3iKAQk3_VAm8SXwt1X>
- If you like to learn by watching you can find a Rust streaming list at <https://github.com/jamesmunns/awesome-rust-streaming/>
- Interesting youtube channel with a lot of great stuff to learn about Rust: <https://www.youtube.com/@NoBoilerplate>
- If any of the basic (textual) learning resources from the actual learning journey didn't quite work for you, you might find a video course more useful. If so, here are two of them:
  - [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course/) 
  - [Ultimate Rust 2: Intermediate Concepts](https://www.udemy.com/course/ultimate-rust-2/)
- If you need inspiration on things you can build with Rust to get the language in your fingers as well as learn how software that you use is implemented, you might want to check out: <https://app.codecrafters.io/tracks/rust>
- Learn Full Stack Rust Interactively: [Rust Insight! - an interactive book for practicing Rust on the laptop](https://rustinsight.com/) (early access)
- A big index of many more learning resources, named aptly "How to learn Modern Rust — A Guide to the adventurer": <https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust>
  - Do not take the list as-is and use mostly as inspiration. No need to really do all on that list, but if you need more learning resources, there is plenty of good content and inspiration in there!
- Black Hat Rust book: <https://kerkour.com/black-hat-rust> (could be nice for inspiration on some hands-on experience while becoming more familiar with Rust)
- If you want to test your Rust concept skills you might also enjoy taking [the Rust Quiz](https://dtolnay.github.io/rust-quiz/). The more answers you have wrong the better, as it means you'll learn new things from the explanation :)
- If you want to learn by doing by contributing to an existing Rust OSS project (including the Rust language and tooling) you can try to find a mentor to help you for free online in achieving this, by going to Awesome Rust Mentors: <https://rustbeginners.github.io/awesome-rust-mentors/>

### Appendix VII. Community Chat

Learning from and working with a community is one of the many perks of Rust, which is rightfully known for its warm, inclusive and charming community. Always open and friendly to others.

Here are some chat servers you can join to interact with this fantastic community reward. All of them are great places to meet other Rust developers, find help, share news or a project and also to give back to the community yourself:

- Discord chat server for the Tokio ecosystem and its community: <https://discord.com/invite/tokio>;
- Discord chat server for the Rust community in general (has a great channel for beginners to find help, and in a very fast manner): <https://discord.com/invite/rust-lang>;
- zulip chat server for Rust, great for core work and advanced purposes: <https://rust-lang.zulipchat.com/>.

There are others of course but these are the one which are both active, inclusive, friendly and known to the authors of this guide as being a great place to be and be part of.
