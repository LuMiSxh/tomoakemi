# ともあけみ (Tomo-Akemi)

The new chip-8 emulator is here!

## About

This emulator uses Rust (Project `Tomo`) as the emulator, which gets compiled to WASM.
The frontend is build upon [SvelteKit](https://kit.svelte.dev/) and [Vite](https://vitejs.dev/) (Project `Akemi`)
and uses the WASM binary provided by [wasm-pack](https://rustwasm.github.io/wasm-pack/) to use the emulator in the web.

## Deployment

`Tomo` gets compiled via GitHub Workflows and is then added to `Akemi` as a build artifact. These two together
then get compiled/build by the [Vercel](https://vercel.com) CLI and uploaded to the [Vercel](https://vercel.com) hosting
service.

## Self Hosting

For `Tomo` you need to install the [Rust-Toolchain](https://www.rust-lang.org/tools/install)
and [wasm-pack](https://rustwasm.github.io/wasm-pack/) in order
to compile it with the command: `wasm-pack build --release`

For `Akemi` you need a node package manager of your choice, preferably [pnpm](https://pnpm.io/), to build the website.
The project commands can be found in the `package.json` file or in `Akemi`'s readme.

**WARNING: `Tomo` (`wasm`) is not compatible with vite's development server and therefore the preview needs to be used
instead of the dev view**

The simplest method would be to use the provided docker implementation. You need to run its compose
command: `docker compose up`

## Link

I know some of you want to visit the website, so here is the **[link](https://akemi-lumisxh.vercel.app/)**
