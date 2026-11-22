# Dodai Tech website

[**dodaitech.dev**](https://dodaitech.dev): the home of Dodai Tech, the Dodai UI design system with its live
showcase, and the SynthWave watch face.

A Rust project. `src/` generates the pages with `dodai-topcoat`, `dodai-icons` and `dodai-core` (all from
crates.io), copies the hand-made files of `static/` next to them, and writes the result to `public/`. That
directory is what gets uploaded to Cloudflare (Workers static assets, project `dodaitech`). `public/` is
never committed.

```
src/        the generator: site.rs (pages, privacy policy), charts.rs (SVG charts), shaders.rs (showcase pictures), main.rs
static/     hand-made files, copied as they are: site.css, favicon, img/, the showcase wasm
public/     the built site (ignored)
tests/      builds the whole site and checks it like a visitor: pages, links, images, theme switch
fuzz/       cargo-fuzz targets for the chart maths
scripts/    Nushell: ci, coverage, deploy, remotes, showcase build (tests in scripts/tests/)
```

## Commands

| Task | Command |
|---|---|
| Build into `public/` | `just build` |
| Preview at http://127.0.0.1:8765 | `just serve` |
| Gate before every commit: fmt, clippy, tests, build, dry-run deploy | `just check` |
| Coverage with a floor of 95 % of lines | `just coverage` |
| Advisories, licences, bans, sources | `just audit` |
| Fuzz a chart target for a minute | `just fuzz chart_points` |
| Upload to Cloudflare | `just deploy` |
| Push to GitHub and Gitea | `just push-all` |
| Rebuild the browser showcase (needs a `dodai-ui` checkout) | `just showcase ~/Projects/dodai-ui` |
| Render the showcase shader pictures (GPU and ffmpeg) | `just shaders` |

`just` lists all recipes, including the git ones (`push`, `push-gitea-starscream`, `push-all-force`,
`pull-all`, `remotes`).

## Change the site

1. Edit `src/` for pages, or `static/` for stylesheet and pictures. The privacy policy text is `PRIVACY` in `src/site.rs`.
2. `just serve`, look at it in the browser.
3. `just check`. It must be green.
4. Commit, then `just push-all`.

Pushing to `main` deploys by itself (see CI below). Nothing else is needed.

## Deploy by hand

Only needed when CI is unavailable or you want to see the result before pushing. Install Rust, Node,
[Nushell](https://www.nushell.sh) and [just](https://just.systems). Wrangler is fetched by `npx`; you never
install it or log in to it.

```sh
export CLOUDFLARE_API_TOKEN=...    # Cloudflare token with Workers Scripts: Edit
export CLOUDFLARE_ACCOUNT_ID=...   # 32 hex characters (wrangler whoami prints it)
just check                         # same gate as CI
just deploy                        # builds public/ and uploads it
just push-all                      # then publish the commit; CI deploys the same content again
```

`nu scripts/deploy.nu --dry-run` lists what would be uploaded without needing the variables.
Whitespace and quotes around the two values are trimmed.

## Another machine

```sh
git clone git@github.com:Dodai-Tech/dodaitech-website.git     # or gitea@192.168.1.44:Dodai-Tech/dodaitech-website.git
cd dodaitech-website
just setup-remotes    # adds whichever of origin and gitea-starscream is missing
just check
```

## Tests and quality

* `cargo test` builds the whole site into a temp directory and checks it: every page exists and has a
  language, title and description; every local link and image resolves; the home page names SynthWave and
  Dodai UI; the theme switch is on the right pages; the store links are there.
* `just coverage` fails below 95 % of lines (now 100 %). `main.rs` and `shaders.rs` are left out: the first
  only parses arguments, the second needs a GPU and ffmpeg.
* Scripts have their own tests: `nu scripts/tests/run.nu`.
* clippy runs with `-D warnings`; `missing_docs` is on.
* `just audit` and the nightly security workflow run `cargo audit` and `cargo deny`; the fuzz workflow
  hammers the chart maths.

## CI (GitHub Actions and Gitea Actions)

| Workflow | When | What |
|---|---|---|
| `ci.yml` | pull request, push to `main` | `nu scripts/ci.nu` and the coverage floor |
| `deploy.yml` | push to `main`, by hand | build and `wrangler deploy` |
| `security.yml` | nightly, dependency changes | `cargo audit`, `cargo deny check` |
| `fuzz.yml` | nightly, source changes | 60 s per target in `fuzz/` |

Both hosts need the secrets `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID`. Both deploy on a push; the site
is identical, and the last upload wins. Deploy does not wait for CI, so run `just check` before pushing.
