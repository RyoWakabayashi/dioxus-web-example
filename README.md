# dioxus-web-example

Dioxus Web Example

## Requirements

[asdf]

## Setup

Install asdf plugins

```bash
asdf plugin-add nodejs \
  ; asdf plugin-add python \
  ; asdf plugin-add rust
```

Install languages

```bash
asdf install
```

Install dependencies

```bash
npm install && \
  pip install --requirement requirements.txt && \
  rustup target add wasm32-unknown-unknown && \
  cargo install trunk && \
  cargo install cargo-edit
```

Setup pre-commit

```bash
pre-commit install
```

## Start server

```bash
npm run dev
```

## Build

```bash
npm run build
```

[asdf]: https://github.com/asdf-vm/asdf
