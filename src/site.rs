//! Builds the pages of the site into a directory: markup from `dodai-topcoat`, icons from
//! `dodai-icons`. The hand-made files (stylesheet, pictures, showcase wasm) live in `static/`.
use crate::charts;
use dodai_core::Surface;
use dodai_core::{ButtonVariant, ControlSize, Tone, themes};
use dodai_icons::IconId;
use dodai_topcoat::html::{Cx, button_classes, esc, icon, tag};
use std::{fs, io, path::Path};

const GITHUB: &str = "https://github.com/Dodai-Tech";
/// The SynthWave listing in the Garmin Connect IQ Store.
const GARMIN_STORE: &str = "https://apps.garmin.com/apps/bc5757e4-365b-4714-b4a7-b5d720cdc2d6";
const MAIL: &str = "dodaidev@gmail.com";
/// Applies the saved choice (light, dark or system) before first paint; the buttons change it.
const THEME_JS: &str = r#"(function(){var m=matchMedia("(prefers-color-scheme: dark)"),r=document.documentElement;function c(){try{return localStorage.getItem("dodai-theme")||"system"}catch(e){return"system"}}function a(){var v=c();r.dataset.theme=v==="dark"||(v==="system"&&m.matches)?"dodai-dark":"dodai";document.querySelectorAll(".theme button").forEach(function(b){b.setAttribute("aria-pressed",b.dataset.mode===v)})}a();m.addEventListener("change",a);document.addEventListener("click",function(e){var b=e.target.closest&&e.target.closest(".theme button");if(!b)return;try{localStorage.setItem("dodai-theme",b.dataset.mode)}catch(x){}a()});document.addEventListener("DOMContentLoaded",a)})();"#;
const THEME_SWITCH: &str = r#"<div class="theme" role="group" aria-label="Theme"><button type="button" data-mode="light">Light</button><button type="button" data-mode="dark">Dark</button><button type="button" data-mode="system">System</button></div>"#;

struct Site<'a> {
    cx: Cx<'a>,
    dest: &'a Path,
}

impl Site<'_> {
    fn ico(&self, id: IconId) -> String {
        icon(&self.cx, id, 18, false)
    }
    fn link(
        &self,
        v: ButtonVariant,
        s: ControlSize,
        href: &str,
        icon: Option<IconId>,
        text: &str,
    ) -> String {
        let i = icon.map(|i| self.ico(i)).unwrap_or_default();
        format!(
            "<a class=\"{}\" href=\"{href}\">{i}{}</a>",
            button_classes(v, Some(s), false, false),
            esc(text)
        )
    }
    fn chips(items: &[&str]) -> String {
        let t: String = items.iter().map(|i| tag(Tone::Info, true, i)).collect();
        format!("<div class=\"chips\">{t}</div>")
    }
    fn card(&self, ic: IconId, title: &str, inner: &str) -> String {
        let head = format!("<h3>{}{}</h3>", self.ico(ic), esc(title));
        format!("<div class=\"dodai-panel\">{head}{inner}</div>")
    }
    fn nav(&self, depth: &str) -> String {
        let l = |h: &str, t: &str| format!("<a href=\"{depth}{h}\">{t}</a>");
        let gh = self.ico(IconId::GithubLogo);
        format!(
            "<header><a class=\"brand\" href=\"{depth}\"><img src=\"/img/logo.png\" width=\"32\" height=\"32\" alt=\"\"><b>Dodai Tech</b></a><nav>{}{}<a class=\"icon-link\" href=\"{GITHUB}\" aria-label=\"GitHub\" title=\"GitHub\">{gh}</a></nav>{THEME_SWITCH}</header>",
            l("design-system/", "Design system"),
            l("products/watchfaces/", "Watch faces")
        )
    }
    fn page(&self, dir: &str, title: &str, desc: &str, body: &str) -> io::Result<()> {
        let depth = "../".repeat(dir.matches('/').count() + usize::from(!dir.is_empty()));
        let depth = if dir.is_empty() { String::new() } else { depth };
        let gh = self.ico(IconId::GithubLogo);
        let html = format!(
            "<!doctype html>\n<html lang=\"en\" data-theme=\"dodai\"><head>\n<meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n<title>{title}</title>\n<meta name=\"description\" content=\"{desc}\">\n<link rel=\"icon\" href=\"/favicon.png\">\n<link rel=\"stylesheet\" href=\"/dodai.css\"><link rel=\"stylesheet\" href=\"/site.css\">\n<script>{THEME_JS}</script>\n</head>\n<body><div class=\"wrap\">{}\n<main>{body}</main>\n<footer><span>© 2026 Dodai Tech</span><span><a class=\"icon-link\" href=\"{GITHUB}\">{gh}GitHub</a> · <a href=\"/privacy/\">Privacy policy</a> · <a href=\"mailto:{MAIL}\">{MAIL}</a></span></footer>\n</div></body></html>\n",
            self.nav(&depth)
        );
        let p = self.dest.join(dir);
        fs::create_dir_all(&p)?;
        fs::write(p.join("index.html"), html)
    }
}

