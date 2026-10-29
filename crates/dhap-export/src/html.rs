//! Standalone responsive HTML5 guide exporter.

use dhap_core::Result;
use dhap_project::Project;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Exports a project to a modern, responsive, standalone HTML guide (`index.html`) at `output_dir`.
pub fn export_html(project: &Project, frames_dir: Option<&Path>, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;

    let mut step_articles = String::new();
    let mut nav_items = String::new();

    for step in &project.steps {
        let step_anchor = format!("step-{}", step.order);

        nav_items.push_str(&format!(
            r##"<li><a href="#{}" class="nav-link"><span class="step-num">{}</span> <span class="step-nav-title">{}</span></a></li>"##,
            step_anchor, step.order, html_escape(&step.title)
        ));

        let mut image_html = String::new();
        if let Some(frame_rel) = &step.frame_relative_path {
            let asset_name = format!("step_{}.png", step.order);
            let dest_asset_path = assets_dir.join(&asset_name);

            if let Some(f_base) = frames_dir {
                let src_path = f_base.join(frame_rel);
                if src_path.exists() {
                    let _ = fs::copy(&src_path, &dest_asset_path);
                }
            }

            image_html = format!(
                r#"<figure class="step-figure"><img src="assets/{}" alt="Step {}: {}" loading="lazy" class="step-img" /><figcaption>Step {}: {}</figcaption></figure>"#,
                asset_name,
                step.order,
                html_escape(&step.title),
                step.order,
                html_escape(&step.title)
            );
        }

        let timestamp_badge =
            if project.export_profile.include_timestamps && step.timestamp_ms.as_millis() > 0 {
                format!(
                    r#"<span class="timestamp-badge">⏱ {}</span>"#,
                    step.timestamp_ms.format_mm_ss()
                )
            } else {
                String::new()
            };

        let expected_box = if let Some(expected) = &step.expected_result {
            format!(
                r#"<div class="callout expected-box"><div class="callout-title">✓ Expected Result</div><p>{}</p></div>"#,
                html_escape(expected)
            )
        } else {
            String::new()
        };

        let notes_box = if let Some(notes) = &step.notes {
            format!(
                r#"<div class="callout note-box"><div class="callout-title">ℹ Note</div><p>{}</p></div>"#,
                html_escape(notes)
            )
        } else {
            String::new()
        };

        step_articles.push_str(&format!(
            r#"<article id="{}" class="step-card">
  <header class="step-header">
    <div class="step-badge">Step {}</div>
    <div class="step-header-text">
      <h2 class="step-title">{}</h2>
      {}
    </div>
  </header>
  <div class="step-instruction">{}</div>
  {}
  {}
  {}
</article>"#,
            step_anchor,
            step.order,
            html_escape(&step.title),
            timestamp_badge,
            html_escape(&step.instruction),
            image_html,
            expected_box,
            notes_box
        ));
    }

    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{} — Dhap Guide</title>
  <meta name="description" content="{}">
  <style>
    :root {{
      --bg: #0b0f17;
      --surface: #151d2a;
      --surface-border: #233147;
      --text: #e2e8f0;
      --text-muted: #94a3b8;
      --accent: #e11d48;
      --accent-glow: rgba(225, 29, 72, 0.15);
      --link: #38bdf8;
      --font: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    }}
    [data-theme="light"] {{
      --bg: #f8fafc;
      --surface: #ffffff;
      --surface-border: #e2e8f0;
      --text: #0f172a;
      --text-muted: #64748b;
      --accent: #e11d48;
      --accent-glow: rgba(225, 29, 72, 0.08);
      --link: #0284c7;
    }}
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      font-family: var(--font);
      background-color: var(--bg);
      color: var(--text);
      line-height: 1.6;
      display: flex;
      min-height: 100vh;
    }}
    .sidebar {{
      width: 320px;
      background: var(--surface);
      border-right: 1px solid var(--surface-border);
      padding: 2rem 1.5rem;
      position: sticky;
      top: 0;
      height: 100vh;
      overflow-y: auto;
      display: flex;
      flex-direction: column;
    }}
    .brand {{
      font-size: 1.25rem;
      font-weight: 700;
      color: var(--accent);
      margin-bottom: 0.5rem;
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }}
    .brand-sub {{
      font-size: 0.85rem;
      color: var(--text-muted);
      margin-bottom: 2rem;
    }}
    .nav-list {{ list-style: none; display: flex; flex-direction: column; gap: 0.5rem; }}
    .nav-link {{
      display: flex;
      align-items: center;
      gap: 0.75rem;
      color: var(--text-muted);
      text-decoration: none;
      padding: 0.5rem 0.75rem;
      border-radius: 6px;
      font-size: 0.9rem;
      transition: background 0.15s, color 0.15s;
    }}
    .nav-link:hover {{
      background: var(--accent-glow);
      color: var(--text);
    }}
    .step-num {{
      display: inline-flex;
      align-items: center;
      justify-content: center;
      width: 22px;
      height: 22px;
      background: var(--surface-border);
      border-radius: 50%;
      font-size: 0.75rem;
      font-weight: 600;
    }}
    .main-content {{
      flex: 1;
      padding: 3rem 4rem;
      max-width: 900px;
    }}
    .hero {{ margin-bottom: 3rem; border-bottom: 1px solid var(--surface-border); padding-bottom: 2rem; }}
    .hero h1 {{ font-size: 2.25rem; font-weight: 800; letter-spacing: -0.025em; margin-bottom: 0.75rem; }}
    .hero p {{ font-size: 1.1rem; color: var(--text-muted); }}
    .step-card {{
      background: var(--surface);
      border: 1px solid var(--surface-border);
      border-radius: 12px;
      padding: 2rem;
      margin-bottom: 2.5rem;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
    }}
    .step-header {{ display: flex; align-items: flex-start; gap: 1rem; margin-bottom: 1.25rem; }}
    .step-badge {{
      background: var(--accent);
      color: #fff;
      font-weight: 700;
      font-size: 0.85rem;
      padding: 0.35rem 0.75rem;
      border-radius: 20px;
      white-space: nowrap;
    }}
    .step-title {{ font-size: 1.4rem; font-weight: 700; line-height: 1.3; }}
    .timestamp-badge {{
      display: inline-block;
      font-size: 0.8rem;
      color: var(--text-muted);
      margin-top: 0.25rem;
    }}
    .step-instruction {{ font-size: 1.05rem; margin-bottom: 1.5rem; }}
    .step-figure {{ margin: 1.5rem 0; border-radius: 8px; overflow: hidden; border: 1px solid var(--surface-border); }}
    .step-img {{ width: 100%; height: auto; display: block; }}
    figcaption {{ padding: 0.75rem 1rem; font-size: 0.85rem; color: var(--text-muted); background: var(--bg); }}
    .callout {{ padding: 1rem 1.25rem; border-radius: 8px; margin: 1.25rem 0; font-size: 0.95rem; }}
    .expected-box {{ background: rgba(34, 197, 94, 0.1); border-left: 4px solid #22c55e; }}
    .expected-box .callout-title {{ font-weight: 600; color: #22c55e; margin-bottom: 0.25rem; }}
    .note-box {{ background: rgba(56, 189, 248, 0.1); border-left: 4px solid #38bdf8; }}
    .note-box .callout-title {{ font-weight: 600; color: #38bdf8; margin-bottom: 0.25rem; }}
    .footer {{ margin-top: 4rem; padding-top: 2rem; border-top: 1px solid var(--surface-border); font-size: 0.9rem; color: var(--text-muted); }}
    .theme-toggle {{
      margin-top: auto;
      padding: 0.5rem;
      border: 1px solid var(--surface-border);
      border-radius: 6px;
      background: var(--bg);
      color: var(--text);
      cursor: pointer;
      font-size: 0.85rem;
      text-align: center;
    }}
    @media (max-width: 800px) {{
      body {{ flex-direction: column; }}
      .sidebar {{ width: 100%; height: auto; position: static; }}
      .main-content {{ padding: 1.5rem; }}
    }}
  </style>
</head>
<body>
  <aside class="sidebar">
    <div class="brand">ধাপ Dhap Studio</div>
    <div class="brand-sub">Illustrated Workflow Guide</div>
    <ul class="nav-list">
      {}
    </ul>
    <button class="theme-toggle" onclick="toggleTheme()">Toggle Theme</button>
  </aside>
  <main class="main-content">
    <div class="hero">
      <h1>{}</h1>
      <p>{}</p>
    </div>
    {}
    <footer class="footer">
      Generated with <strong>Dhap (ধাপ)</strong> • Record once. Create the video and guide together.
    </footer>
  </main>
  <script>
    function toggleTheme() {{
      const html = document.documentElement;
      const cur = html.getAttribute('data-theme');
      const next = cur === 'dark' ? 'light' : 'dark';
      html.setAttribute('data-theme', next);
    }}
  </script>
</body>
</html>"#,
        html_escape(&project.title),
        html_escape(
            project
                .description
                .as_deref()
                .unwrap_or("Dhap illustrated step-by-step guide")
        ),
        nav_items,
        html_escape(&project.title),
        html_escape(
            project
                .description
                .as_deref()
                .unwrap_or("Follow the illustrated steps below to complete this workflow.")
        ),
        step_articles
    );

    let html_path = output_dir.join("index.html");
    let mut file = File::create(html_path)?;
    file.write_all(html_content.as_bytes())?;

    Ok(())
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use dhap_project::GuideStep;
    use tempfile::tempdir;

    #[test]
    fn test_export_html_generation() {
        let tmp = tempdir().unwrap();
        let mut proj = Project::new("Deploying with Dhap");
        proj.add_step(GuideStep::new(
            "Configure Destination",
            "Set the output path to /dist-guide.",
        ));

        let out = tmp.path().join("html_out");
        export_html(&proj, None, &out).unwrap();

        let index_html = out.join("index.html");
        assert!(index_html.exists());

        let content = fs::read_to_string(index_html).unwrap();
        assert!(content.contains("<title>Deploying with Dhap — Dhap Guide</title>"));
        assert!(content.contains("Configure Destination"));
        assert!(content.contains("Set the output path to /dist-guide."));
    }
}
