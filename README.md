Diff and populate your `.env` file from `.env.dist` automatically.

Usage
-----

If you have a `.env.dist` file committed in GIT and after
a recent `git pull` it has been changed, so now you need to
add those new env-variables to your own `.env`.  
This should be as easy as running `envpopulate`.

Upon running it, you will be asked if you want to enter
custom values for every variable that doesn't yet exist in 
`.env`, if you don't want any interactivity then you need 
to include `--quiet` option.

Building
--------

This utility is written in Rust so that you could get 
a small binary executable that doesn't require any additional
dependencies like an interpreter or a VM and it could be easily
bundled in a docker container.

In order from simplest to hardest:
 - download already-built binaries from the latest [Release](https://github.com/nikita2206/envpopulate/releases/latest)
 - install using Cargo: `cargo install envpopulate`
 - checkout this repo using GIT and build with `cargo build`
 - using Docker run in directory containing your `.env.dist` file: 
 `docker run --rm -ti -v "$PWD":/usr/src/env -w /usr/src/env nikita2206/envpopulate:latest`
 