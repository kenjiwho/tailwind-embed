use std::{
    io::Write,
    process::{Command, Stdio},
};

use proc_macro::{Literal, TokenStream, TokenTree};
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn tailwind_css(input: TokenStream) -> TokenStream {
    let input_str = if input.is_empty() {
        String::new()
    } else {
        parse_macro_input!(input as LitStr).value()
    };

    match compile(input_str) {
        Ok(css) => {
            let mut result = TokenStream::new();
            result.extend([TokenTree::Literal(Literal::string(&css))]);
            result
        }
        Err(e) => panic!("tailwind-embed: {e}"),
    }
}

fn compile(input: String) -> Result<String, String> {
    let project_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "Could not determine project directory".to_string())?;

    // create tailwindcss compiler call
    let mut child = Command::new("tailwindcss")
        .args(["-i", "-", "--minify"])
        .current_dir(project_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start Tailwind CLI: {e}"))?;

    // Send CSS input to Tailwind
    let input_css = format!(r#"@import "tailwindcss"; @source "."; {}"#, input);
    child
        .stdin
        .take()
        .ok_or("Failed to open stdin")?
        .write_all(input_css.as_bytes())
        .map_err(|e| format!("Failed to write CSS input: {e}"))?;

    // Collect generated CSS
    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to collect Tailwind output: {e}"))?;

    // check tailwindcss command status
    if !output.status.success() {
        let stderr = strip_ansi_escapes::strip(&output.stderr);

        return Err(format!(
            "Tailwind compilation failed:\n{}",
            String::from_utf8_lossy(&stderr)
        ));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 in Tailwind output: {e}"))
}
