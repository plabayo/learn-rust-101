### Appendix II. WebAssembly (WASM)

> ⓘ WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.
>
> 🔗 <https://webassembly.org/>

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
- If you are looking for a complete framework you  might want to give <https://github.com/framesurge/perseus> a chance, it builds on top of Sycamore;
- Exciting new and very fast front-end framework: [GitHub - leptos-rs/leptos: Build fast web applications with Rust](https://github.com/leptos-rs/leptos).
- Another front-end framework (or perhaps rather a GUI framework) that you might want to know about is Dioxus (<https://dioxuslabs.com/>) which is made not only for web-apps, but also with CLI, mobile and Desktop in mind.
- Also a pretty nice front-end framework: [GitHub - framesurge/perseus: A state-driven web development framework for Rust with full support for server-side rendering and static generation](https://github.com/framesurge/perseus).

Run WASM Backend Services in the cloud:

- Cloudflare workers: [Cloudflare Workers®](https://workers.cloudflare.com/)
- Fermyon Cloud (free for now): [Deploy & manage cloud native WebAssembly apps](https://www.fermyon.com/cloud)
- Wasmer's goal is to run any code on any client (think Java's dream, but for real): <https://wasmer.io/>

As you can see, WASM on the backend is a thing. Whether it's more then just hype is something the future will tell, but either way a lot of interesting stuff is happening and there's big money behind it somehow. So you might as well jump in the car, especially if you missed the Docker train somehow. Happens.
