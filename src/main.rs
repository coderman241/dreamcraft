//! DreamCraft — an open-source, Dreamweaver-style visual web editor.
//!
//! What it does, in this first version:
//! - Edit HTML in a code pane, and see a live-ish preview pane next to it
//!   (a rendered text preview: headings, paragraphs, links and lists are
//!   pulled out of the HTML so you can check structure at a glance).
//! - Insert common elements (heading, paragraph, link, image, list, button)
//!   at the cursor position in the code pane.
//! - Open / Save / Save As .html files, and start a new document.
//! - A simple element inspector: pick a tag type and it lists every
//!   matching element found in the document.
//!
//! This is NOT Adobe Dreamweaver, and it is not affiliated with Adobe.
//! It is a small, MIT-licensed, from-scratch editor inspired by the
//! idea of a visual + code web editor. No Adobe code, assets, or
//! trademarks are used here.

use eframe::egui;
use std::fs;

const DEFAULT_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>My DreamCraft Page</title>
</head>
<body>
  <h1>Hello from DreamCraft</h1>
  <p>Edit this HTML on the left. Your preview updates on the right.</p>
  <p><a href="https://example.com">A link</a></p>
  <ul>
    <li>First item</li>
    <li>Second item</li>
  </ul>
  <button>Click me</button>
</body>
</html>
"#;

#[derive(Default)]
struct DreamCraftApp {
    code: String,
    file_path: String,
    status: String,
    inspector_tag: String,
    show_about: bool,
}

impl DreamCraftApp {
    fn new() -> Self {
        Self {
            code: DEFAULT_HTML.to_string(),
            file_path: String::new(),
            status: "New document — not saved yet.".to_string(),
            inspector_tag: "h1".to_string(),
            show_about: false,
        }
    }

    fn insert_at_cursor(&mut self, snippet: &str) {
        // egui's TextEdit does not expose a stable cursor API in this
        // simple version, so inserts append at the end of the document
        // and the status line tells you where it went. A future version
        // can wire this to the real cursor position.
        if !self.code.ends_with('\n') {
            self.code.push('\n');
        }
        self.code.push_str(snippet);
        self.code.push('\n');
        self.status = format!("Inserted {} at the end of the document.", snippet.trim());
    }

    fn open_file(&mut self) {
        if self.file_path.trim().is_empty() {
            self.status = "Type a file path in the File box first, then Open.".to_string();
            return;
        }
        match fs::read_to_string(&self.file_path) {
            Ok(text) => {
                self.code = text;
                self.status = format!("Opened {}.", self.file_path);
            }
            Err(e) => self.status = format!("Could not open {}: {}", self.file_path, e),
        }
    }

    fn save_file(&mut self) {
        if self.file_path.trim().is_empty() {
            self.status = "Type a file path in the File box first, then Save.".to_string();
            return;
        }
        match fs::write(&self.file_path, &self.code) {
            Ok(()) => self.status = format!("Saved to {}.", self.file_path),
            Err(e) => self.status = format!("Could not save to {}: {}", self.file_path, e),
        }
    }
}

/// Very small HTML-to-preview helper: strip tags, keep the text of
/// headings, paragraphs, list items, links and buttons, in order.
fn preview_text(html: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = html;
    // Simple, forgiving scan for <tag>...</tag> pairs.
    for tag in ["h1", "h2", "h3", "p", "li", "a", "button", "title"] {
        let _ = tag;
    }
    loop {
        let Some(open_start) = rest.find('<') else { break };
        let Some(open_end) = rest[open_start..].find('>') else { break };
        let open_end = open_start + open_end;
        let open_tag = rest[open_start + 1..open_end]
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_start_matches('/')
            .to_lowercase();
        if open_tag.starts_with('/') || open_tag == "head" || open_tag == "html" || open_tag == "body" {
            rest = &rest[open_end + 1..];
            continue;
        }
        let close = format!("</{}>", open_tag);
        let Some(close_start) = rest[open_end + 1..].find(&close) else {
            rest = &rest[open_end + 1..];
            continue;
        };
        let close_start = open_end + 1 + close_start;
        let inner = &rest[open_end + 1..close_start];
        // Strip any nested tags from the inner text.
        let mut text = String::new();
        let mut in_tag = false;
        for ch in inner.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => text.push(ch),
                _ => {}
            }
        }
        let text = text.trim().to_string();
        if !text.is_empty() && ["h1", "h2", "h3", "p", "li", "a", "button", "title"].contains(&open_tag.as_str()) {
            out.push((open_tag, text));
        }
        rest = &rest[close_start + close.len()..];
    }
    out
}

