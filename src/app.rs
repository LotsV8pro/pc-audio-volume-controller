use std::collections::HashSet;
use eframe::egui::{self, Color32, CornerRadius, Frame, Margin, Rect, Vec2};

use crate::config::AppConfig;
use crate::graph::AudioGraph;
use crate::pipewire;

pub struct LotusAudioApp {
    graph: AudioGraph,
    error: Option<String>,

    dragging_source: Option<String>,

    refresh_timer: f64,
    auto_refresh: bool,

    show_settings: bool,
    enabled_sinks: HashSet<String>,
    all_sink_ids_snapshot: Vec<(String, String, String)>,

    config: AppConfig,
    config_initialized: bool,
}

impl Default for LotusAudioApp {
    fn default() -> Self {
        let mut app = Self {
            graph: AudioGraph::new(),
            error: None,
            dragging_source: None,
            refresh_timer: 0.0,
            auto_refresh: true,
            show_settings: false,
            enabled_sinks: HashSet::new(),
            all_sink_ids_snapshot: Vec::new(),
            config: crate::config::load(),
            config_initialized: false,
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
        let old_volumes: std::collections::HashMap<String, f32> = self.graph.nodes.iter()
            .map(|(k, v)| (k.clone(), v.volume))
            .collect();

        match pipewire::fetch_graph() {
            Ok(graph) => {
                self.graph = graph;

                if self.config_initialized {
                    for (id, vol) in old_volumes {
                        if let Some(node) = self.graph.nodes.get_mut(&id) {
                            node.volume = vol;
                        }
                    }
                } else {
                    let sink_ids: Vec<String> = self.graph.sinks().iter().map(|n| n.id.clone()).collect();
                    for id in &sink_ids {
                        if let Some((vol, _)) = pipewire::get_node_volume(id) {
                            if let Some(node) = self.graph.nodes.get_mut(id) {
                                node.volume = vol;
                            }
                        }
                    }
                }
                self.error = None;

                let current: Vec<(String, String, String)> = self.graph.sinks().iter()
                    .map(|n| {
                        let display = if n.description.is_empty() || n.description == n.name {
                            n.name.clone()
                        } else {
                            n.description.clone()
                        };
                        (n.id.clone(), n.name.clone(), display)
                    })
                    .collect();

                let current_ids: HashSet<String> = current.iter().map(|(id, _, _)| id.clone()).collect();
                let prev_ids: HashSet<String> = self.all_sink_ids_snapshot.iter().map(|(id, _, _)| id.clone()).collect();
                let truly_new: HashSet<String> = current_ids.difference(&prev_ids).cloned().collect();

                if !self.config_initialized {
                    for (_, name, _) in &current {
                        if self.config.is_sink_enabled(name) {
                            self.enabled_sinks.insert(name.clone());
                        }
                    }
                    self.config_initialized = true;

                    for node in self.graph.nodes.values() {
                        if let Some(vol) = self.config.volumes.get(&node.name) {
                            let _ = pipewire::set_node_volume(&node.id, *vol);
                        }
                    }
                } else if !truly_new.is_empty() && self.config.enabled_sinks.is_none() {
                    for id in &truly_new {
                        let name = current.iter()
                            .find(|(i, _, _)| i == id)
                            .map(|(_, n, _)| n.clone())
                            .unwrap_or_default();
                        self.enabled_sinks.insert(name);
                    }
                }

                if self.all_sink_ids_snapshot != current {
                    self.all_sink_ids_snapshot = current;
                }
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

    fn sink_display_name(&self, id: &str, fallback: &str) -> String {
        self.graph.nodes.get(id)
            .map(|n| if n.description.is_empty() || n.description == n.name {
                n.name.clone()
            } else {
                n.description.clone()
            })
            .unwrap_or_else(|| fallback.to_string())
    }

    fn source_display_name(&self, id: &str, fallback: &str) -> String {
        self.graph.nodes.get(id)
            .map(|n| if n.description.is_empty() || n.description == n.name {
                n.name.clone()
            } else {
                n.description.clone()
            })
            .unwrap_or_else(|| fallback.to_string())
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
                        if ui.button("⚙").on_hover_text("Configure zones").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                        ui.toggle_value(&mut self.auto_refresh, "Auto-refresh");
                        if ui.button("↻").on_hover_text("Refresh now").clicked() { self.refresh(); }
                    });
                });
            });

        if self.show_settings {
            egui::Window::new("Zone Settings")
                .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-8.0, 48.0))
                .title_bar(true)
                .resizable(true)
                .default_width(240.0)
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new("Show zones:").color(a).size(13.0));
                    ui.separator();
                    egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                        let items = self.all_sink_ids_snapshot.clone();
                        let mut changed = false;
                        for (_, name, display_name) in &items {
                            let mut enabled = self.enabled_sinks.contains(name);
                            let before = enabled;
                            ui.checkbox(&mut enabled, display_name);
                            if enabled != before {
                                if enabled {
                                    self.enabled_sinks.insert(name.clone());
                                } else {
                                    self.enabled_sinks.remove(name);
                                }
                                self.config.set_sink_enabled(name, enabled);
                                changed = true;
                            }
                        }
                        if changed {
                            crate::config::save(&self.config);
                        }
                    });
                    ui.add_space(8.0);
                    if ui.button("Reset").clicked() {
                        for (_, name, _) in &self.all_sink_ids_snapshot {
                            self.enabled_sinks.insert(name.clone());
                        }
                        self.config.enabled_sinks = Some(
                            self.all_sink_ids_snapshot.iter().map(|(_, n, _)| n.clone()).collect(),
                        );
                        crate::config::save(&self.config);
                    }
                });
        }

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
        let sink_ids: Vec<String> = self.graph.sinks().iter()
            .filter(|n| self.enabled_sinks.contains(&n.name))
            .map(|n| n.id.clone())
            .collect();

        let zone_data: Vec<(String, String, Vec<(String, String)>)> = sink_ids.iter()
            .filter_map(|id| self.graph.nodes.get(id))
            .map(|sink| {
                let sources = self.graph.sources_for_sink(&sink.id);
                let pills: Vec<(String, String)> = sources.iter()
                    .map(|s| (s.id.clone(), s.name.clone()))
                    .collect();
                (sink.id.clone(), sink.name.clone(), pills)
            })
            .collect();

        let pointer_pos = ctx.input(|i| i.pointer.hover_pos());
        let pointer_released = ctx.input(|i| i.pointer.any_released());

        egui::CentralPanel::default()
            .frame(Frame { fill: bg, inner_margin: Margin::symmetric(8, 8), ..Default::default() })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("◈ AUDIO ROUTER").color(a).size(14.0));
                    ui.label(egui::RichText::new(format!("{} zones", zone_data.len())).color(dim).size(11.0));
                    if self.dragging_source.is_some() {
                        ui.label(egui::RichText::new("drag to a zone").color(a_dim).size(11.0));
                    }
                });
                ui.separator();
                ui.add_space(4.0);

                let avail = ui.available_width();
                let num = zone_data.len().max(1);
                let raw_col = (avail - (num as f32 * 4.0) - 8.0) / num as f32;
                let col_width = raw_col.clamp(120.0, 160.0);
                let needs_scroll = raw_col < 120.0;

                let mut scroll = egui::ScrollArea::horizontal().auto_shrink(false);
                if !needs_scroll {
                    scroll = scroll.max_width(avail);
                }
                scroll.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let mut zone_rects: Vec<(String, Rect)> = Vec::new();

                        for (sink_id, sink_name, pills) in &zone_data {
                            let inner = Frame {
                                fill: card,
                                corner_radius: CornerRadius::same(10),
                                stroke: egui::Stroke::new(0.0_f32, a_dim),
                                inner_margin: Margin::symmetric(10, 8),
                                ..Default::default()
                            }.show(ui, |ui| {
                                ui.set_width(col_width - 20.0);

                                ui.vertical(|ui| {
                                    let display_name = self.sink_display_name(sink_id, sink_name);

                                    ui.label(egui::RichText::new("◆").color(a).size(12.0));
                                    ui.label(egui::RichText::new(display_name).color(txt).size(14.0).strong());
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
                                            let vol = mut_node.volume;
                                            let _ = pipewire::set_node_volume(sink_id, vol);
                                            self.config.volumes.insert(sink_name.clone(), vol);
                                            crate::config::save(&self.config);
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
                                            let is_dragging = self.dragging_source.as_deref() == Some(source_id);
                                            let pill_fill = if is_dragging {
                                                Color32::from_rgb(0x4C, 0x3C, 0x5C)
                                            } else {
                                                card_light
                                            };

                                            let desired_size = Vec2::new(col_width - 20.0, 26.0);
                                            let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

                                            let p = ui.painter();
                                            p.rect_filled(rect, 6.0, pill_fill);
                                            if is_dragging {
                                                p.rect_stroke(rect, 6.0, egui::Stroke::new(1.5_f32, a), egui::StrokeKind::Outside);
                                            } else if response.hovered() {
                                                p.rect_stroke(rect, 6.0, egui::Stroke::new(1.0_f32, a_dim), egui::StrokeKind::Outside);
                                            }
                                            let pill_label = self.source_display_name(source_id, source_name);
                                            p.text(
                                                egui::Pos2::new(rect.left() + 8.0, rect.center().y),
                                                egui::Align2::LEFT_CENTER,
                                                pill_label,
                                                egui::FontId::proportional(11.0),
                                                txt,
                                            );

                                            if response.drag_started() {
                                                self.dragging_source = Some(source_id.clone());
                                            }

                                            ui.add_space(1.0);
                                        }
                                    }
                                });
                            });

                            zone_rects.push((sink_id.clone(), inner.response.rect));
                            ui.add_space(4.0);
                        }

                        // Process drop
                        if let Some(pos) = pointer_pos {
                            if pointer_released && self.dragging_source.is_some() {
                                for (sid, rect) in &zone_rects {
                                    if rect.contains(pos) {
                                        if let Some(ref src) = self.dragging_source.clone() {
                                            let already_here = zone_data.iter()
                                                .find(|(zid, _, _)| zid == sid)
                                                .map_or(false, |(_, _, pills)| pills.iter().any(|(id, _)| id == src));
                                            if !already_here {
                                                self.try_relink_source(src, sid);
                                            }
                                        }
                                        break;
                                    }
                                }
                                self.dragging_source = None;
                            }
                        }
                    });

                    let all_linked_source_ids: HashSet<String> = zone_data.iter()
                        .flat_map(|(_, _, pills)| pills.iter().map(|(id, _)| id.clone()))
                        .collect();

                    let all_source_ids: Vec<String> = self.graph.sources().iter().map(|n| n.id.clone()).collect();
                    let disconnected: Vec<(String, String)> = all_source_ids.iter()
                        .filter_map(|id| {
                            if all_linked_source_ids.contains(id) { return None; }
                            self.graph.nodes.get(id).map(|n| (id.clone(), n.name.clone()))
                        })
                        .collect();

                    if !disconnected.is_empty() {
                        ui.add_space(12.0);
                        Frame {
                            fill: card,
                            corner_radius: CornerRadius::same(8),
                            inner_margin: Margin::symmetric(10, 8),
                            ..Default::default()
                        }.show(ui, |ui| {
                            ui.set_max_width(160.0);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("◈ DISCONNECTED").color(a_dim).size(12.0));
                                ui.separator();
                                ui.add_space(4.0);
                                for (source_id, source_name) in &disconnected {
                                    let is_dragging = self.dragging_source.as_deref() == Some(source_id);
                                    let pill_fill = if is_dragging {
                                        Color32::from_rgb(0x4C, 0x3C, 0x5C)
                                    } else {
                                        card_light
                                    };
                                    let desired_size = Vec2::new(140.0, 26.0);
                                    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
                                    let p = ui.painter();
                                    p.rect_filled(rect, 6.0, pill_fill);
                                    if is_dragging {
                                        p.rect_stroke(rect, 6.0, egui::Stroke::new(1.5_f32, a), egui::StrokeKind::Outside);
                                    }
                                    let pill_label = self.source_display_name(source_id, source_name);
                                    p.text(
                                        egui::Pos2::new(rect.left() + 8.0, rect.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        pill_label,
                                        egui::FontId::proportional(11.0),
                                        txt,
                                    );
                                    if response.drag_started() {
                                        self.dragging_source = Some(source_id.clone());
                                    }
                                    ui.add_space(1.0);
                                }
                            });
                        });
                    }
                });
            });
    }
}
