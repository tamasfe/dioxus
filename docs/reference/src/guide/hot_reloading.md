# Hot Reloading
1. Hot reloading allows much faster iteration times inside of rsx calls by interperting them and streaming the edits.
2. It is useful when changing the styling/layout of a program, but will not help with changing the logic of a program.
3. Currently the cli only implements hot reloading for the web renderer.

# Setup
Install [dioxus-cli](https://github.com/DioxusLabs/cli).
Enable the hot_reload feature on dioxus:
```toml
dioxus = { version = "*", features = ["web", "hot_reload"] }
```

# Usage
1. run:
```
dioxus serve --hot-reload
```
2. change some code within a rsx macro
3. open your localhost in a browser
4. save and watch the style change without recompiling

# Limitations
1. The interperter can only use expressions that existed on the last full recompile. If you introduce a new variable or expression to the rsx call, it will trigger a full recompile to capture the expression.
2. Components and Iterators can contain abritary rust code, and will trigger a full recompile when changed.