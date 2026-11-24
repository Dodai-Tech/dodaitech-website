//! A live gallery of chart types: inline SVG in the theme's colours, animated with CSS
//! (draw-in, grow, sweep, pulse), so it needs no script.

use std::f32::consts::PI;
use std::fmt::Write;

const C: [&str; 4] = [
    "var(--dodai-primary)",
    "var(--dodai-accent)",
    "var(--dodai-info)",
    "var(--dodai-danger)",
];
const SERIES: [f32; 12] = [
    8.0, 14.0, 11.0, 19.0, 15.0, 24.0, 20.0, 28.0, 23.0, 31.0, 27.0, 36.0,
];
const SERIES2: [f32; 12] = [
    20.0, 17.0, 22.0, 15.0, 19.0, 13.0, 18.0, 12.0, 16.0, 10.0, 14.0, 9.0,
];

/// The points of the series `v` spread over a `w` by `h` box, scaled so `max` reaches the top.
/// Fewer than two values have no line to draw and give no points.
pub fn pts(v: &[f32], w: f32, h: f32, max: f32) -> Vec<(f32, f32)> {
    if v.len() < 2 {
        return Vec::new();
    }
    let step = w / (v.len() - 1) as f32;
    v.iter()
        .enumerate()
        .map(|(i, y)| (i as f32 * step, h - y / max * h))
        .collect()
}

