## Study using the "Zero to Production in Rust" book

> 🔗 <https://zero2prod.com>

Go through the entire book, filling any gaps you might still have and do all the exercises as well.

At the end of the book you should be able to present your own GitHub or GitLab hosted projected based on the exercises and projects of the books, where you applied all your knowledge and also put in something of yourself in it.

> ⓘ Backend frameworks like [Rocket](https://rocket.rs/) and [Actix](https://actix.rs/) are pretty commonly used in books. However know that [the Axum framework](https://docs.rs/axum/latest/axum/) is currently probably your best if you need to choose a backend framework for your next or current Rust project.
>
> Learning Axum in specific could be done by following tutorials
> such as <https://github.com/jeremychone-channel/rust-axum-course>, which seem pretty complete and focussed.
>
> Given how much Axum is built with [Tower in mind, you probably also want to learn that](#tower).

> There's also a "Crust of Rust" episode, by Jon Gjengset, about "decrusting"
> Axum — and by extension _Tower_ — available at <https://www.youtube.com/watch?v=Wnb_n5YktO8>.

Questions you should be able to answer at the end of this step:

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

### Alternatives

The goal of hands-on learning with "Zero to Production" is mostly to help you get started writing some
actual Rust projects, there are however also plenty of alternatives to achieve the same.

- <https://knowledge.dev/> is an interactive book to help you practice Rust by doing projects;
- <https://www.rustadventure.dev/> is a similar concept, which also has a discord and a bit more variety in projects;
- most Rust conferences also have workshops, free or paid, to help you get into Rust or tackle more advanced concepts;
- Shuttle, which allows you to build and ship Rust code, also has a series of tutorials to create, build and ship all kind of projects,
  available at <https://www.shuttle.rs/launchpad>;
- <https://codecrafters.io/> is a paid platform (with very limited free tier) is a platform to help you build projects like your own
  Redis, Git, Grep, Docker and more. And that all using Rust (among other languages available);
