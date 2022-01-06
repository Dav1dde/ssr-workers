SSR WorkeRS
===========

Small demo of creating a Cloudflare Worker to render a
[Sycamore](https://github.com/sycamore-rs/sycamore) application and hydrate it client side.

The demo needs:

* [Trunk](https://trunkrs.dev/)
* [Wrangler](https://github.com/cloudflare/wrangler)
* npx / miniflare (optional)

## Usage

Running locally:

```sh
$ npx miniflare@next
```

Building just the app:

```sh
$ cd app
$ trunk build # or watch
```

Watching for changes of the worker:

```sh
$ cd worker
$ cargo watch -- worker-build --dev
# Second Terminal
$ npx miniflare@next --watch
```

## Notes:

* There is a very weird bug that corrupts the returned HTML, if I don't clone the string.
* The Cloudflare builtin router is not very good for this usecase, due to the lack of support for
  multiple and overlapping wildcard matches, e.g. a route for static files `/*.*`.
* There is no good way to modify `<meta>` tags and data, this might need an additional templating
  layer.
* A very minimal setup already needs `500kb` of only `1000kb` available, this might not leave
  enough space for an actual application.
* ~~I dont currently have a way to watch the files and auto-reload the worker, this is annoying!~~
* The app entrypoint needs `sycamore::hydrate_to`, this does not work in standalone mode without
  SSR, this is annoying!
* A lot of APIs are missing, incomplete or not usable (e.g. needs git version to access KV value as
  bytes, no cache API).
* workers-rs feels like a PoC not like something people actually used.
