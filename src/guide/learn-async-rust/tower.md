### Tower

Once you step into the world of Async Rust, Tokio and Web Services you'll sooner or later come across [Tower](https://github.com/tower-rs/tower), in a similar fashion as [Serde](https://serde.rs/) blew your mind away for your serialization needs. Yes it did, you can admit it, this is a safe space.

> ⓘ Tower is a library of modular and reusable components for building robust networking clients and servers.

With [Tower](https://github.com/tower-rs/tower) everything is a service. A service takes in a request and outputs either an error or a response.
What the actual types for these request-response pairs are, is up to you and depend mostly on the layer it is used.

In tower it is typical that a service wraps another service, called "layers", as such all your services and middleware will stack on top of each other,
like a... _tower_.

- if you are new to Tower, you can start learning how to use it by building your own tower middleware from scratch by following the guide at: <https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md>;
  - if you are not convinced on the usefulness of Tower, you can perhaps read [the "inventing your own Service trait" guide](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait).
 
If you want to get an idea of how `Tower` might look like in a Rust ecosystem since 2024,
you can enjoy this future today already at <https://github.com/plabayo/tower-async>.
Have fun not requiring to implement futures by hand.
