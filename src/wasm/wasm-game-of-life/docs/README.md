# Game of Life

## Hello, World

```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

```bash
🤷   Project Name: wasm-game-of-life
🔧   Destination: wasm-game-of-life ...
🔧   project-name: wasm-game-of-life ...
🔧   Generating template ...
💡   Initializing a fresh Git repository
✨   Done! New project created wasm-game-of-life
```

```bash
wasm-game-of-life/
├── Cargo.toml
├── LICENSE_APACHE
├── LICENSE_MIT
├── README.md
├── docs/
│   └── README.md
├── src/
│   ├── lib.rs
│   └── utils.rs
└── tests/
    └── web.rs
```

### Build the Project

```bash
wasm-pack build
```

```bash
[INFO]: Checking for the Wasm target...
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
[INFO]: Compiling to Wasm...
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 12.88s
[INFO]: :-) Your wasm pkg is ready to publish at wasm-game-of-life/pkg.
```

```bash
pkg/
├── README.md
├── package.json
├── wasm_game_of_life.d.ts
├── wasm_game_of_life.js
├── wasm_game_of_life_bg.js
├── wasm_game_of_life_bg.wasm
└── wasm_game_of_life_bg.wasm.d.ts
```

### Putting it into a Web Page

```bash
npm init wasm-app www
cd www
npm install
```

### Using our Local wasm-game-of-life Package in www

`vi www/package.json`

```json
{
  "dependencies": { // Add this three lines block!
    "wasm-game-of-life": "file:../pkg"
  },
}
```

```bash
npm install
```

### Serving Locally

```bash
npm run start
```
