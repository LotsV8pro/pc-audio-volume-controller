use eframe::egui::{self, Color32, CornerRadius, Frame, Margin, Vec2};

use crate::graph::AudioGraph;
use crate::pipewire;

pub struct LotusAudioApp {
    graph: AudioGraph,
    error: Option<String>,

    dragging_source: Option<String>,
    drag_hover_zone: Option<String>,

    refresh_timer: f64,
    auto_refresh: bool,
}

impl Default for LotusAudioApp {
    fn default() -> Self {
        let mut app = Self {
            graph: AudioGraph::new(),
            error: None,
            dragging_source: None,
            drag_hover_zone: None,
            refresh_timer: 0.0,
            auto_refresh: true,
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

    fn try_relink_source(&mut self, source_id: &str, target_sink_id: &str) {
        let source_name = self.graph.nodes.get(source_id).map(|n| n.name.clone());
        let sink_name = self.graph.nodes.get(target_sink_id).map(|n| n.name.clone());
        if let (Some(sn), Some(tn)) = (source_name, sink_name) {
            if let Err(e) = pipewire::relink_source(&sn, &tn) {
                self.error = Some(format!("Relink failed: {}", e));
            }
            self.refresh();
        }
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

        let pointer_released = ctx.input(|i| i.pointer.any_released());

        if pointer_released {
            if let (Some(src), Some(zone)) = (self.dragging_source.clone(), self.drag_hover_zone.clone()) {
                if src != zone {
                    self.try_relink_source(&src, &zone);
                }
            }
            self.dragging_source = None;
            self.drag_hover_zone = None;
        }

        let dt = ctx.input(|i| i.unstable_dt) as f64;
        self.refresh_timer += dt;

        if self.auto_refresh && self.refresh_timer >= 2.0 && self.dragging_source.is_none() {
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

        self.show_zones(ctx, a, a_dim, bg, card, card_light, txt, dim);

        ctx.request_repaint();
    }
}

impl LotusAudioApp {
    fn show_zones(
        &mut self, ctx: &egui::Context,
        a: Color32, a_dim: Color32, bg: Color32, card: Color32, card_light: Color32,
        txt: Color32, dim: Color32,
    ) {
        let sink_ids: Vec<String> = self.graph.sinks().iter().map(|n| n.id.clone()).collect();

        let zone_data: Vec<(String, String, f32, Vec<(String, String)>)> = sink_ids.iter()
            .filter_map(|id| self.graph.nodes.get(id))
            .map(|sink| {
                let sources = self.graph.sources_for_sink(&sink.id);
                let pills: Vec<(String, String)> = sources.iter()
                    .map(|s| (s.id.clone(), s.name.clone()))
                    .collect();
                (sink.id.clone(), sink.name.clone(), sink.volume, pills)
            })
            .collect();

        let pointer_pos = ctx.input(|i| i.pointer.hover_pos());

        egui::CentralPanel::default()
            .frame(Frame { fill: bg, inner_margin: Margin::symmetric(8, 8), ..Default::default() })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("◈ AUDIO ROUTER").color(a).size(14.0));
                ui.separator();
                ui.add_space(4.0);

                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let mut new_hover: Option<String> = None;

                        let col_width = 180.0;
                        let col_height = ui.available_height();

                        for (sink_id, sink_name, volume, pills) in &zone_data {
                            let is_hover_zone = self.dragging_source.is_some()
                                && self.drag_hover_zone.as_deref() == Some(sink_id);

                            let zone_fill = if is_hover_zone {
                                Color32::from_rgb(0x38, 0x20, 0x48)
                            } else {
                                card
                            };

                            let frame = Frame {
                                fill: zone_fill,
                                corner_radius: CornerRadius::same(10),
                                stroke: egui::Stroke::new(if is_hover_zone { 1.5_f32 } else { 0.0_f32 }, a_dim),
                                inner_margin: Margin::symmetric(10, 8),
                                ..Default::default()
                            };

                            ui.allocate_space(Vec2::new(col_width, col_height));

                            frame.show(ui, |ui| {
                                ui.set_min_width(col_width - 20.0);

                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("◆").color(a).size(12.0));
                                    ui.label(egui::RichText::new(sink_name).color(txt).size(14.0).strong());
                                    ui.add_space(2.0);

                                    if let Some(mut_node) = self.graph.nodes.get_mut(sink_id) {
                                        ui.set_min_height(60.0);
                                        let resp = ui.add(
                                            egui::Slider::new(&mut mut_node.volume, 0.0..=1.5)
                                                .show_value(false)
                                                .trailing_fill(true)
                                                .vertical()
                                        );
                                        if resp.drag_stopped() {
                                            let _ = pipewire::set_node_volume(sink_name, mut_node.volume);
                                        }
                                        let vol_pct = (mut_node.volume * 100.0) as u32;
                                        ui.label(egui::RichText::new(format!("{}%", vol_pct)).color(a_dim).size(10.0));
                                    }

                                    ui.add_space(6.0);
                                    ui.separator();
                                    ui.add_space(4.0);

                                    if pills.is_empty() {
                                        ui.label(egui::RichText::new("—").color(dim).size(11.0));
                                    } else {
                                        for (source_id, source_name) in pills {
                                            let pill_hover = self.dragging_source.as_deref() == Some(source_id);
                                            let pill_fill = if pill_hover {
                                                Color32::from_rgb(0x4C, 0x3C, 0x5C)
                                            } else {
                                                card_light
                                            };

                                            let desired_size = Vec2::new(col_width - 20.0, 26.0);
                                            let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

                                            let p = ui.painter();
                                            p.rect_filled(rect, 6.0, pill_fill);
                                            if response.hovered() || pill_hover {
                                                p.rect_stroke(rect, 6.0, egui::Stroke::new(1.0_f32, a_dim), egui::StrokeKind::Outside);
                                            }
                                            p.text(
                                                egui::Pos2::new(rect.left() + 8.0, rect.center().y),
                                                egui::Align2::LEFT_CENTER,
                                                source_name,
                                                egui::FontId::proportional(11.0),
                                                txt,
                                            );

                                            if response.drag_started() {
                                                self.dragging_source = Some(source_id.clone());
                                            }

                                            ui.add_space(4.0);
                                        }
                                    }

                                    if let Some(pos) = pointer_pos {
                                        if ui.min_rect().contains(pos) && self.dragging_source.is_some() {
                                            new_hover = Some(sink_id.clone());
                                        }
                                    }
                                });
                            });
                            ui.add_space(8.0);
                        }

                        self.drag_hover_zone = new_hover;
                    });
                });
            });
    }
}
