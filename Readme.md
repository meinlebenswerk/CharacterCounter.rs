# Character Counter
[![Deploy to GithubPages](https://github.com/meinlebenswerk/CharacterCounter.rs/actions/workflows/pages.yml/badge.svg)](https://github.com/meinlebenswerk/CharacterCounter.rs/actions/workflows/pages.yml)

A small yew based app to count characters and words.

I have to write a lot of essays this semester and they all come with strict character limits.\
While GoogleDocs has my back when I'm writing my own - we're required to cross-check each others work, and so instead of copying the text into a GoogleDoc, I normally just paste it into a character counter.

But there's one major flaw!
None of the come in dark mode :/

Ah who am I kidding... I just wanted to try and write a web-app in Rust and try out [Yew](https://yew.rs/).


## No Code! I just want to count characters, without my eyes bleeding!
Yeah okay... the app is deployed [here](https://meinlebenswerk.github.io/CharacterCounter.rs/)

## Installation / Usage

### Prerequisites - Running Locally
- You have to have [rust](https://www.rust-lang.org/tools/install) installed
- That comes with rustup, but you'll still need to install the WebAssembly target with `rustup target add wasm32-unknown-unknown`
- Then you're ready to install trunk: `cargo install trunk`
- And voila, you can start the app locally by running `trunk serve`

Alternatively you can do a release build using `trunk build --release` and then serve the dist-folder with something like `npx serve dist`.