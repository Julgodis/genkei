<p align="center">
  <a href="https://github.com/Julgodis/genkei/">
    <h1 align="center">
      Genkei
    </h1>
  </a>
</p>

<div align="center">

[![license](https://img.shields.io/crates/l/picori)](https://github.com/Julgodis/genkei/LICENSE)

```diff
!!! This project is primarily for personal use, and I may not actively accept !!!
!!! pull requests. Feel free to explore the code, but please understand that  !!!
!!! contributions might not be incorporated.                                  !!!
```

</div>

Genkei is a specialized Rust library engineered to streamline the process of generating server-side HTML and CSS content. With an intuitive, fluent interface complemented by a Tailwind-based styling system, Genkei offers a robust yet idiomatic approach for rendering HTML elements.

## Getting Started

To install Genkei, add the following line to your Cargo.toml:
```toml
[dependencies]
genkei = { git = "https://github.com/Julgodis/genkei/" }
```

## Usage

```rust
let tag = genkei::div()
  .id("id")
  .bg_color(Color::Slate500)
  .p(2)
  .text_content("Hello, world!");

let html = tag.to_html()?;
```

##  License
This project is licensed under the MIT License. See the LICENSE file for details.

