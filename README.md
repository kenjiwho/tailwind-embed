# tailwindcss-embed

`tailwindcss-embed` is a Rust procedural macro crate that runs the Tailwind CSS CLI at compile time, captures the generated CSS, and embeds it directly into your application as a static string.

It allows Rust projects to use Tailwind CSS without manually generating or maintaining a CSS output file.

Originally designed to make integrating Tailwind CSS with [ZGUI](https://github.com/zortax/zgui) easier, this crate is not tied to ZGUI and can be used with any Rust application or library that accepts CSS stylesheets.

## Features

- **Compile-time CSS generation:** Runs Tailwind CSS during Rust compilation.
- **Embedded CSS:** Generated CSS becomes a string literal in your compiled application.
- **Automatic source detection:** Tailwind scans your project for classes used in your source files.

## Requirements

You must have the following installed:

- [Tailwind CSS CLI](https://github.com/tailwindlabs/tailwindcss/releases).

The Tailwind CLI must be accessible through your system's `PATH` as `tailwindcss`.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
tailwind-embed = "0.1"
```

## Usage

Using Tailwind CSS is as simple as invoking the `tailwind_css!()` macro:

```rust
use tailwind_embed::tailwind_css;

const CSS: &'static str = tailwind_css!();
```

The macro runs the Tailwind CLI, captures the generated CSS, and expands into a Rust string literal.

The resulting value has a `'static` lifetime.

### Example: ZGUI

One of the original motivations for this crate was integrating Tailwind CSS with [ZGUI](https://github.com/zortax/zgui).

```rust
fn main() {
    let app = /* create your ZGUI application */
        .with_stylesheet(tailwind_css!());

    // Run application...
}
```

You can now use Tailwind utility classes in your ZGUI components:

```rust
view! {
    column(class = "flex items-center justify-center gap-4 p-4") {
        text(class = "text-xl font-bold text-blue-500") {
            "Hello, Tailwind!"
        }
    }
}
```

The generated stylesheet is embedded in the application. No CSS file needs to be loaded at runtime.

### Multi-Module Projects

The `tailwind_css!()` macro runs the Tailwind CSS CLI when it is expanded. To avoid running the CLI multiple times in a project, invoke the macro in a single module and expose the generated CSS through a public constant.

For example, create a `style.rs` file:

```rust
// src/style.rs

pub const CSS: &'static str = tailwind_css!();
```

Then access the generated stylesheet from anywhere in your project:

```rust
// src/main.rs

mod style;

use crate::style::CSS;

fn main() {
    // Use CSS to pass the stylesheet to your UI framework.
}
```

## How it works

When Rust expands the procedural macro during compilation:

1. The macro locates the consuming Cargo project's directory.
2. It supplies Tailwind CSS input to the Tailwind CLI through stdin.
3. Tailwind scans the project's source files for utility classes.
4. The CLI generates the CSS.
5. The macro captures the generated CSS through stdout.
6. The generated CSS is converted into a Rust string literal and embedded in the compiled application.

The default CSS input is:

```css
@import "tailwindcss";
@source ".";
```

The generated CSS contains the utilities detected in your project.

## Important considerations

### Source detection

Tailwind scans source files as plain text. It detects complete class names, but cannot reliably detect dynamically constructed class names.

For example, this works:

```rust
let class = "bg-blue-500";
```

But this may not be detected:

```rust
let class = format!("bg-{}", color);
```

For dynamically generated classes, use complete class names or explicitly include them in your Tailwind source configuration.

### Compilation requirements

The Tailwind CLI must be available when compiling the application.

It is not required when running the compiled application.

### CSS compatibility

Tailwind CSS v4 generates modern CSS. Your application's CSS engine must support the CSS features used by the generated stylesheet.

For example, when using ZGUI, compatibility depends on the CSS features supported by its Stylo-based styling system.

## License
- [MIT](LICENSE)
