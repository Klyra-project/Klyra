<!-- markdownlint-disable -->
<p align="center">
<img width="300" src="https://raw.githubusercontent.com/klyra-hq/klyra/master/assets/logo-rectangle-transparent.png"/>
</p>
<br>
<p align="center">
  <a href="https://github.com/klyra-hq/klyra/search?l=rust">
    <img alt="language" src="https://img.shields.io/badge/language-Rust-orange.svg">
  </a>
  <a href="https://docs.klyra.rs/">
    <img alt="docs" src="https://img.shields.io/badge/docs-klyra.rs-orange">
  </a>
  <a href="https://docs.rs/klyra-runtime">
    <img alt="crate-docs" src="https://img.shields.io/badge/docs-docs.rs-orange">
  </a>
  <a href="https://status.klyra.rs/">
    <img alt="status" src="https://img.shields.io/badge/status-blue">
  </a>
  <a href="https://circleci.com/gh/klyra-hq/klyra/">
    <img alt="build status" src="https://circleci.com/gh/klyra-hq/klyra.svg?style=shield"/>
  </a>
</p>
<p align="center">
  <a href="https://crates.io/crates/cargo-klyra">
    <img alt="crates" src="https://img.shields.io/crates/d/cargo-klyra">
  </a>
  <a href="https://discord.gg/klyra">
    <img alt="discord" src="https://img.shields.io/discord/803236282088161321?logo=discord"/>
  </a>
  <a href="https://twitter.com/klyra_dev">
    <img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/klyra_dev">
  </a>
</p>
<p align="center">
  <a href="https://console.algora.io/org/klyra/bounties?status=open">
    <img alt="open bounties" src="https://img.shields.io/endpoint?url=https%3A%2F%2Fconsole.algora.io%2Fapi%2Fshields%2Fklyra%2Fbounties%3Fstatus%3Dopen"/>
  </a>
  <a href="https://console.algora.io/org/klyra/bounties?status=completed">
    <img alt="rewarded bounties" src="https://img.shields.io/endpoint?url=https%3A%2F%2Fconsole.algora.io%2Fapi%2Fshields%2Fklyra%2Fbounties%3Fstatus%3Dcompleted"/>
  </a>
</p>
<!-- markdownlint-restore -->

---

# Klyra

