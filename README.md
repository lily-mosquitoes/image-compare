# Image Compare (frontend)

This is a citizen science project which
aims to understand what is considered by
people as a "good" image when it comes to
images that have been artificially recovered
from some type of image degradation.

Currently we are testing images that have been
[denoised](https://en.wikipedia.org/wiki/Noise_reduction#In_images)
using a
[modified first-order primal-dual algorithm](https://github.com/lily-mosquitoes/image-recovery).

## Technical info

This frontend is made with the framework
[Yew](https://yew.rs)
and uses
[Trunk](https://trunkrs.dev/)
to bundle the Rust code to
WebAssembly.[TailwindCSS](https://tailwindcss.com/docs)
was used for styling.

### Tests:

Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/).

`wasm-pack test --firefox --headless`

`wasm-pack test --chrome --headless`

### How to run:

Requires [node](https://nodejs.dev/en/learn/how-to-install-nodejs/),
[rust](https://www.rust-lang.org/tools/install) and
[trunk](https://trunkrs.dev/).

Install the wasm target if you don't have it already:

`rustup target add wasm32-unknown-unknown`

Then:

`npm install`

`npm run tailwind-watch`

`trunk serve`

You should now be able to see something like:

`INFO 📡 server listening at http://127.0.0.1:8080`

### Building for production:

`npm run tailwind-minify`

`trunk build --release`

Artifacts will be in the `dist` folder.
