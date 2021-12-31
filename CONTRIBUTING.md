# Contribution guidelines

So you want to contribute with this project. That's cool ^^.

Please read the items in the toc so you'll can help us on mergin you PR or fixing a bug that you've found.

## Table of Contents
  - [Bug reporting](#bug-reporting)
  - [New features](#new-features)
  - [Coding improvement](#code-improvement)
  - [Project guide](#project-internals)


## Bug reporting
 Before reporting check if it was already reported on the [issues tab](https://github.com/0xc0ffeec0de/opm/issues).
 If you got faced by a bug, you can open an issue describing the problem. When reporting it's always good to give some context, like when, where and what was expected then.
 So things like what was you expecting to get, your operation system and version, rustc, toolchain and cargo version, if got some unexpected `panic!` enable the `RUST_BACKTRACE=full` environment variable.
 Basically the more info you gave the better.
 
 For secutiry bugs check [SECUTIRY.md](SECURITY.md) for more information.
 
## New features
  You got an idea of a good feature that would be good in `opm`. We appreciate it ^-^.
  Now you have two ways of see that new feature in the project:
  1. Sending a PR
  2. Open an Issue
  
  **Note:** Both should be tagged with `suggestion`.
  
  Then we'll analize and see if it's possible to implement that feature you suggest.
 
## Code improvement
  When you was looking through the code you thought _"I can do this better"_. We dont mind, go ahead an create a PR (should be tagged with `code-improvement`).
  Just make sure it'll not break anything. Just keep in mind that here we follow the [`KISS principle`](https://en.wikipedia.org/wiki/KISS_principle),
  so we'll probably merge your PR, unless it adds much more code complexity/boilerplate (even though it gains more performance).