const COMPONENTS: &[(&str, &str, &[&str])] = &[
    (
        "Foundations",
        "Text, headings, labels, the colour palette, type scale, motion tokens, spacing and elevation, layout, frosted panels, shape tokens and the icon sets.",
        &[
            "Text · Heading · Label · Mono",
            "Colour palette",
            "Type scale",
            "Motion tokens",
            "Spacing",
            "Elevation",
            "Layout",
            "Frosted panels",
            "Shape tokens",
            "Icon sets",
        ],
    ),
    (
        "Shape",
        "Roundness, borders, corner shapes and button crops: the geometry that tells the designs apart.",
        &[
            "Shape gallery",
            "Roundness · Borders",
            "Corner shape",
            "Button crop",
            "Border",
        ],
    ),
    (
        "Surface effects",
        "Textures, frosted glass, light effects, depth and the click ripple.",
        &[
            "Presets",
            "Component textures",
            "Container textures",
            "Frosted glass",
            "Light FX",
            "Edges · Depth",
            "Click ripple",
        ],
    ),
    (
        "Actions",
        "Buttons in every variant, icon buttons and FABs, button groups, split buttons, toolbars and snackbars.",
        &[
            "Buttons · IconButton · FAB",
            "Button groups · Split button",
            "Toolbar · Snackbar",
        ],
    ),
    (
        "Data display",
        "Tags, chips, badges, lists, timelines, tables with sorting and virtual scroll, code and markdown, media, clocks and timers.",
        &[
            "Tags · Chips · Badges",
            "List items",
            "Rating · Tooltips",
            "Timeline",
            "Tables",
            "Code · Markdown",
            "Media",
            "Clocks · Timers",
        ],
    ),
    (
        "Navigation",
        "Menus, tabs, trees, sidebars, steppers, breadcrumbs, pagination, carousels, drawers, dialogs, sheets and app bars.",
        &[
            "Popover · Hover card · Context menu",
            "Menu · Sub-menu",
            "Tabs",
            "Tree",
            "Sidebar",
            "Stepper",
            "Breadcrumb",
            "Pagination",
            "Carousel",
            "Drawer · Dialog · Sheet",
            "App bar",
        ],
    ),
    (
        "Inputs",
        "Toggles, checkboxes, radios, sliders, knobs, text fields, search, tag and password fields, forms and date and time pickers.",
        &[
            "Toggle · Checkbox · Radio",
            "Checkbox variants",
            "Toggle group · Segmented",
            "Slider · Range",
            "Knob",
            "Text field",
            "Search",
            "Tag field",
            "Password field",
            "Forms",
            "Date · Time pickers",
        ],
    ),
    (
        "Loading",
        "Spinners, skeletons and loading indicators.",
        &["Spinner · Indeterminate · Skeleton", "Loading indicator"],
    ),
    (
        "Animation",
        "Physics-based dynamics and one-shot mixins.",
        &["Dynamics · physics models", "Dynamics · one-shot mixins"],
    ),
];

