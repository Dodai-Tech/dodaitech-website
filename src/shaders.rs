//! The shader stills and loops of the showcase page. They need a GPU and `ffmpeg`, so this runs
//! on a developer machine (`just shaders`) and the result is committed under `static/`.
use dodai_shaders::{ShaderPalette, gpu::Gpu};
use std::{fs, path::Path};

/// The shader pictures of the showcase page: every generator alone and every preset, one still
/// each (the page cannot run the GPU renderer). `host.js` picks by preset id or first shader id.
///
/// # Errors
/// Returns the first error from writing a file or encoding a picture.
///
/// # Panics
/// Panics without a GPU, or without `ffmpeg` on the path.
pub fn render(
    static_dir: &Path,
    theme: &dodai_core::Theme,
) -> Result<(), Box<dyn std::error::Error>> {
    use dodai_shaders::{Kind, catalog, presets};
    let dir = static_dir.join("design-system/showcase/shaders");
    fs::create_dir_all(&dir)?;
    let mut gpu = Gpu::new().expect("no GPU");
    let mut jobs: Vec<(String, Vec<&'static str>, ShaderPalette)> = catalog()
        .iter()
        .filter(|s| s.kind == Kind::Generator)
        .map(|s| {
            (
                s.id.to_string(),
                vec![s.id],
                ShaderPalette::from_theme(theme),
            )
        })
        .collect();
    jobs.extend(presets::all().map(|p| (p.id.to_string(), p.chain.to_vec(), (p.palette)())));
    for (name, chain, pal) in jobs {
        // 12 frames, played forward and back, so the loop has no seam.
        let mut frames = Vec::new();
        for f in 0..12 {
            let Ok(bgra) = gpu
                .render(&chain, 480, 270, 1.0 + f as f32 * 0.15, 1.0, &pal)
                .map(<[u8]>::to_vec)
            else {
                break; // a shader the adapter cannot run: the page leaves its picture out
            };
            let rgb: Vec<u8> = bgra
                .as_chunks::<4>()
                .0
                .iter()
                .flat_map(|p| [p[2], p[1], p[0]])
                .collect();
            let mut buf = Vec::new();
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 80).encode(
                &rgb,
                480,
                270,
                image::ExtendedColorType::Rgb8,
            )?;
            frames.push(buf);
        }
        if frames.len() < 12 {
            continue;
        }
        fs::write(dir.join(format!("{name}.jpg")), &frames[0])?;
        let tmp = std::env::temp_dir().join("dodai-site-frames");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        for (n, i) in (0..12).chain((1..11).rev()).enumerate() {
            fs::write(tmp.join(format!("{n:03}.jpg")), &frames[i])?;
        }
        let ok = std::process::Command::new("ffmpeg")
            .args(["-v", "error", "-y", "-framerate", "10", "-i"])
            .arg(tmp.join("%03d.jpg"))
            .args([
                "-c:v",
                "libx264",
                "-crf",
                "30",
                "-pix_fmt",
                "yuv420p",
                "-movflags",
                "+faststart",
            ])
            .arg(dir.join(format!("{name}.mp4")))
            .status()
            .expect("ffmpeg is needed to make the shader loops");
        assert!(ok.success(), "ffmpeg failed for {name}");
    }
    Ok(())
}
