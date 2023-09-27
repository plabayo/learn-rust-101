### Appendix IV. Rust in the background

As with any (new) language, there's no point in replacing every existing technology written before, with a new implementation written in Rust. While at times there are benefits such as with [ripgrep](https://github.com/BurntSushi/ripgrep) instead of grep, this is not a generally useful strategy not a desired one.

Starting new projects inspired by classic/foundational tooling is however an opportunity to use Rust, even if your users do not need to know or care about it. A great example of that is just,a modern alternative to Makefiles, as much as love as we have for it. You can find it at <https://github.com/casey/just>, basically 'Make' with variables + more.

- In [Appendix II.](/appendix/appendix-ii-webassembly-wasm.md) we already saw that Rust can be compiled to WASM, and as such you can build an entire WebApp in Rust. However instead of replacing your entire WebApp, it is also just as well possible that you only replace the components of your App where it makes sense, e.g. where you need to do a lot of computations;
- In [Appendix III.](/appendix/appendix-iii-native-apps.md) we learned about Tauri to build native apps with Rust, and also here noted that it can be done while at the same time building your core frontend logic in the frameworks you are used to;

At times however you might be able to incorporate the benefits of Rust in your existing benefits without having to expose the language to anyone:

- [FNM](https://github.com/Schniz/fnm) is a replacement for NVM. While the latter is more powerful the first is a lot faster, which speeds up terminal start up times by a lot, just to give one benefit;
- [Deno](https://deno.land/) can be used as a replacement for NodeJS, and can not only run javascript but also typescript (for which you would otherwise use something like ts-node). It is not a 1-to-1 replacement, and it will require some effort, but it is already today a faster and more secure alternative;
- [SWC](https://swc.rs/) is a speedy light alternative to babel, with the first outperforming the last with its eyes closed and 1 foot still in the bed;

These give just a small taste of what Rust can mean for Web developers even if they do not know or use Rust themselves directly.

In fact some thing most Rust users will be users that do not even know they are using Rust. This is not only True
for the Javascript world but for many others as well, such as Python for example.

- [Pydantic](https://github.com/pydantic/pydantic) is a library for Data validation using Python type hints.
- [Ruff](https://github.com/astral-sh/ruff) is An extremely fast Python linter, written in Rust.

Python web developers can also make use of Rust to write their Python web services, without having to know Rust:

- Granian (<https://github.com/emmett-framework/granian>):
  - A Rust HTTP server for Python applications (written on top of Hyper);
  - Supports ASGI/3, RSGI and WSGI interface applications;
  - Example usage: run your FastAPI web service :)
- Robyn (<https://news.ycombinator.com/item?id=34399125>):
  - A High-Performance, Community-Driven, and Innovator Friendly Web Framework with a Rust runtime.
  - In contrast to `Granian` this does mean you need to write your service using the Robyn way to do things,
    so not possible to use an existing service built with FastAPI.

And plenty more will follow surely.

A language is a tool, but a tool is not to be applied to any problem. Rust has its advantages and use cases, it would be foolish not to use it where desirable.