/// The privacy policy: the intro paragraph, then headed sections (heading, paragraphs as HTML).
/// The contact section is added by [`privacy_page`] because it needs [`MAIL`].
const PRIVACY_INTRO: &str = "Last updated: 17 November 2026. Applies to the Wear OS watch face \"SynthWave Watch Face\" (package <code>com.dodai.watchfaces.synthwave</code>) and other watch faces published by Dodai.";
const PRIVACY: &[(&str, &[&str])] = &[
    (
        "Summary",
        &[
            "The watch faces do not collect, store, transmit or share any personal data. They have no accounts, no ads, no analytics and no network access.",
        ],
    ),
    (
        "Data the watch face reads",
        &[
            "To show them on the dial, the watch face reads the following from your watch, on the watch only: the time and date, battery level, step count, heart rate, and the data of the complications you choose (for example sunrise/sunset or notification count). Heart rate and step count are read through the standard Wear OS watch face APIs and require the permissions you grant on your watch.",
            "This data is used only to draw the watch face. It is not saved by the watch face, not sent off the watch, and not shared with anyone, including Dodai.",
        ],
    ),
    (
        "Third parties",
        &[
            "The watch face contains no third-party SDKs. Google Play and Wear OS process data under Google's own privacy policy.",
        ],
    ),
    (
        "Children",
        &["The watch face is not directed at children and collects no data from anyone."],
    ),
    (
        "Changes",
        &[
            "If this policy changes, the new version will be published at this address with a new date.",
        ],
    ),
];

const CHARTS: &[(&str, &[&str])] = &[
    (
        "Trends",
        &[
            "Line · area",
            "Time axis",
            "Band",
            "Stacked area",
            "Stream",
            "Horizon",
            "Sparkline",
            "Calendar heatmap",
            "Control chart",
            "Event timeline",
            "Span fan",
            "Peak range",
            "Bubble timeline",
        ],
    ),
    (
        "Comparison",
        &[
            "Bar",
            "Bullet",
            "Paired bars",
            "Lollipop · Dumbbell",
            "Slope",
            "Bump",
            "Radar",
            "Pareto",
            "Diverging bars",
        ],
    ),
    (
        "Part of a whole",
        &[
            "Pie",
            "Donut",
            "Treemap",
            "Sunburst",
            "Icicle",
            "Circle packing",
            "Proportional squares",
            "Waffle",
            "Marimekko",
            "Radial bar",
            "100% stacked",
            "Shard map",
        ],
    ),
    (
        "Distribution",
        &[
            "Histogram",
            "Box plot",
            "Violin",
            "Ridgeline",
            "Beeswarm",
            "Bubble swarm",
            "ECDF",
            "Error bars",
        ],
    ),
    (
        "Relationship",
        &[
            "Scatter",
            "Heatmap",
            "Hexbin",
            "Parallel coordinates",
            "Network graph",
            "Time scatter",
        ],
    ),
    (
        "Flow",
        &[
            "Sankey",
            "Split flow",
            "Chord",
            "Arc diagram",
            "Funnel",
            "Waterfall",
        ],
    ),
    (
        "Finance",
        &[
            "Candlestick",
            "Renko",
            "Order book depth",
            "Secondary axis",
            "Chart stack",
        ],
    ),
    ("Planning", &["Gantt"]),
    (
        "Status",
        &[
            "Slot grid",
            "Status strip",
            "Bucket bar",
            "Stage bars",
            "Gradient scale",
        ],
    ),
    ("Single value", &["KPI tile"]),
    ("Examples", &["Live throughput", "Network overview"]),
];

