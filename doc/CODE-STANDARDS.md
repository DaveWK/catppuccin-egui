# Code standards

Copied from xitely/sbc-ci-tools doc/CODE-STANDARDS.md at 0cabcefbae77dd79a0df2bfe21c9880e3c5d13e3.

Every repo of ours carries a verbatim copy at `doc/CODE-STANDARDS.md`. In a copy the line after the title reads
`Copied from xitely/sbc-ci-tools doc/CODE-STANDARDS.md at <commit>.`, everything from there to `## This repository`
is the canonical text of that commit (sbc-ci-tools `standards-check` verifies it), and `## This repository` holds the
repo's own choices and any written exceptions. Upstream trees and fork branches bound for upstream follow upstream's
style instead; their Forgejo-only CI commit carries the copy.

Source: Dave's rules as recorded in the orchestrator `doc/DECISIONS.md`; a change here needs Dave's go.

## Languages

- Order of preference for tools and automation: **Rust > Ansible > Python > Perl > Bash**.
  - Durable tools and services: Rust.
  - Host and runner configuration (packages, services, users, files): Ansible.
  - One-off automation and multi-step file work: Python, not Bash.
  - Bash only for thin launchers and single commands.
- Firmware: Rust (oreboot, shared crates) with thin C where EDK2 needs it.

## Rust

- **Toolchain:** pin an exact stable in `rust-toolchain.toml` (today `1.99.0`, installed by rustup). Bumps are
  deliberate commits. Never depend on rustup's default or PATH order: a nested cargo call (`cargo run` -> child
  cargo) must name its toolchain or clear the inherited `RUSTUP_TOOLCHAIN`, because rustup passes it to children.
- **Nightly** only with a written justification kept next to the pin: every unstable feature used, a build attempt on
  the pinned stable and why it fails. A nightly without a current justification is a defect. (Today: the K1 bt0 stays
  on `nightly-2026-05-06` by Dave's decision, revisited after A7A.)
- **Async:** tokio. **Data:** serde. **Services:** actix-web.
- **Logs and telemetry:** OpenTelemetry logs (OTLP export) through `tracing`, with a stderr fallback; the shared
  setup is `ci-common`'s telemetry module.
- **Linking and targets:** prefer mold + clang. `-C target-cpu=native` only for tools built and run on the same host;
  binaries copied between our hosts target **x86-64-v4** (digdug, outlander and the workstation all support it;
  revisit if a runner without AVX-512 joins).
- **Checks, all clean with no deprecation warnings:** `rustfmt`, `clippy -D warnings`, `cargo-deny` (licences,
  advisories, bans).
- **Tests:** put external services (Forgejo, serial consoles, hardware) behind a trait and test with fakes; use
  `tokio::time::pause` for time; no live services in CI tests.

## Ansible

- Real modules (`ansible.builtin.*`, collections), never `shell`/`command` wrappers for what a module does.
  `command` only where no module exists (for example rustup), guarded so it reports changes truthfully; read-only
  probes use `changed_when: false` and must not change state.
- `ansible-lint` and `yamllint` clean. Before any run against a real host: `--check --diff`, read the diff back,
  then apply.

## Python

- Formatted with **black**; no pip installs in CI jobs (packages come from the runner package list).
- Forgejo access through the client libraries (`forgejo-api` in Rust, `pyforgejo` in Python), not raw HTTP.

## Shell

- **shellcheck** clean and **shfmt** formatted.

## YAML

- **yamllint** clean.

## Makefiles

- Every Makefile of ours has `help`, `clean` and `mrproper` targets.

## Dependencies and layout

- No git submodules. Dependencies come through a pin file (`patches/<group>/pin.env`: URL, base commit, expected tree)
  fetched by **pin-fetch**, with board changes as `git format-patch` patchsets applied by `git am`.
- Fetching is done by the Rust pin-fetch, never by ad hoc scripts.
- No live dependencies on `~/src/archived`.

## Repository docs

- README status section and `doc/QUEUE.md` carry the repo's current state and queue, groomed in the same commit
  series as the change.
- `doc/CODE-STANDARDS.md` as described at the top.

## Commits and publishing

- Identity: `DaveWK <DaveWK225@protonmail.com>` on private repos; `DaveWK <13626476+DaveWK@users.noreply.github.com>`
  for anything published on GitHub; "Dave Klotz" only for upstream submissions that need a real name (DCO).
- **No AI attribution, generated-by footers or co-author trailers** in commits, PR bodies or release notes.
- Private Forgejo first; nothing goes to GitHub or upstream without Dave's go.
- Never print or commit secrets (they live in `~/.myStuff`). Values from confidential vendor documents may be used,
  never cited; no JEDEC document numbers.
- Main only in our repos; trial branches are merged then archive-tagged before deletion.

## Licences

- Each repo states its licence in `## This repository`; Dave decides it per repo (so far rusty_net, sbc-ci-tools,
  pin-fetch and the RustSBI work: MIT OR Apache-2.0). Firmware trees and forks follow their upstream licence.
- SPDX line in every source file.

## This repository

- MIT, following the upstream Catppuccin library licence and authorship.
- Compatibility fork 5.8.0: additive egui36 and egui37 features; original egui26
  default and existing features remain unchanged. Select exactly one egui feature.
- Rust 1.99.0 stable; native and wasm32-unknown-unknown library checks.
- This is a synchronous theme library, not a service or orchestration tool;
  no Tokio runtime, telemetry exporter, network server or background worker is added.
- Preserve upstream library style. Tests apply actual palette helpers without
  loading default font bundles. New version features disable egui default features.
- Inherited GitHub Actions are disabled; their Ubuntu runners are not used.
  Local Fedora checks use make check. No crates.io publication is implied.
- The checked-in palette is authoritative when the optional Whiskers generator
  is unavailable; generation failure must not truncate or replace that source.
