### Rust Atomics and Locks

Understanding concurrency, the primitives used to do so safely and how it all works "under the hood" (e.g. the OS level) is not critical but it will help you big time the rest of your Async professional career.

The book "[Rust Atomics and Locks](https://marabos.nl/atomics/)" by O'Reilly is an amazingly good book by Mara Bos, an important Rust Contributor. Going through the book will take you some time despite its smaller size, but doing so is very rewarding. As with any technical book it is best to read this book as well as do its exercises and apply its knowledge using one program or another. Learn, Apply, Repeat. Play with it, Understand it.

This book is so good, that even Go, Python or JS developers that will never ever touch Rust, might want to learn Rust just to understand the knowledge that can be found in this Gem of a book. Read it. You won't regret it.

As it is more advanced knowledge it can however be seen as "Extra", so in case you are in a hurry to finish [section (3) of the Rust track](#3-learn-async-rust), feel free to skip it. But even if so, keep its existence in mind and come back to it perhaps when you start to get yourself into trouble.

> FYI: if you ever write real-world production low level sync/atomic/concurrent code,
> you might want to check out Loom, as it will help you test that code better then possible with regular tests:
> <https://docs.rs/loom/latest/loom/>.

### Rust Atomics and Locs: Questions

Questions you should be able to answer at the end of this extra step:

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
16. What is Acquire and Release? What is this an example of?
17. When we talk about a "happens-before relationship". What do we mean with it? Why is it important?
18. What is caching in context of your CPU? How does it affect us? What can we do about it?