/// Writes every generated page and stylesheet under `dest`.
///
/// # Errors
/// Returns the first I/O error while creating directories or writing files.
pub fn build(dest: &Path) -> io::Result<()> {
    let theme = themes::all()
        .into_iter()
        .find(|t| t.id() == "dodai")
        .expect("the dodai theme");
    // Only the Dodai design: smaller than the all-theme stylesheet.
    let own: Vec<_> = themes::all()
        .into_iter()
        .filter(|t| t.meta.family == theme.meta.family)
        .collect();
    // The page sits on the theme's own gradient wash, as the showcase does.
    let wash_for = |t: &dodai_core::Theme| {
        dodai_topcoat::backdrop_css(t).map_or(String::new(), |b| format!("\n[data-theme=\"{}\"] body{{background-image:{b};background-attachment:fixed;background-size:100% 100%}}\n", t.id()))
    };
    fs::write(
        dest.join("dodai.css"),
        dodai_topcoat::multi_theme_stylesheet(&own)
            + &wash_for(&theme)
            + &wash_for(
                &themes::all()
                    .into_iter()
                    .find(|t| t.id() == "dodai-dark")
                    .expect("dodai-dark theme"),
            ),
    )?;

    showcase_page(dest)?;

    let s = Site {
        cx: Cx {
            theme: &theme,
            surface: Surface::INHERIT,
        },
        dest,
    };
    home_page(&s)?;
    watchfaces_page(&s)?;
    design_system_page(&s)?;
    components_page(&s)?;
    charts_page(&s)?;
    privacy_page(&s)
}

/// Pictures of the projects: a frame of one of the site's own shaders, or the watch faces.
fn faces(pre: &str, a: &str, b: &str) -> String {
    format!(
        "<div class=\"faces\"><img src=\"{pre}img/{a}.png\" alt=\"\"><img src=\"{pre}img/{b}.png\" alt=\"\"></div>"
    )
}

