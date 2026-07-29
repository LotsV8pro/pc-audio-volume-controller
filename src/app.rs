use eframe::egui::{self, Color32, CornerRadius, Frame, Margin, Pos2, Vec2, Id};

use crate::graph::{AudioGraph, AudioNode};
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

    dragging_from_port: Option<String>,
    drag_start_pos: Option<Pos2>,
    hovered_port: Option<String>,

    refresh_timer: f64,
    auto_refresh: bool,

    node_hover: Option<String>,
}

impl Default for LotusAudioApp {
    fn default() -> Self {
        let mut app = Self {
            graph: AudioGraph::new(),
            panel: Panel::Graph,
            error: None,
            dragging_from_port: None,
            drag_start_pos: None,
            hovered_port: None,
            refresh_timer: 0.0,
            auto_refresh: true,
            node_hover: None,
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
                self.error = Some(format!("Failed to fetch graph: {}", e));
            }
        }
    }

    fn try_create_link(&mut self, output: &str, input: &str) {
        if let Err(e) = pipewire::create_link(output, input) {
            self.error = Some(format!("Link failed: {}", e));
        }
        self.refresh();
    }

    fn try_remove_link(&mut self, output: &str, input: &str) {
        if let Err(e) = pipewire::remove_link(output, input) {
            self.error = Some(format!("Unlink failed: {}", e));
        }
        self.refresh();
    }
}

impl eframe::App for LotusAudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let a = Color32::from_rgb(0xC4, 0xA8, 0xE2);
        let a_dim = Color32::from_rgb(0x8C, 0x7A, 0xA6);
        let bg = Color32::from_rgb(0x14, 0x12, 0x18);
        let card = Color32::from_rgb(0x1C, 0x1A, 0x22);
        let card_light = Color32::from_rgb(0x28, 0x24, 0x30);
        let txt = Color32::from_rgb(0xD8, 0xD0, 0xE8);
        let dim = Color32::from_rgb(0x88, 0x78, 0x98);

        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.panel_fill = bg;
        style.visuals.window_fill = card;
        style.visuals.faint_bg_color = card;
        style.visuals.extreme_bg_color = bg;
        style.visuals.widgets.noninteractive.bg_fill = card;
        style.visuals.widgets.inactive.bg_fill = card;
        style.visuals.widgets.active.bg_fill = card_light;
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(0x38, 0x30, 0x48);
        style.visuals.hyperlink_color = a;
        style.visuals.selection.stroke.color = a;
        style.visuals.selection.bg_fill = Color32::from_rgba_premultiplied(0xC4, 0xA8, 0xE2, 40);
        ctx.set_style(style);

        let pointer_pos = ctx.input(|i| i.pointer.hover_pos());
        let pointer_released = ctx.input(|i| i.pointer.any_released());

        if pointer_released {
            let drag_port = self.dragging_from_port.clone();
            let drop_port = self.hovered_port.clone();
            if let (Some(ref dp), Some(ref hp)) = (drag_port, drop_port) {
                if dp != hp {
                    self.try_create_link(dp, hp);
                }
            }
            self.dragging_from_port = None;
            self.drag_start_pos = None;
        }

        let dt = ctx.input(|i| i.unstable_dt) as f64;
        self.refresh_timer += dt;

        if self.auto_refresh && self.refresh_timer >= 2.0 && self.dragging_from_port.is_none() {
            self.refresh();
            self.refresh_timer = 0.0;
        }

        egui::TopBottomPanel::top("top_bar")
            .frame(Frame { fill: bg, inner_margin: Margin::symmetric(16, 8), ..Default::default() })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new("◈ LOTUS AUDIO").color(a).size(20.0));
                    ui.label(egui::RichText::new(format!("{} nodes", self.graph.nodes.len())).color(dim).size(11.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.toggle_value(&mut self.auto_refresh, "Auto-refresh");
                        if ui.button("↻").on_hover_text("Refresh now").clicked() { self.refresh(); }
                        ui.separator();
                        ui.selectable_value(&mut self.panel, Panel::Volumes, "Volumes");
                        ui.selectable_value(&mut self.panel, Panel::Graph, "Graph");
                    });
                });
            });

        if let Some(ref err) = self.error.clone() {
            egui::TopBottomPanel::top("error_bar")
                .frame(Frame { fill: Color32::from_rgb(0x38, 0x10, 0x10), inner_margin: Margin::symmetric(8, 4), ..Default::default() })
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new(err).color(Color32::from_rgb(0xFF, 0x60, 0x60)).size(12.0));
                });
        }

        match self.panel {
            Panel::Graph => self.show_graph(ctx, a, a_dim, card, card_light, txt, dim, pointer_pos),
            Panel::Volumes => self.show_volumes(ctx, a, card, card_light, txt, dim),
        }

        if let Some(ref drag_port) = self.dragging_from_port.clone() {
            if let Some(pos) = pointer_pos {
                let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, Id::new("drag_line")));
                if let Some(start) = self.drag_start_pos {
                    let line_color = if self.hovered_port.is_some() { a } else { a_dim };
                    painter.line_segment([start, pos], (2.0, line_color));
                    painter.circle_filled(pos, 4.0, a);
                    painter.text(
                        pos + Vec2::new(12.0, -8.0),
                        egui::Align2::LEFT_CENTER,
                        &drag_port.split(':').last().unwrap_or(drag_port),
                        egui::FontId::proportional(10.0),
                        dim,
                    );
                }
            }
        }

        ctx.request_repaint();
    }
}

