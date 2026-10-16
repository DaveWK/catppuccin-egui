<h3 align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
	Catppuccin for <a href="https://github.com/emilk/egui">egui</a>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
	<a href="https://github.com/catppuccin/egui/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/egui?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/egui/issues"><img src="https://img.shields.io/github/issues/catppuccin/egui?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/egui/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/egui?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/egui/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/catppuccin/egui/ci.yml?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://crates.io/crates/catppuccin-egui"><img src="https://img.shields.io/crates/v/catppuccin-egui?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
	<a href="https://docs.rs/catppuccin-egui"><img src="https://img.shields.io/docsrs/catppuccin-egui?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/preview.webp"/>
</p>

## Status

This public compatibility fork is version **5.8.0**, based on upstream 5.7.0.
It adds `egui36` and `egui37` (latest stable egui 0.37), preserving the original
`egui26` default and previous features. Select exactly one version feature.
The fork is source-only: 5.8.0 is not published to crates.io. GitHub Actions
remain disabled; checks run locally on Fedora with `make check`.
Native palette tests, doctests, strict Clippy and WASM checks pass on egui
0.36.2 and 0.37.0; the default 0.26.2 smoke also passes. Dependency policy
checks pass for the latest selected feature.
See [code standards](doc/CODE-STANDARDS.md) and [queue](doc/QUEUE.md).

## Previews

<details>
<summary>🌻 Latte</summary>
<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/latte.png"/>
</details>
<details>
<summary>🪴 Frappé</summary>
<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/frappe.png"/>
</details>
<details>
<summary>🌺 Macchiato</summary>
<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/macchiato.png"/>
</details>
<details>
<summary>🌿 Mocha</summary>
<img src="https://raw.githubusercontent.com/catppuccin/egui/main/assets/previews/mocha.png"/>
</details>

## Usage

catppuccin-egui uses Cargo features to add support for new egui versions without breaking backwards compatibility.

Add the crate to your `Cargo.toml`:

<!-- x-release-please-start-version -->

```toml
[dependencies]
catppuccin-egui = { git = "https://github.com/DaveWK/catppuccin-egui", branch = "main", default-features = false, features = ["egui37"] }
```

<!-- x-release-please-end -->

To use a theme, call the `set_theme` function with a theme and the egui context:

```rust
catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);
```

The inherited todo example targets egui 0.32. The compatibility checks target
the library directly; no renderer or bundled font is required.

See the full documentation at https://docs.rs/catppuccin-egui.

[Whiskers](https://github.com/catppuccin/toolbox/tree/main/whiskers) is optional
when building a Git checkout. Without it, the checked-in palette is preserved.
If generation is attempted and fails, the build fails without truncating the palette.

## 💝 Thanks to

- [Stonks3141](https://github.com/Stonks3141)

&nbsp;

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" />
</p>

<p align="center">
	Copyright &copy; 2023-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/egui/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>
</p>
