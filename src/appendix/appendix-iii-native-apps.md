### Appendix III. Native Apps

Native apps are applications written for a specific platform. While that could be any platform, it usually refers to anything that is not the "web" and even more so, these days, it pretty much always refers to Desktop, Mobile or the combination of. Cross platform apps mean that multiple platforms are supported, which could be any set of web or non-web platforms.

> If you ever need to interact with other native languages:
>
> - For C you probably want to just use bindgen;
> - For C++ you can make use of <https://github.com/dtolnay/cxx> (cxx);

#### Tauri

**Tauri** is the toolset you want to check out if you want to build native apps with a Gui in Rust. You can find all information about it on their official website: <https://tauri.app/>

> ⓘ Build an optimized, secure, and frontend-independent application for multi-platform deployment.

Tauri applications can be compared to Electron, but it uses a very different approach. Multiple rendering engines are supported, with web view like rendering engines being the default. But this is still different then shipping a Node runtime and Chromium with each one of your apps. Meaning your desktop applications can be as small as a couple of megabytes, compared to the usually bloated 150MB apps. And once running, they will also be a lot less resource hungry compared to your topical Electron app.

Mobile applications are not yet officially supported, but they will soon get support as well, as the underlying technology for it is already more or less ready.

In case you want to write applications that work on both the web and a non-web platform you can achieve this by splitting your business logic from your minimal front-end logic, such that you can write one front-end in any of the available front-end approaches (e.g. leptos or perseus) and another one using Tauri, with most of your code being available as one or more Rust libraries. 

The front-end logic can be written using any web application front-end framework, e.g. Svelte.

You can also opt to go fully native by using plugins such as tauri-egui: <https://github.com/tauri-apps/tauri-egui>