impl LotusAudioApp {
    fn show_graph(
        &mut self, ctx: &egui::Context,
        a: Color32, a_dim: Color32, card: Color32, card_light: Color32,
        txt: Color32, dim: Color32, pointer_pos: Option<Pos2>,
    ) {
        let mut new_hover: Option<String> = None;

        let source_ids: Vec<String> = self.graph.sources().iter().map(|n| n.id.clone()).collect();
        let sink_ids: Vec<String> = self.graph.sinks().iter().map(|n| n.id.clone()).collect();

        let sources_data: Vec<(AudioNode, Vec<crate::graph::AudioPort>, usize)> = source_ids.iter()
            .filter_map(|id| {
                self.graph.nodes.get(id).map(|n| {
                    let ports: Vec<crate::graph::AudioPort> = self.graph.output_ports(id).into_iter().cloned().collect();
                    let lc = self.graph.links.iter()
                        .filter(|l| ports.iter().any(|p| p.id == l.output_port || p.id == l.input_port))
                        .count();
                    (n.clone(), ports, lc)
                })
            })
            .collect();

        let sinks_data: Vec<(AudioNode, Vec<crate::graph::AudioPort>, usize)> = sink_ids.iter()
            .filter_map(|id| {
                self.graph.nodes.get(id).map(|n| {
                    let ports: Vec<crate::graph::AudioPort> = self.graph.input_ports(id).into_iter().cloned().collect();
                    let lc = self.graph.links.iter()
                        .filter(|l| ports.iter().any(|p| p.id == l.output_port || p.id == l.input_port))
                        .count();
                    (n.clone(), ports, lc)
                })
            })
            .collect();

        let links = self.graph.links.clone();

        egui::SidePanel::left("sources_panel")
            .resizable(true).default_width(280.0).min_width(200.0)
            .frame(Frame { fill: card, inner_margin: Margin::symmetric(8, 8), ..Default::default() })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("◈ SOURCES").color(a).size(14.0));
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (node, ports, lc) in &sources_data {
                        self.render_node(ui, node, true, &ports, *lc, a, a_dim, card_light, txt, dim, &mut new_hover, pointer_pos);
                    }
                });
            });

        egui::SidePanel::right("sinks_panel")
            .resizable(true).default_width(280.0).min_width(200.0)
            .frame(Frame { fill: card, inner_margin: Margin::symmetric(8, 8), ..Default::default() })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("◈ SINKS").color(a).size(14.0));
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (node, ports, lc) in &sinks_data {
                        self.render_node(ui, node, false, &ports, *lc, a, a_dim, card_light, txt, dim, &mut new_hover, pointer_pos);
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(Frame { fill: Color32::from_rgb(0x10, 0x0E, 0x14), inner_margin: Margin::symmetric(12, 8), ..Default::default() })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("◈ CONNECTIONS").color(a).size(14.0));
                ui.separator();
                ui.add_space(4.0);

                let hint = if self.dragging_from_port.is_some() {
                    "Drop on a sink port to connect"
                } else {
                    "Drag ⬤ from a source port to a sink port"
                };
                ui.label(egui::RichText::new(hint).color(dim).size(11.0));
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for link in &links {
                        let out_s = link.output_port.split(':').last().unwrap_or(&link.output_port);
                        let in_s = link.input_port.split(':').last().unwrap_or(&link.input_port);
                        let out_p = link.output_port.clone();
                        let in_p = link.input_port.clone();

                        Frame { fill: card_light, corner_radius: CornerRadius::same(6), inner_margin: Margin::symmetric(8, 6), ..Default::default() }
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(out_s).color(a).size(11.0));
                                    ui.label(egui::RichText::new(" → ").color(dim));
                                    ui.label(egui::RichText::new(in_s).color(txt).size(11.0));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("✕").clicked() { self.try_remove_link(&out_p, &in_p); }
                                    });
                                });
                            });
                        ui.add_space(3.0);
                    }
                    if links.is_empty() && self.dragging_from_port.is_none() {
                        ui.label(egui::RichText::new("No connections").color(dim).size(12.0));
                    }
                });
            });

        self.hovered_port = new_hover;
    }

    #[allow(clippy::too_many_arguments)]
    fn render_node(
        &mut self, ui: &mut egui::Ui, node: &AudioNode, is_source: bool,
        ports: &[crate::graph::AudioPort], link_count: usize,
        a: Color32, a_dim: Color32, card_light: Color32, txt: Color32, dim: Color32,
        new_hover: &mut Option<String>, pointer_pos: Option<Pos2>,
    ) {
        let is_hovered = self.node_hover.as_deref() == Some(&node.id);
        let bg = if is_hovered { Color32::from_rgb(0x38, 0x30, 0x48) } else { card_light };

        Frame { fill: bg, corner_radius: CornerRadius::same(6), inner_margin: Margin::symmetric(8, 6), ..Default::default() }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&node.name).color(txt).size(12.0));
                    if link_count > 0 {
                        ui.label(egui::RichText::new(format!("🔗{}", link_count)).color(a).size(10.0));
                    }
                });

                let desc = if node.description.is_empty() { &node.name } else { &node.description };
                ui.label(egui::RichText::new(desc).color(dim).size(10.0));
                ui.add_space(2.0);

                for port in ports {
                    let port_label = port.name.split(':').last().unwrap_or(&port.name);
                    let is_dragging_this = self.dragging_from_port.as_deref() == Some(&port.id);
                    let dot_color = if is_dragging_this { Color32::WHITE } else { a };

                    let dot_size = 8.0;
                    let (rect, response) = ui.allocate_exact_size(Vec2::new(dot_size + 4.0, 14.0), egui::Sense::drag());

                    let dot_center = rect.center();
                    let painter = ui.painter();
                    painter.circle_filled(dot_center, dot_size / 2.0, dot_color);
                    painter.circle_stroke(dot_center, dot_size / 2.0_f32, egui::Stroke::new(1.0_f32, a_dim));

                    let label_pos = Pos2::new(rect.right() + 4.0, rect.center().y);
                    painter.text(label_pos, egui::Align2::LEFT_CENTER, port_label, egui::FontId::proportional(10.0), dim);

                    if response.drag_started() {
                        self.dragging_from_port = Some(port.id.clone());
                        self.drag_start_pos = pointer_pos;
                    }

                    if !is_source && self.dragging_from_port.is_some() {
                        if let Some(pos) = pointer_pos {
                            if response.rect.expand(4.0).contains(pos) {
                                *new_hover = Some(port.id.clone());
                            }
                        }
                    }
                }
            });
        ui.add_space(4.0);
    }

    fn show_volumes(
        &mut self, ctx: &egui::Context,
        a: Color32, card: Color32, _card_light: Color32, txt: Color32, _dim: Color32,
    ) {
        egui::CentralPanel::default()
            .frame(Frame { fill: Color32::from_rgb(0x14, 0x12, 0x18), inner_margin: Margin::symmetric(16, 8), ..Default::default() })
            .show(ctx, |ui| {
                ui.heading(egui::RichText::new("◈ VOLUMES").color(a).size(16.0));
                ui.separator();
                ui.add_space(8.0);

                let ids: Vec<String> = self.graph.nodes.keys().cloned().collect();
                let node_snapshots: Vec<(String, String, f32)> = ids.iter()
                    .filter_map(|id| self.graph.nodes.get(id))
                    .map(|n| (n.id.clone(), n.name.clone(), n.volume))
                    .collect();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (id, name, _vol) in &node_snapshots {
                        Frame { fill: card, corner_radius: CornerRadius::same(8), inner_margin: Margin::symmetric(12, 8), ..Default::default() }
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(name).color(txt).size(13.0));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if let Some(mut_node) = self.graph.nodes.get_mut(id) {
                                            let vol = mut_node.volume;
                                            ui.label(egui::RichText::new(format!("{:.0}%", vol * 100.0)).color(a).size(12.0));
                                        }
                                    });
                                });
                                if let Some(mut_node) = self.graph.nodes.get_mut(id) {
                                    let resp = ui.add(egui::Slider::new(&mut mut_node.volume, 0.0..=1.5).show_value(false).trailing_fill(true));
                                    if resp.drag_stopped() {
                                        let _ = pipewire::set_node_volume(name, mut_node.volume);
                                    }
                                }
                            });
                        ui.add_space(6.0);
                    }
                });
            });
    }
}