/// The home page.
fn home_page(s: &Site<'_>) -> io::Result<()> {
    let big = |href: &str, ic, t: &str, v| s.link(v, ControlSize::Large, href, Some(ic), t);
    let prim = ButtonVariant::Primary;
    let out = ButtonVariant::Outline;

    let badge = |tone, t: &str| tag(tone, true, t);
    let project = |href: &str,
                   pic: String,
                   ic: IconId,
                   title: &str,
                   kind: String,
                   text: &str,
                   links: String| {
        format!(
            "<div class=\"dodai-panel proj\"><a class=\"pic\" href=\"{href}\">{pic}</a><h3>{}{title}</h3><div class=\"chips kind\">{kind}</div><p>{text}</p><div class=\"row\">{links}</div></div>",
            s.ico(ic)
        )
    };
    let open_badges = format!(
        "{}{}",
        badge(Tone::Success, "Open source"),
        badge(Tone::Info, "Rust")
    );
    let open_source = [
        project("design-system/", "<img src=\"img/showcase.jpg\" alt=\"The Dodai UI showcase\">".to_string(), IconId::Palette, "Dodai UI", open_badges,
            "A design-system kit for Rust and the web with four complete designs (Dodai, Cyberpunk, Material 3 and Neo-Brutalism), each in light and dark. Tokens, 160+ components, charts, icons, motion and shaders for gpui, iced, egui, Slint, Bevy and more.",
            format!("{}{}", s.link(prim, ControlSize::Small, "design-system/showcase/", Some(IconId::Palette), "Live showcase"), s.link(out, ControlSize::Small, "design-system/", Some(IconId::OpenInNew), "Read more"))),
    ].concat();
    let paid = [
        project("products/watchfaces/", faces("", "wear-synthwave", "garmin-synthwave_mountains"), IconId::Watch, "SynthWave watch face", format!("{}{}", badge(Tone::Warning, "Paid"), badge(Tone::Info, "Wear OS · Garmin")),
            "A retro outrun sunset for your wrist, with two looks, Flat and Mountains. The same face on Wear OS and Garmin, with a true always-on mode. Available now on the Connect IQ Store for Garmin; the Wear OS version is coming to Google Play.",
            format!(
                "{}{}",
                s.link(prim, ControlSize::Small, "products/watchfaces/", Some(IconId::Watch), "See the watch face"),
                s.link(out, ControlSize::Small, GARMIN_STORE, Some(IconId::OpenInNew), "Get it on Connect IQ")
            ),
        ),
    ].concat();
    let stack = [
        (
            IconId::Phone,
            "Android and mobile",
            "Native apps in Kotlin and Jetpack Compose, built for large screens, foldables and every window size.",
            &["Kotlin", "Jetpack Compose", "Material 3", "Gradle"][..],
        ),
        (
            IconId::Watch,
            "Wear OS and Garmin",
            "Watch Face Format faces and Connect IQ faces in Monkey C, with a true always-on mode.",
            &["Watch Face Format", "Connect IQ", "Always-on"][..],
        ),
        (
            IconId::Terminal,
            "Rust and the web",
            "Fast native tools, desktop apps and embedded firmware in Rust, and web pages from the same design tokens.",
            &["Rust", "ESP32", "Web / CSS"][..],
        ),
    ];
    let stack_html: String = stack
        .iter()
        .map(|(i, t, p, c)| s.card(*i, t, &format!("<p>{p}</p>{}", Site::chips(c))))
        .collect();
    let home = format!(
        "<div class=\"hero\"><div><h1>Open source, and a few things worth paying for.</h1><p class=\"lead\">We develop open source software, starting with Dodai UI, a design-system kit with four complete designs. We also make watch faces for Wear OS and Garmin, which we sell.</p><div class=\"row\">{}{}</div></div><div class=\"pair\"><img src=\"img/wear-synthwave.png\" alt=\"SynthWave watch face, flat look\"><img src=\"img/wear-synthwave_mountains.png\" alt=\"SynthWave watch face, mountains look\"></div></div>\
<section id=\"open-source\"><h2>Open source</h2><p class=\"lead\">Free to use and read.</p><div class=\"grid two\">{open_source}</div></section>\
<section id=\"products\"><h2>Products</h2><p class=\"lead\">Finished work you can buy. Paying for it keeps the open source side going.</p><div class=\"grid two\">{paid}</div></section>\
<section id=\"stack\"><h2>What we build with</h2><div class=\"grid narrow\">{stack_html}</div></section>\
<section id=\"about\"><h2>About</h2><p class=\"lead\">Dodai Tech is a small software studio. We design once and build natively for each platform, from Rust libraries to Wear OS and Garmin apps. No accounts, no ads, no tracking. Our code lives in the <a href=\"{GITHUB}\">Dodai-Tech GitHub organisation</a>; write to <a href=\"mailto:{MAIL}\">{MAIL}</a>.</p></section>",
        big("#open-source", IconId::Code, "Open source", prim),
        big("#products", IconId::Dashboard, "Products", out)
    );
    s.page("", "Dodai Tech – Open source and products", "Dodai Tech develops open source projects, a design-system kit with four designs, and sells watch faces for Wear OS and Garmin.",  &home)?;
    Ok(())
}

