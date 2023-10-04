### Appendix IX. Shipping Rust

Shipping Rust does not have to be different from shipping code in other languages.
So all the knowledge you have or will learn about the Cloud, Docker, Bare Metal, Vms,
Lambda functions, WASM workers, containers, and whatever else also can be applied here.

If you however find cloud platforms like AWS or GCloud a bit dounting (or have choice stress),
and find deploying to your own VPS or VM a bit too barebones, you might find some of the following
solutions to be helpful, specifically with the goal of shipping rust code such as a web service:

- <https://fly.io/>: allows you to ship rust code fairly painlessly by using docker containers;
- <https://render.com/> ship rust code without having to define a docker comtainer and with tls, cdn, db and other capabilities that can be enabled from an easy to use WebUI;
- <https://www.shuttle.rs/> has a pretty unique approach by letting you deploy your Rust code directly, without containers,
  and by adding your stack dependencies (e.g. database needs) as macros to your Rust `main.rs` code;
- Using <https://github.com/cloudflare/workers-rs> you can ship your Rust code to Cloudflare's workers platform;
- <https://www.fermyon.com/#> is another similar cloud platform that also allows you to ship Rust (WASM) workers;

If you want to ship on a more standard cloud platform such as _AWS_
you can learn to do so as part of practicing Rust using "[4. Study using the "Zero to Production in Rust" book](#4-study-using-the-zero-to-production-in-rust-book)".

Finally, in the spirit of KISS, nothing wrong with deploying like this: <https://logankeenan.com/posts/deploy-your-rust-project-to-any-hosting-provider-in-minutes/>.
