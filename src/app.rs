use eframe::egui::{self, Color32, CornerRadius, Frame, Margin};

use crate::graph::AudioGraph;
use crate::pipewire;

#[derive(Clone, PartialEq)]
enum Panel {
    Graph,
    Volumes,
}

pub struct LotusAudioApp {
    graph: AudioGraph,
    panel: Panel,
    error: Option<String>,
    dragging_from: Option<String>,
    selected_node: Option<String>,
    refresh_counter: u32,
}

impl Default for LotusAudioApp {
    fn default() -> Self {
        let mut app = Self {
            graph: AudioGraph::new(),
            panel: Panel::Graph,
            error: None,
            dragging_from: None,
            selected_node: None,
            refresh_counter: 0,
        };
        app.refresh();
        app
    }
}

impl LotusAudioApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }

    fn refresh(&mut self) {
        match pipewire::fetch_graph() {
            Ok(graph) => {
                self.graph = graph;
                self.error = None;
            }
            Err(e) => {
                self.error = Some(format!("Failed to fetch audio graph: {}", e));
            }
        }
    }

    fn do_create_link(&mut self, output: &str, input: &str) {
        if let Err(e) = pipewire::create_link(output, input) {
            self.error = Some(format!("Failed to create link: {}", e));
        }
        self.refresh();
    }

    fn do_remove_link(&mut self, output: &str, input: &str) {
        if let Err(e) = pipewire::remove_link(output, input) {
            self.error = Some(format!("Failed to remove link: {}", e));
        }
        self.refresh();
    }
}

impl eframe::App for LotusAudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let accent = Color32::from_rgb(0xC4, 0xA8, 0xE2);
        let bg_dark = Color32::from_rgb(0x14, 0x12, 0x18);
        let bg_card = Color32::from_rgb(0x1C, 0x1A, 0x22);
        let text_color = Color32::from_rgb(0xD8, 0xD0, 0xE8);
        let dim_text = Color32::from_rgb(0x88, 0x78, 0x98);

        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.panel_fill = bg_dark;
        style.visuals.window_fill = bg_card;
        style.visuals.faint_bg_color = bg_card;
        style.visuals.extreme_bg_color = bg_dark;
        style.visuals.widgets.noninteractive.bg_fill = bg_card;
        style.visuals.widgets.inactive.bg_fill = bg_card;
        style.visuals.widgets.active.bg_fill = Color32::from_rgb(0x28, 0x24, 0x30);
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(0x38, 0x30, 0x48);
        style.visuals.hyperlink_color = accent;
        style.visuals.selection.stroke.color = accent;
        style.visuals.selection.bg_fill = Color32::from_rgba_premultiplied(0xC4, 0xA8, 0xE2, 40);
        ctx.set_style(style);

        egui::TopBottomPanel::top("top_bar")
            .frame(Frame {
                fill: bg_dark,
                inner_margin: Margin::symmetric(16, 8),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(
                        egui::RichText::new("◈ LOTUS AUDIO")
                            .color(accent)
                            .size(20.0),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button("↻ Refresh")
                            .on_hover_text("Refresh audio graph")
                            .clicked()
                        {
                            self.refresh();
                        }
                        ui.separator();
                        ui.selectable_value(&mut self.panel, Panel::Graph, "Graph");
                        ui.selectable_value(&mut self.panel, Panel::Volumes, "Volumes");
                    });
                });
            });

        if let Some(ref err) = self.error.clone() {
            egui::TopBottomPanel::top("error_bar")
                .frame(Frame {
                    fill: Color32::from_rgb(0x38, 0x10, 0x10),
                    inner_margin: Margin::symmetric(8, 4),
                    ..Default::default()
                })
                .show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new(err)
                            .color(Color32::from_rgb(0xFF, 0x60, 0x60))
                            .size(12.0),
                    );
                });
        }

        match self.panel {
            Panel::Graph => self.show_graph_panel(ctx, accent, bg_card, text_color, dim_text),
            Panel::Volumes => self.show_volumes_panel(ctx, accent, bg_card, text_color, dim_text),
        }
    }
}

