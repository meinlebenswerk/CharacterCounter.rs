# Character Counter
A small yew based app to count characters and words.

I have to write a lot of essays this semester and they all come with strict character limits.\
While GoogleDocs has my back when I'm writing my own - we're required to cross-check each others work, and so instead of copying the text into a GoogleDoc, I normally just paste it into a character counter.

But there's one major flaw!
None of the come in dark mode :/

Ah who am I kidding... I just wanted to try and write a web-app in Rust and try out [Yew](https://yew.rs/).

## Installation / Usage

### Prerequisites - Running Locally
- You have to have [rust](https://www.rust-lang.org/tools/install) installed
- That comes with rustup, but you'll still need to install the WebAssembly target with `rustup target add wasm32-unknown-unknown`
- Then you're ready to install trunk: `cargo install trunk`
- And voila, you can start the app locally by running `trunk serve`

### I just want to count characters, without my eyes bleeding!
Yeah I get you - the app should be deployed on github, soonish - I'll put the link here.