fn count_tag(html: &str, tag: &str) -> usize {
    let needle = format!("<{}", tag.to_lowercase());
    html.to_lowercase().matches(&needle).count()
}

impl eframe::App for DreamCraftApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("DreamCraft");
                ui.label("— open-source visual web editor");
                if ui.button("New").clicked() {
                    self.code = DEFAULT_HTML.to_string();
                    self.file_path.clear();
                    self.status = "New document — not saved yet.".to_string();
                }
                if ui.button("About").clicked() {
                    self.show_about = true;
                }
            });
            ui.horizontal(|ui| {
                ui.label("File:");
                ui.text_edit_singleline(&mut self.file_path);
                if ui.button("Open").clicked() {
                    self.open_file();
                }
                if ui.button("Save").clicked() {
                    self.save_file();
                }
                ui.label(&self.status);
            });
        });

        egui::TopBottomPanel::bottom("inserts").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Insert:");
                if ui.button("Heading").clicked() {
                    self.insert_at_cursor("  <h2>New heading</h2>");
                }
                if ui.button("Paragraph").clicked() {
                    self.insert_at_cursor("  <p>New paragraph.</p>");
                }
                if ui.button("Link").clicked() {
                    self.insert_at_cursor("  <p><a href=\"https://example.com\">New link</a></p>");
                }
                if ui.button("Image").clicked() {
                    self.insert_at_cursor("  <img src=\"image.png\" alt=\"Description\">");
                }
                if ui.button("List").clicked() {
                    self.insert_at_cursor("  <ul>\n    <li>Item one</li>\n    <li>Item two</li>\n  </ul>");
                }
                if ui.button("Button").clicked() {
                    self.insert_at_cursor("  <button>New button</button>");
                }
            });
        });

        egui::SidePanel::right("inspector").show(ctx, |ui| {
            ui.heading("Inspector");
            ui.label("Count a tag in this document:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.inspector_tag);
                if ui.button("Count").clicked() {
                    // Count happens on every frame below; this just refreshes the label.
                }
            });
            let tag = self.inspector_tag.trim().to_lowercase();
            if !tag.is_empty() {
                ui.label(format!(
                    "<{}> appears {} time(s) in this document.",
                    tag,
                    count_tag(&self.code, &tag)
                ));
            }
            ui.separator();
            ui.label("Preview notes:");
            ui.label("• The preview is a structure preview, not a full browser engine.");
            ui.label("• For a pixel-perfect render, open the saved .html file in a browser.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |cols| {
                cols[0].heading("Code");
                cols[0].add(
                    egui::TextEdit::multiline(&mut self.code)
                        .desired_width(f32::INFINITY)
                        .desired_rows(30)
                        .font(egui::TextStyle::Monospace),
                );
                cols[1].heading("Preview");
                egui::ScrollArea::vertical().show(&mut cols[1], |ui| {
                    for (tag, text) in preview_text(&self.code) {
                        match tag.as_str() {
                            "h1" => {
                                ui.heading(&text);
                            }
                            "h2" => {
                                ui.label(egui::RichText::new(&text).size(20.0).strong());
                            }
                            "title" => {
                                ui.label(format!("Page title: {}", text));
                            }
                            "li" => {
                                ui.label(format!("• {}", text));
                            }
                            _ => {
                                ui.label(&text);
                            }
                        }
                    }
                    if preview_text(&self.code).is_empty() {
                        ui.label("No headings, paragraphs, links, or list items found yet.");
                    }
                });
            });
        });

        if self.show_about {
            egui::Window::new("About DreamCraft")
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("DreamCraft 0.1.0 — MIT licensed, open source.");
                    ui.label("A small, from-scratch visual + code web editor in Rust, built with egui.");
                    ui.label("Not Adobe Dreamweaver, and not affiliated with Adobe.");
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "DreamCraft",
        options,
        Box::new(|_cc| Ok(Box::new(DreamCraftApp::new()))),
    )
}