impl LotusAudioApp {
    fn show_graph_panel(
        &mut self,
        ctx: &egui::Context,
        accent: Color32,
        bg_card: Color32,
        text_color: Color32,
        dim_text: Color32,
    ) {
        egui::SidePanel::left("sources_panel")
            .resizable(true)
            .default_width(280.0)
            .frame(Frame {
                fill: bg_card,
                inner_margin: Margin::symmetric(8, 8),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("◈ SOURCES").color(accent).size(14.0));
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let sources = self.graph.sources();
                    let source_ids: Vec<String> = sources.iter().map(|n| n.id.clone()).collect();
                    for id in &source_ids {
                        if let Some(node) = self.graph.nodes.get(id) {
                            node_card(
                                ui,
                                node,
                                accent,
                                text_color,
                                dim_text,
                                &self.graph.output_ports(id),
                                &self.graph.links,
                            );
                        }
                    }
                });
            });

        egui::SidePanel::right("sinks_panel")
            .resizable(true)
            .default_width(280.0)
            .frame(Frame {
                fill: bg_card,
                inner_margin: Margin::symmetric(8, 8),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("◈ SINKS").color(accent).size(14.0));
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let sinks = self.graph.sinks();
                    let sink_ids: Vec<String> = sinks.iter().map(|n| n.id.clone()).collect();
                    for id in &sink_ids {
                        if let Some(node) = self.graph.nodes.get(id) {
                            node_card(
                                ui,
                                node,
                                accent,
                                text_color,
                                dim_text,
                                &self.graph.input_ports(id),
                                &self.graph.links,
                            );
                        }
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgb(0x14, 0x12, 0x18),
                inner_margin: Margin::symmetric(8, 8),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("◈ CONNECTIONS").color(accent).size(14.0));
                ui.separator();
                ui.label(
                    egui::RichText::new(
                        "Connections between audio nodes.\nClick ✕ to remove a connection.",
                    )
                    .color(dim_text)
                    .size(11.0),
                );
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let links = self.graph.links.clone();
                    for link in &links {
                        let output_short =
                            link.output_port.split(':').last().unwrap_or(&link.output_port);
                        let input_short =
                            link.input_port.split(':').last().unwrap_or(&link.input_port);

                        Frame {
                            fill: bg_card,
                            corner_radius: CornerRadius::same(6),
                            inner_margin: Margin::symmetric(8, 6),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(output_short)
                                        .color(accent)
                                        .size(12.0),
                                );
                                ui.label(egui::RichText::new("  →  ").color(dim_text));
                                ui.label(
                                    egui::RichText::new(input_short)
                                        .color(text_color)
                                        .size(12.0),
                                );
                                let out_p = link.output_port.clone();
                                let in_p = link.input_port.clone();
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.button("✕").clicked() {
                                            self.do_remove_link(&out_p, &in_p);
                                        }
                                    },
                                );
                            });
                        });
                        ui.add_space(4.0);
                    }

                    if self.graph.links.is_empty() {
                        ui.label(
                            egui::RichText::new("No connections yet")
                                .color(dim_text)
                                .size(12.0),
                        );
                    }
                });
            });
    }

    fn show_volumes_panel(
        &mut self,
        ctx: &egui::Context,
        accent: Color32,
        bg_card: Color32,
        text_color: Color32,
        _dim_text: Color32,
    ) {
        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgb(0x14, 0x12, 0x18),
                inner_margin: Margin::symmetric(16, 8),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.heading(egui::RichText::new("◈ VOLUMES").color(accent).size(16.0));
                ui.separator();
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let node_ids: Vec<String> =
                        self.graph.nodes.keys().cloned().collect();
                    for id in &node_ids {
                        if let Some(node) = self.graph.nodes.get(id).cloned() {
                            Frame {
                                fill: bg_card,
                                corner_radius: CornerRadius::same(8),
                                inner_margin: Margin::symmetric(12, 8),
                                ..Default::default()
                            }
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(&node.name)
                                            .color(text_color)
                                            .size(13.0),
                                    );
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "{:.0}%",
                                                    node.volume * 100.0
                                                ))
                                                .color(accent)
                                                .size(12.0),
                                            );
                                        },
                                    );
                                });
                                if let Some(mut_node) = self.graph.nodes.get_mut(id) {
                                    ui.add(
                                        egui::Slider::new(&mut mut_node.volume, 0.0..=1.5)
                                            .show_value(false)
                                            .trailing_fill(true),
                                    );
                                }
                            });
                            ui.add_space(6.0);
                        }
                    }
                });
            });
    }
}

fn node_card(
    ui: &mut egui::Ui,
    node: &crate::graph::AudioNode,
    accent: Color32,
    text_color: Color32,
    dim_text: Color32,
    ports: &[&crate::graph::AudioPort],
    links: &[crate::graph::AudioLink],
) {
    Frame {
        fill: Color32::from_rgb(0x28, 0x24, 0x30),
        corner_radius: CornerRadius::same(6),
        inner_margin: Margin::symmetric(8, 6),
        ..Default::default()
    }
    .show(ui, |ui| {
        ui.label(egui::RichText::new(&node.name).color(text_color).size(12.0));
        let desc = if node.description.is_empty() {
            &node.name
        } else {
            &node.description
        };
        ui.label(egui::RichText::new(desc).color(dim_text).size(10.0));

        let link_count = links
            .iter()
            .filter(|l| {
                ports.iter().any(|p| {
                    p.id == l.output_port || p.id == l.input_port
                })
            })
            .count();

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("⬤ {}", ports.len()))
                    .color(accent)
                    .size(10.0),
            );
            if link_count > 0 {
                ui.label(
                    egui::RichText::new(format!("🔗 {}", link_count))
                        .color(accent)
                        .size(10.0),
                );
            }
        });
    });
    ui.add_space(4.0);
}
