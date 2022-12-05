# ともあけみ (Tomo-Akemi)

The new chip-8 emulator is here!

## About

This emulator uses Rust (Project `Tomo`) as the emulator, which gets compiled to WASM.
The frontent is build upon SvelteKit and Vite (Project `Akemi`) and uses the WASM binary provided by `wasm-pack`
to use the emulator in the web.

## Deployment

`Tomo` gets compiled via GitHub Workflows and is then added to `Akemi` as a build artifact. These two together
then get compiled/build by the Vercel CLI and uploaded to the Vercel hosting service.

## Self Hosting

For `Tomo` you need to install the [Rust-Toolchain](https://www.rust-lang.org/tools/install) and [wasm-pack](https://rustwasm.github.io/wasm-pack/) in order 
to compile it with the command: `wasm-pack build --release`

For `Akemi` you only need a node package manager of your chice, preferably [pnpm](https://pnpm.io/), to build the website.
Its commands can be found in the `package.json` file (Most importantly `pnpm build`).
**WARNING: `Tomo` (generally `wasm` itself) are not compatible with vite's development server and therefore the preview needs to be used**

If this is too much work, or you want a simpler method, you can use the provided docker implementation. You only need to run its
compose command: `docker compose up`

## Link

I know some of you want to visit the website, so here is the **[link](https://akemi-lumisxh.vercel.app/)**