/// SVG `points` attribute text for the points `p`.
pub fn poly(p: &[(f32, f32)]) -> String {
    p.iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn grid(w: f32, h: f32) -> String {
    (0..=4)
        .map(|i| {
            let y = h * i as f32 / 4.0;
            format!("<line x1=\"0\" x2=\"{w}\" y1=\"{y}\" y2=\"{y}\" class=\"cg\"/>")
        })
        .collect()
}

/// SVG path data of a circle arc of radius `r` around (`cx`, `cy`) from angle `a0` to `a1` (radians).
pub fn arc(cx: f32, cy: f32, r: f32, a0: f32, a1: f32) -> String {
    let (x0, y0) = (cx + r * a0.cos(), cy + r * a0.sin());
    let (x1, y1) = (cx + r * a1.cos(), cy + r * a1.sin());
    let large = u8::from(a1 - a0 > PI);
    format!("M{x0:.1},{y0:.1} A{r},{r} 0 {large} 1 {x1:.1},{y1:.1}")
}

fn line() -> String {
    let (w, h) = (300.0, 140.0);
    let a = poly(&pts(&SERIES, w, h, 40.0));
    let b = poly(&pts(&SERIES2, w, h, 40.0));
    format!(
        "<svg viewBox=\"-6 -6 312 152\">{}<polyline pathLength=\"1\" class=\"draw\" points=\"{a}\" stroke=\"{}\"/><polyline pathLength=\"1\" class=\"draw d2\" points=\"{b}\" stroke=\"{}\"/></svg>",
        grid(w, h),
        C[0],
        C[1]
    )
}

fn area() -> String {
    let (w, h) = (300.0, 140.0);
    let p = pts(&SERIES, w, h, 40.0);
    format!(
        "<svg viewBox=\"-6 -6 312 152\">{}<polygon class=\"fadein\" points=\"0,{h} {} {w},{h}\" fill=\"{c}\" fill-opacity=\".25\"/><polyline pathLength=\"1\" class=\"draw\" points=\"{}\" stroke=\"{c}\"/></svg>",
        grid(w, h),
        poly(&p),
        poly(&p),
        c = C[2]
    )
}

fn bars() -> String {
    let mut s = String::from("<svg viewBox=\"-6 -6 312 152\">");
    s += &grid(300.0, 140.0);
    for (i, v) in SERIES.iter().enumerate() {
        let hh = v / 40.0 * 140.0;
        let _ = write!(
            s,
            "<rect class=\"grow\" style=\"animation-delay:{}ms\" x=\"{}\" y=\"{}\" width=\"16\" height=\"{hh}\" rx=\"3\" fill=\"{}\"/>",
            i * 70,
            6.0 + i as f32 * 25.0,
            140.0 - hh,
            C[i % 2]
        );
    }
    s + "</svg>"
}

fn stacked() -> String {
    let mut s = String::from("<svg viewBox=\"-6 -6 312 152\">");
    s += &grid(300.0, 140.0);
    for i in 0..8 {
        let (a, b, c) = (SERIES[i], SERIES2[i], 6.0 + (i % 3) as f32 * 3.0);
        let mut y = 140.0;
        for (k, v) in [a, b, c].iter().enumerate() {
            let hh = v / 60.0 * 140.0;
            y -= hh;
            let _ = write!(
                s,
                "<rect class=\"grow\" style=\"animation-delay:{}ms\" x=\"{}\" y=\"{y}\" width=\"26\" height=\"{}\" fill=\"{}\"/>",
                i * 80 + k * 120,
                8.0 + i as f32 * 36.0,
                hh - 1.0,
                C[k]
            );
        }
    }
    s + "</svg>"
}

fn pie(donut: bool) -> String {
    let parts = [0.38, 0.27, 0.21, 0.14];
    let mut a = -PI / 2.0;
    let mut s = String::from("<svg viewBox=\"0 0 140 140\">");
    for (i, p) in parts.iter().enumerate() {
        let e = a + p * 2.0 * PI;
        if donut {
            let _ = write!(
                s,
                "<path class=\"fadein\" style=\"animation-delay:{}ms\" d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"22\"/>",
                i * 160,
                arc(70.0, 70.0, 48.0, a + 0.03, e - 0.03),
                C[i]
            );
        } else {
            let _ = write!(
                s,
                "<path class=\"fadein\" style=\"animation-delay:{}ms\" d=\"M70,70 L{:.1},{:.1} A62,62 0 {} 1 {:.1},{:.1} Z\" fill=\"{}\"/>",
                i * 160,
                70.0 + 62.0 * a.cos(),
                70.0 + 62.0 * a.sin(),
                u8::from(e - a > PI),
                70.0 + 62.0 * e.cos(),
                70.0 + 62.0 * e.sin(),
                C[i]
            );
        }
        a = e;
    }
    s + "</svg>"
}

fn radar() -> String {
    let v = [0.9, 0.55, 0.75, 0.4, 0.85, 0.65];
    let at = |i: usize, r: f32| {
        let a = -PI / 2.0 + i as f32 * 2.0 * PI / 6.0;
        (70.0 + r * a.cos(), 70.0 + r * a.sin())
    };
    let mut s = String::from("<svg viewBox=\"0 0 140 140\">");
    for ring in [20.0, 40.0, 60.0] {
        let p: Vec<_> = (0..6).map(|i| at(i, ring)).collect();
        let _ = write!(
            s,
            "<polygon points=\"{}\" class=\"cg\" fill=\"none\"/>",
            poly(&p)
        );
    }
    let p: Vec<_> = v.iter().enumerate().map(|(i, r)| at(i, r * 60.0)).collect();
    let _ = write!(
        s,
        "<polygon class=\"breathe\" points=\"{}\" fill=\"{c}\" fill-opacity=\".3\" stroke=\"{c}\" stroke-width=\"2\"/>",
        poly(&p),
        c = C[0]
    );
    s + "</svg>"
}

fn scatter() -> String {
    let mut s = String::from("<svg viewBox=\"-6 -6 312 152\">");
    s += &grid(300.0, 140.0);
    for i in 0..34 {
        let x = (i * 37 % 100) as f32 * 2.9 + 4.0;
        let y = 130.0 - ((i * 53 % 100) as f32 * 0.55 + x * 0.25).min(125.0);
        let _ = write!(
            s,
            "<circle class=\"pulse\" style=\"animation-delay:{}ms\" cx=\"{x:.0}\" cy=\"{y:.0}\" r=\"{}\" fill=\"{}\" opacity=\".8\"/>",
            i * 90,
            3 + i % 4,
            C[i % 3]
        );
    }
    s + "</svg>"
}

fn gauge() -> String {
    format!(
        "<svg viewBox=\"0 0 160 100\"><path d=\"{}\" class=\"track\"/><path pathLength=\"1\" class=\"draw\" d=\"{}\" stroke=\"{}\" stroke-width=\"14\" stroke-linecap=\"round\"/><text x=\"80\" y=\"82\" text-anchor=\"middle\" class=\"big\">72%</text></svg>",
        arc(80.0, 86.0, 62.0, PI, 2.0 * PI),
        arc(80.0, 86.0, 62.0, PI, PI + 0.72 * PI),
        C[0]
    )
}

fn heat() -> String {
    let mut s = String::from("<svg viewBox=\"0 0 312 130\">");
    for r in 0..6 {
        for c in 0..16 {
            let v = (((r * 7 + c * 3) % 11) as f32 / 10.0).max(0.1);
            let _ = write!(
                s,
                "<rect class=\"fadein\" style=\"animation-delay:{}ms\" x=\"{}\" y=\"{}\" width=\"17\" height=\"17\" rx=\"3\" fill=\"{}\" fill-opacity=\"{v:.2}\"/>",
                (r + c) * 40,
                c as f32 * 19.5,
                r as f32 * 21.0,
                C[0]
            );
        }
    }
    s + "</svg>"
}

fn candles() -> String {
    let mut s = String::from("<svg viewBox=\"-6 -6 312 152\">");
    s += &grid(300.0, 140.0);
    let mut p = 70.0_f32;
    for i in 0..16 {
        let d = (((i * 29 + 7) % 23) as f32 - 11.0) * 1.3;
        let (o, c) = (p, p + d);
        p = c.clamp(25.0, 115.0);
        let (hi, lo) = (o.max(c) + 6.0, o.min(c) - 6.0);
        let col = if c >= o { C[1] } else { C[3] };
        let _ = write!(
            s,
            "<g class=\"fadein\" style=\"animation-delay:{}ms\"><line x1=\"{x}\" x2=\"{x}\" y1=\"{hi}\" y2=\"{lo}\" stroke=\"{col}\" stroke-width=\"1.5\"/><rect x=\"{}\" y=\"{}\" width=\"10\" height=\"{}\" fill=\"{col}\"/></g>",
            i * 60,
            14.0 + i as f32 * 18.0 - 5.0,
            o.min(c),
            d.abs().max(2.0),
            x = 14.0 + i as f32 * 18.0
        );
    }
    s + "</svg>"
}

fn spark() -> String {
    let p = poly(&pts(&SERIES, 200.0, 40.0, 40.0));
    format!(
        "<svg viewBox=\"-4 -4 208 48\"><polyline pathLength=\"1\" class=\"draw\" points=\"{p}\" stroke=\"{}\"/><circle class=\"pulse\" cx=\"200\" cy=\"{:.1}\" r=\"4\" fill=\"{}\"/></svg>",
        C[1],
        40.0 - SERIES[11],
        C[1]
    )
}

fn treemap() -> String {
    let cells = [
        (0.0, 0.0, 150.0, 90.0),
        (150.0, 0.0, 120.0, 50.0),
        (150.0, 50.0, 120.0, 40.0),
        (0.0, 90.0, 80.0, 50.0),
        (80.0, 90.0, 110.0, 50.0),
        (190.0, 90.0, 80.0, 50.0),
    ];
    let mut s = String::from("<svg viewBox=\"0 0 272 142\">");
    for (i, (x, y, w, h)) in cells.iter().enumerate() {
        let _ = write!(
            s,
            "<rect class=\"fadein\" style=\"animation-delay:{}ms\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"4\" fill=\"{}\" fill-opacity=\"{}\"/>",
            i * 140,
            x + 1.0,
            y + 1.0,
            w - 2.0,
            h - 2.0,
            C[i % 3],
            0.9 - i as f32 * 0.1
        );
    }
    s + "</svg>"
}

/// The gallery: a titled card per chart.
pub fn gallery() -> String {
    let all = [
        ("Line", line()),
        ("Area", area()),
        ("Bar", bars()),
        ("Stacked bar", stacked()),
        ("Pie", pie(false)),
        ("Donut", pie(true)),
        ("Radar", radar()),
        ("Scatter", scatter()),
        ("Gauge", gauge()),
        ("Heatmap", heat()),
        ("Candlestick", candles()),
        ("Treemap", treemap()),
        ("Sparkline", spark()),
    ];
    let cards: String = all
        .iter()
        .map(|(t, svg)| {
            format!("<figure class=\"chart\">{svg}<figcaption>{t}</figcaption></figure>")
        })
        .collect();
    format!("<section><h2>Live gallery</h2><div class=\"charts\">{cards}</div></section>")
}

/// The CSS of the gallery.
pub const CSS: &str = "
.charts{display:grid;grid-template-columns:repeat(auto-fill,minmax(17rem,1fr));gap:1rem}
.chart{margin:0;padding:1rem;border:1px solid var(--dodai-border);border-radius:var(--dodai-radius-lg,1rem);background:var(--dodai-surface);display:flex;flex-direction:column;gap:.5rem}
.chart svg{width:100%;height:9.5rem}.chart figcaption{color:var(--dodai-text-muted);font-size:.85rem}
.chart polyline,.chart .draw{fill:none;stroke-width:2.5;stroke-linejoin:round;stroke-linecap:round}
.chart .cg{stroke:var(--dodai-border);stroke-width:1}.chart .track{fill:none;stroke:var(--dodai-border);stroke-width:14;stroke-linecap:round}
.chart .big{fill:var(--dodai-text);font-size:22px;font-weight:600}
.chart .draw{stroke-dasharray:1;stroke-dashoffset:1;animation:cdraw 5s ease-in-out infinite alternate}.chart .d2{animation-delay:.4s}
.chart .grow{transform-box:fill-box;transform-origin:bottom;animation:cgrow 3.2s ease-in-out infinite alternate}
.chart .fadein{animation:cfade 2.6s ease-in-out infinite alternate}
.chart .pulse{transform-box:fill-box;transform-origin:center;animation:cpulse 2.4s ease-in-out infinite alternate}
.chart .breathe{transform-box:view-box;transform-origin:center;animation:cbreathe 3.2s ease-in-out infinite alternate}
@keyframes cdraw{to{stroke-dashoffset:0}}@keyframes cgrow{from{transform:scaleY(.15)}to{transform:scaleY(1)}}
@keyframes cfade{from{opacity:.2}to{opacity:1}}@keyframes cpulse{from{transform:scale(.6)}to{transform:scale(1.25)}}
@keyframes cbreathe{from{transform:scale(.72)}to{transform:scale(1.05)}}
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fewer_than_two_values_draw_nothing() {
        assert!(pts(&[], 100.0, 50.0, 10.0).is_empty());
        assert!(pts(&[5.0], 100.0, 50.0, 10.0).is_empty());
    }

    #[test]
    fn points_span_the_box_and_the_maximum_touches_the_top() {
        let p = pts(&[0.0, 10.0], 100.0, 50.0, 10.0);
        assert_eq!(p, vec![(0.0, 50.0), (100.0, 0.0)]);
        assert_eq!(poly(&p), "0.0,50.0 100.0,0.0");
    }

    #[test]
    fn a_half_turn_is_not_a_large_arc_and_a_bit_more_is() {
        assert!(arc(0.0, 0.0, 10.0, 0.0, PI).contains(" 0 0 1 "));
        assert!(arc(0.0, 0.0, 10.0, 0.0, PI + 0.1).contains(" 0 1 1 "));
    }

    #[test]
    fn the_gallery_has_no_broken_numbers() {
        let g = gallery();
        assert!(
            !g.contains("NaN")
                && !g.contains("=\"inf")
                && !g.contains(",inf")
                && !g.contains(" inf")
        );
    }
}