/// The watch faces page.
fn watchfaces_page(s: &Site<'_>) -> io::Result<()> {
    let prim = ButtonVariant::Primary;
    // Watch faces
    let gal: String = [("wear-synthwave", "Wear OS, Flat"), ("wear-synthwave_mountains", "Wear OS, Mountains"), ("garmin-synthwave", "Garmin, Flat"), ("garmin-synthwave_mountains", "Garmin, Mountains"), ("wear-synthwave_mountains-ambient", "Wear OS, always-on"), ("garmin-synthwave_mountains-ambient", "Garmin, always-on")]
        .iter().map(|(f, c)| format!("<figure><img src=\"../../img/{f}.png\" alt=\"{c}\"><figcaption>{c}</figcaption></figure>")).collect();
    let wf = format!(
        "<section><h1>Watch faces</h1><p class=\"lead\">One look on two platforms: Wear OS in Watch Face Format, and Garmin Connect IQ. Every face has an always-on variant that keeps the lit area small, so AMOLED screens stay safe.</p></section>\
<section><h2>SynthWave</h2><div class=\"stage\"><div class=\"gal\">{gal}</div></div></section>\
<section><div class=\"grid\"><div class=\"dodai-panel\">{}<h3>{}SynthWave for Wear OS</h3><p>A watch face in Watch Face Format with two looks, Flat and Mountains at dusk. Light on the battery, with a true always-on mode.</p><p>Coming soon to Google Play.</p></div><div class=\"dodai-panel\">{}<h3>{}SynthWave for Garmin</h3><p>The same watch face for Connect IQ, with an AMOLED-safe always-on mode.</p><p>{}</p></div></div></section>",
        faces("../../", "wear-synthwave", "wear-synthwave_mountains"),
        s.ico(IconId::Watch),
        faces("../../", "garmin-synthwave", "garmin-synthwave_mountains"),
        s.ico(IconId::Watch),
        s.link(
            prim,
            ControlSize::Small,
            GARMIN_STORE,
            Some(IconId::OpenInNew),
            "Get it on Connect IQ"
        )
    );
    s.page(
        "products/watchfaces",
        "Watch faces – Dodai Tech",
        "Watch faces for Wear OS and Garmin with matching looks and always-on modes.",
        &wf,
    )?;
    Ok(())
}

/// The design system overview page.
fn design_system_page(s: &Site<'_>) -> io::Result<()> {
    let prim = ButtonVariant::Primary;
    let out = ButtonVariant::Outline;
    let n_charts: usize = CHARTS.iter().map(|c| c.1.len()).sum();
    let n_sections: usize = COMPONENTS.iter().map(|c| c.2.len()).sum();
    // Design system
    let cards = [
        (IconId::Devices, "Toolkits", "<p>A native adapter per framework, all from the same tokens.</p>".to_string(), Site::chips(&["gpui", "iced", "egui", "Slint", "Bevy", "Dioxus", "Yew", "Leptos", "Freya", "ratatui", "Web (CSS)"])),
        (IconId::Palette, "Designs", "<p>Four complete designs, each with light and dark themes.</p>".into(), Site::chips(&["Dodai (frosted glass)", "Cyberpunk", "Material 3", "Neo-Brutalism"])),
        (IconId::Extension, "Components", format!("<p>160+ components in {n_sections} showcase sections, from buttons to date-range pickers.</p>"), format!("<div class=\"row\">{}</div>", s.link(out, ControlSize::Small, "components/", None, "Browse components"))),
        (IconId::BarChart, "Charts", format!("<p>{n_charts} chart types for dashboards and analysis, in eleven families.</p>"), format!("<div class=\"row\">{}</div>", s.link(out, ControlSize::Small, "charts/", None, "Browse charts"))),
        (IconId::Brush, "Shaders", format!("<p>{} pixel shaders written in Rust that follow the theme palette: cityscapes, CRT and VHS monitors, painted scenes. The banner behind this page is one of them.</p>", dodai_shaders::catalog().len()), String::new()),
        (IconId::Memory, "Icons and motion", "<p>One typed icon id across icon packs, plus easing, springs, stagger and entry effects.</p>".into(), String::new()),
    ];
    let cards: String = cards
        .iter()
        .map(|(i, t, p, c)| s.card(*i, t, &format!("{p}{c}")))
        .collect();
    s.page("design-system", "Dodai design system – Dodai Tech", "Tokens, components, themes, charts, icons and motion for Rust and the web.",
        &format!("<section><h1>The Dodai design system</h1><p class=\"lead\">One token set, many toolkits. Colour roles, type, spacing, shape, glow and motion are defined once and resolved into components for every framework we use, so an app, a dashboard and a website feel like one product.</p><div class=\"row\">{}{}{}{}</div></section><section><iframe class=\"live\" src=\"showcase/\" title=\"The Dodai UI showcase\" loading=\"lazy\"></iframe></section><section><div class=\"grid\">{cards}</div></section>",
            s.link(prim, ControlSize::Large, "showcase/", Some(IconId::Palette), "Open the live showcase"), s.link(out, ControlSize::Large, "components/", Some(IconId::Extension), "Components"), s.link(out, ControlSize::Large, "charts/", Some(IconId::BarChart), "Charts"), s.link(out, ControlSize::Large, &format!("{GITHUB}/dodai-ui"), Some(IconId::Code), "GitHub")))?;
    Ok(())
}