[Klyra](https://www.klyra.rs/) is a Rust-native cloud development platform that lets you deploy your Rust apps for free.

Klyra is built for productivity, reliability and performance:

- Zero-Configuration support for Rust using annotations
- Automatic resource provisioning (databases, caches, subdomains, etc.) via [Infrastructure-From-Code](https://www.klyra.rs/blog/2022/05/09/ifc)
- First-class support for popular Rust frameworks ([Actix Web](https://docs.klyra.rs/examples/actix), [Rocket](https://docs.klyra.rs/examples/rocket), [Axum](https://docs.klyra.rs/examples/axum),
  [Tide](https://docs.klyra.rs/examples/tide), [Poem](https://docs.klyra.rs/examples/poem) and [Tower](https://docs.klyra.rs/examples/tower))
- Support for deploying Discord bots using [Serenity](https://docs.klyra.rs/examples/serenity)
- Scalable hosting (with optional self-hosting)

📖 Check out our documentation to get started quickly: [docs.klyra.rs](https://docs.klyra.rs)

🙋‍♂️ If you have any questions, join our [Discord](https://discord.gg/klyra) server.

⭐ If you find Klyra interesting, and would like to stay up-to-date, consider starring this repo to help spread the word.

![star](https://i.imgur.com/kLWmThm.gif)

## (NEW) Klyra Console

Your projects can now be viewed on the brand new [Klyra Console](https://console.klyra.rs/)!
The CLI is still used for most tasks.

![console-preview](https://i.imgur.com/1qdWipP.gif)
*The GIF above visualizes the ease of adding resources to your project(s), along with how they are displayed in the console.*

## Getting Started

The `cargo-klyra` CLI can be installed with a pre-built binary or from source with cargo.

Klyra provides pre-built binaries of the `cargo-klyra` CLI with every release
for most platforms, they can be found on [our GitHub](https://github.com/klyra-hq/klyra/releases/latest).

Our binaries can also be installed using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall),
which will automatically install the correct target for your system.
To install with `cargo-binstall`, run:

```sh
# cargo-binstall can also be installed directly as a binary to skip the compilation time: https://github.com/cargo-bins/cargo-binstall#installation
cargo install cargo-binstall
cargo binstall cargo-klyra
# If installing binstall or cargo-klyra fails, try adding `--locked` to the install command
```

Although a bit slower, you can also install directly with cargo:

```sh
cargo install cargo-klyra
```

After installing, log in with:

```sh
cargo klyra login
```

To initialize your project, simply write:

```bash
cargo klyra init --template axum hello-world
# Choose a unique project name!
```

And to deploy it, write:

```bash
cd hello-world
cargo klyra project start  # Only needed if project has not already been created during init
cargo klyra deploy --allow-dirty
```

And... that's it!

```text
Service Name:  hello-world
Deployment ID: 3d08ac34-ad63-41c1-836b-99afdc90af9f
Status:        running
Last Updated:  2022-04-01T08:32:34Z
URI:           https://hello-world.klyraapp.rs
```

Feel free to build on top of the generated `hello-world` boilerplate or take a stab at one of our [examples](https://github.com/klyra-hq/klyra-examples).

For the full documentation, visit [our docs](https://docs.klyra.rs).

## Repositories

| Name | Description |  |  |
|-|-|-|-|
| [klyra](https://github.com/klyra-hq/klyra) 🚀 (This repo) | The core Klyra product. Contains all crates that users interact with. | [Issues](https://github.com/klyra-hq/klyra/issues) | [PRs](https://github.com/klyra-hq/klyra/pulls)
| [klyra-examples](https://github.com/klyra-hq/klyra-examples) 👨‍🏫 | Officially maintained examples of projects that can be deployed on Klyra. Also has a list of [community examples](https://github.com/klyra-hq/klyra-examples#community-examples). | [Issues](https://github.com/klyra-hq/klyra-examples/issues) | [PRs](https://github.com/klyra-hq/klyra-examples/pulls)
| [klyra-docs](https://github.com/klyra-hq/klyra-docs) 📃 | Documentation hosted on [docs.klyra.rs](https://docs.klyra.rs/). | [Issues](https://github.com/klyra-hq/klyra-docs/issues) | [PRs](https://github.com/klyra-hq/klyra-docs/pulls)
| [www](https://github.com/klyra-hq/www) 🌍 | Our website [klyra.rs](https://www.klyra.rs/), including the [blog](https://www.klyra.rs/blog/tags/all) and [Launchpad newsletter](https://www.klyra.rs/launchpad). | [Issues](https://github.com/klyra-hq/www/issues) | [PRs](https://github.com/klyra-hq/www/pulls)
| [deploy-action](https://github.com/klyra-hq/deploy-action) ⚙ | GitHub Action for continuous deployments. | [Issues](https://github.com/klyra-hq/deploy-action/issues) | [PRs](https://github.com/klyra-hq/deploy-action/pulls)
| [awesome-klyra](https://github.com/klyra-hq/awesome-klyra) 🌟 | An awesome list of Klyra-hosted projects and resources that users can add to. | [Issues](https://github.com/klyra-hq/awesome-klyra/issues) | [PRs](https://github.com/klyra-hq/awesome-klyra/pulls)

## Contributing to Klyra

Contributing to Klyra is highly encouraged!

Check out our [contributing docs](./CONTRIBUTING.md) and find the appropriate repo above to contribute to.

For development of this repo, check the [development docs](./DEVELOPING.md).

Even if you are not planning to submit any code, joining our [Discord server](https://discord.gg/klyra) and providing feedback helps us a lot!

### Algora Bounties 💰

To offload work from the engineering team on low-priority issues, we will sometimes add a cash bounty to issues.
Sign up to the [Algora Console](https://console.algora.io/org/klyra/bounties?status=open) to find open issues with bounties.

## Community and Support

- [GitHub Issues](https://github.com/klyra-hq/klyra/issues). Best for: bugs and errors you encounter using Klyra.
- [Twitter](https://twitter.com/klyra_dev). Best for: keeping up with announcements, releases, collaborations and other events.
- [Discord](https://discord.gg/klyra). Best for: *ALL OF THE ABOVE* + help, support, sharing your applications and hanging out with the community.

## Project Status

Check for any outages and incidents on [Klyra Status](https://status.klyra.rs/).

We are currently in Public Beta.
Watch "releases" of this repo to get notified of major updates!
Also, check out the [Beta announcement](https://www.klyra.rs/beta#06) for features we are looking forward to.

- [x] Alpha: We are testing Klyra, API and deployments may be unstable
- [x] Public Alpha: Anyone can sign up, but go easy on us,
  there are a few kinks
- [x] Public Beta: Stable enough for most non-enterprise use-cases
- [ ] Public: Production-ready!

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://kaleidawave.github.io/"><img src="https://avatars.githubusercontent.com/u/26967284?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Ben</b></sub></a><br /><a href="#blog-kaleidawave" title="Blogposts">📝</a> <a href="https://github.com/klyra-hq/klyra/issues?q=author%3Akaleidawave" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=kaleidawave" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=kaleidawave" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/SonicZentropy"><img src="https://avatars.githubusercontent.com/u/12196028?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Casey Bailey</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/issues?q=author%3ASonicZentropy" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=SonicZentropy" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/christoshadjiaslanis"><img src="https://avatars.githubusercontent.com/u/14791384?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Christos Hadjiaslanis</b></sub></a><br /><a href="#blog-christoshadjiaslanis" title="Blogposts">📝</a> <a href="https://github.com/klyra-hq/klyra/commits?author=christoshadjiaslanis" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/pulls?q=is%3Apr+reviewed-by%3Achristoshadjiaslanis" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/klyra-hq/klyra/commits?author=christoshadjiaslanis" title="Tests">⚠️</a> <a href="#video-christoshadjiaslanis" title="Videos">📹</a> <a href="https://github.com/klyra-hq/klyra/issues?q=author%3Achristoshadjiaslanis" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/brokad"><img src="https://avatars.githubusercontent.com/u/13315034?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Damien</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/issues?q=author%3Abrokad" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=brokad" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=brokad" title="Documentation">📖</a> <a href="#fundingFinding-brokad" title="Funding Finding">🔍</a> <a href="https://github.com/klyra-hq/klyra/pulls?q=is%3Apr+reviewed-by%3Abrokad" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/klyra-hq/klyra/commits?author=brokad" title="Tests">⚠️</a></td>
    <td align="center"><a href="http://alsuren.github.io/"><img src="https://avatars.githubusercontent.com/u/254647?v=4?s=100" width="100px;" alt=""/><br /><sub><b>David Laban</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/issues?q=author%3Aalsuren" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=alsuren" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=alsuren" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/ivancernja"><img src="https://avatars.githubusercontent.com/u/14149737?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Ivan</b></sub></a><br /><a href="#blog-ivancernja" title="Blogposts">📝</a> <a href="https://github.com/klyra-hq/klyra/issues?q=author%3Aivancernja" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=ivancernja" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=ivancernja" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/lilianmoraru"><img src="https://avatars.githubusercontent.com/u/621738?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Lilian Anatolie Moraru</b></sub></a><br /><a href="#content-lilianmoraru" title="Content">🖋</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/coszio"><img src="https://avatars.githubusercontent.com/u/62079184?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Luis Cossío</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/commits?author=coszio" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=coszio" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/marioidival"><img src="https://avatars.githubusercontent.com/u/1129263?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Mario Idival</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/commits?author=marioidival" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=marioidival" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/Butch78"><img src="https://avatars.githubusercontent.com/u/19205392?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Matthew Aylward </b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/commits?author=Butch78" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=Butch78" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/bmoxb"><img src="https://avatars.githubusercontent.com/u/42641081?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Max</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/issues?q=author%3Abmoxb" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=bmoxb" title="Code">💻</a> <a href="#example-bmoxb" title="Examples">💡</a> <a href="https://github.com/klyra-hq/klyra/pulls?q=is%3Apr+reviewed-by%3Abmoxb" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/klyra-hq/klyra/commits?author=bmoxb" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/nahuakang"><img src="https://avatars.githubusercontent.com/u/18533347?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Nahua</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/issues?q=author%3Anahuakang" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=nahuakang" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=nahuakang" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/nodard"><img src="https://avatars.githubusercontent.com/u/12720758?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Nodar Daneliya</b></sub></a><br /><a href="#business-nodard" title="Business development">💼</a> <a href="#fundingFinding-nodard" title="Funding Finding">🔍</a></td>
    <td align="center"><a href="https://github.com/oddgrd"><img src="https://avatars.githubusercontent.com/u/29732646?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Oddbjørn Grødem</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/commits?author=oddgrd" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=oddgrd" title="Tests">⚠️</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/chesedo"><img src="https://avatars.githubusercontent.com/u/5367103?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Pieter</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/issues?q=author%3Achesedo" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=chesedo" title="Code">💻</a> <a href="#example-chesedo" title="Examples">💡</a> <a href="#mentoring-chesedo" title="Mentoring">🧑‍🏫</a> <a href="https://github.com/klyra-hq/klyra/pulls?q=is%3Apr+reviewed-by%3Achesedo" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/klyra-hq/klyra/commits?author=chesedo" title="Tests">⚠️</a> <a href="https://github.com/klyra-hq/klyra/commits?author=chesedo" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/thecotne"><img src="https://avatars.githubusercontent.com/u/1606993?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Tsotne Nazarashvili</b></sub></a><br /><a href="#content-thecotne" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/Xavientois"><img src="https://avatars.githubusercontent.com/u/34867186?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Xavientois</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/issues?q=author%3AXavientois" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=Xavientois" title="Code">💻</a> <a href="https://github.com/klyra-hq/klyra/commits?author=Xavientois" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://thesnugco.com/"><img src="https://avatars.githubusercontent.com/u/19667780?v=4?s=100" width="100px;" alt=""/><br /><sub><b>jmwill86</b></sub></a><br /><a href="https://github.com/klyra-hq/klyra/issues?q=author%3Ajmwill86" title="Bug reports">🐛</a> <a href="https://github.com/klyra-hq/klyra/commits?author=jmwill86" title="Documentation">📖</a> <a href="https://github.com/klyra-hq/klyra/commits?author=jmwill86" title="Tests">⚠️</a> <a href="https://github.com/klyra-hq/klyra/commits?author=jmwill86" title="Code">💻</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
