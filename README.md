# Sudoku game and solver
## Description
A small gui sudoku game and solver written in Rust using Dioxus and Tailwind CSS.

## Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload=true --platform desktop
```

Needed if you want to deploy to the web: \
**Remember to uncomment the line `base_path = "/sudoku"` in `Dioxus.toml` file** \
(Random crate needs wasm backend to be enabled explicitly to use it)
```bash
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' dx bundle --platform web --out-dir sudoku-web-app
```