/// The components page.
fn components_page(s: &Site<'_>) -> io::Result<()> {
    let out = ButtonVariant::Outline;
    let comp: String = COMPONENTS
        .iter()
        .map(|(h, d, c)| {
            format!(
                "<section><h2>{h}</h2><p class=\"lead\">{d}</p>{}</section>",
                Site::chips(c)
            )
        })
        .collect();
    s.page("design-system/components", "Components – Dodai Tech", "Every component section of the Dodai design system.",
        &format!("<section><h1>Components</h1><p class=\"lead\">The component sections of the Dodai design system, by category. Each one is shown in every design and theme in the live showcase.</p><div class=\"row\">{}{}</div></section>{comp}",
            s.link(out, ControlSize::Medium, "../", Some(IconId::Palette), "Design system"), s.link(out, ControlSize::Medium, "../charts/", Some(IconId::BarChart), "Charts")))?;
    Ok(())
}

/// The charts page (and its stylesheet).
fn charts_page(s: &Site<'_>) -> io::Result<()> {
    let out = ButtonVariant::Outline;
    let n_charts: usize = CHARTS.iter().map(|c| c.1.len()).sum();
    let gal2 = charts::gallery();
    fs::write(s.dest.join("charts.css"), charts::CSS)?;
    let ch: String = CHARTS
        .iter()
        .map(|(h, c)| format!("<section><h2>{h}</h2>{}</section>", Site::chips(c)))
        .collect();
    s.page("design-system/charts", "Charts – Dodai Tech", "The chart types of the Dodai design system.",
        &format!("<section><h1>Charts</h1><p class=\"lead\">{n_charts} chart types in eleven families. They draw from the active theme, so every chart follows the design and light or dark mode, and they animate with live data.</p><div class=\"row\">{}{}</div></section>{gal2}{ch}",
            s.link(out, ControlSize::Medium, "../", Some(IconId::Palette), "Design system"), s.link(out, ControlSize::Medium, "../components/", Some(IconId::Extension), "Components")))?;
    Ok(())
}

/// The privacy policy page, in the site's own look.
fn privacy_page(s: &Site) -> io::Result<()> {
    let sections: String = PRIVACY
        .iter()
        .map(|(h, ps)| {
            let body: String = ps.iter().map(|p| format!("<p>{p}</p>")).collect();
            format!("<h2>{h}</h2>{body}")
        })
        .collect();
    s.page(
        "privacy",
        "Privacy Policy – Dodai Tech",
        "How Dodai Tech watch faces and this website handle data: no accounts, no ads, no tracking.",
        &format!("<section class=\"prose\"><h1>Privacy policy</h1><p class=\"lead\">{PRIVACY_INTRO}</p><div class=\"dodai-panel\">{sections}<h2>Contact</h2><p>Dodai – <a href=\"mailto:{MAIL}\">{MAIL}</a></p></div></section>"),
    )
}

fn showcase_page(dest: &Path) -> io::Result<()> {
    let html = "<!doctype html>\n<html lang=\"en\" data-theme=\"dodai\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Dodai UI showcase – Dodai Tech</title><link rel=\"icon\" href=\"/favicon.png\"><style>html,body{margin:0;height:100%}body{background:var(--dodai-background);color:var(--dodai-text);font-family:var(--dodai-font-body);font-size:var(--dodai-size-md);line-height:1.5}</style></head><body><noscript>The showcase needs JavaScript and WebAssembly.</noscript><script src=\"host.js\"></script></body></html>\n";
    fs::write(dest.join("design-system/showcase/index.html"), html)
}
