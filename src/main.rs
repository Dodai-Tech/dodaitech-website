//! `dodaitech-website [OUT]` builds the site into `OUT` (default `public`): the files of `static/`,
//! then the generated pages. `dodaitech-website shaders` renders the
//! showcase shader pictures into `static/` instead.
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    if arg.as_deref() == Some("shaders") {
        let theme = dodai_core::themes::all()
            .into_iter()
            .find(|t| t.id() == "dodai-dark")
            .expect("dodai-dark theme");
        return match dodaitech_website::shaders::render(Path::new("static"), &theme) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("rendering the shaders: {e}");
                ExitCode::FAILURE
            }
        };
    }
    let out = Path::new(arg.as_deref().unwrap_or("public"));
    match dodaitech_website::build_site(Path::new("static"), out) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("building into {}: {e}", out.display());
            ExitCode::FAILURE
        }
    }
}
