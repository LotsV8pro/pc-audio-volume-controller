use std::process::Command;

use anyhow::{Context, Result};

use crate::graph::{AudioGraph, AudioLink, AudioNode, AudioPort, PortDirection};

fn run_cmd(program: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to run {}", program))?;
    if !out.status.success() {
        anyhow::bail!("{} failed: {}", program, String::from_utf8_lossy(&out.stderr));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn fetch_graph() -> Result<AudioGraph> {
    let mut graph = AudioGraph::new();

    let pw_cli = run_cmd("pw-cli", &["list-objects"])?;
    parse_nodes(&pw_cli, &mut graph)?;

    let output_ports = run_cmd("pw-link", &["-o"])?;
    let input_ports = run_cmd("pw-link", &["-i"])?;
    add_ports(&output_ports, &mut graph, PortDirection::Output);
    add_ports(&input_ports, &mut graph, PortDirection::Input);

    let links_output = run_cmd("pw-link", &["-l"])?;
    parse_links(&links_output, &mut graph)?;

    Ok(graph)
}

fn parse_nodes(output: &str, graph: &mut AudioGraph) -> Result<()> {
    let mut current_id: Option<String> = None;
    let mut current_name: Option<String> = None;
    let mut current_class: Option<String> = None;

    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("id ") {
            if let Some(id_str) = rest.split(',').next() {
                if let Some(nid) = current_id.take() {
                    if let (Some(name), Some(class)) = (current_name.take(), current_class.take()) {
                        if name != "pipewire-0" {
                            graph.nodes.entry(nid.clone()).or_insert_with(|| AudioNode {
                                id: nid,
                                name: name.clone(),
                                description: name,
                                media_class: class,
                                ports: Vec::new(),
                                volume: 1.0,
                                muted: false,
                            });
                        }
                    }
                }
                current_id = Some(id_str.trim().to_string());
            }
        } else if let Some(val) = trimmed.strip_prefix("node.name = \"") {
            current_name = Some(val.trim_end_matches('"').to_string());
        } else if let Some(val) = trimmed.strip_prefix("media.class = \"") {
            current_class = Some(val.trim_end_matches('"').to_string());
        } else if let Some(val) = trimmed.strip_prefix("node.description = \"") {
            let desc = val.trim_end_matches('"').to_string();
            if let Some(ref id) = current_id {
                if let Some(node) = graph.nodes.get_mut(id) {
                    node.description = desc;
                }
            }
        }
    }

    if let (Some(id), Some(name), Some(class)) = (current_id, current_name, current_class) {
        if name != "pipewire-0" {
            graph.nodes.entry(id.clone()).or_insert_with(|| AudioNode {
                id,
                name,
                description: String::new(),
                media_class: class,
                ports: Vec::new(),
                volume: 1.0,
                muted: false,
            });
        }
    }

    Ok(())
}

fn add_ports(output: &str, graph: &mut AudioGraph, direction: PortDirection) {
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (node_name, port_name) = match trimmed.split_once(':') {
            Some((n, p)) => (n, p),
            None => continue,
        };

        let port_id = trimmed.to_string();

        if let Some(node) = graph.nodes.values_mut().find(|n| n.name == node_name) {
            if !node.ports.iter().any(|p| p.id == port_id) {
                node.ports.push(AudioPort {
                    id: port_id,
                    name: port_name.to_string(),
                    node_id: node.id.clone(),
                    direction: direction.clone(),
                });
            }
        }
    }
}

fn parse_links(output: &str, graph: &mut AudioGraph) -> Result<()> {
    let mut current_port: Option<String> = None;

    for line in output.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("|-> ") {
            if let Some(ref port) = current_port {
                graph.links.push(AudioLink {
                    output_port: port.clone(),
                    input_port: rest.to_string(),
                });
            }
        } else if let Some(rest) = trimmed.strip_prefix("|<- ") {
            if let Some(ref port) = current_port {
                graph.links.push(AudioLink {
                    output_port: rest.to_string(),
                    input_port: port.clone(),
                });
            }
        } else if !trimmed.is_empty() && !trimmed.starts_with('|') {
            current_port = Some(trimmed.to_string());
        }
    }
    Ok(())
}

pub fn create_link(output: &str, input: &str) -> Result<()> {
    run_cmd("pw-link", &[output, input]).context("failed to create link")?;
    Ok(())
}

pub fn remove_link(output: &str, input: &str) -> Result<()> {
    run_cmd("pw-link", &["-d", output, input]).context("failed to remove link")?;
    Ok(())
}

pub fn get_node_volume(node_id: &str) -> Option<(f32, bool)> {
    let out = run_cmd("wpctl", &["get-volume", node_id]).ok()?;
    let mut volume: Option<f32> = None;
    let mut muted = false;
    let mut toks = out.split_whitespace().peekable();
    while let Some(tok) = toks.next() {
        if tok == "Volume:" {
            if let Some(v) = toks.next() {
                volume = v.parse::<f32>().ok();
            }
        } else if tok == "[MUTED]" {
            muted = true;
        }
    }
    volume.map(|v| (v, muted))
}

pub fn set_node_volume(node_id: &str, volume: f32) -> Result<()> {
    let vol = format!("{:.3}", volume);
    run_cmd("wpctl", &["set-volume", node_id, &vol]).context("failed to set volume")?;
    Ok(())
}

fn channel_name(port_name: &str) -> &str {
    port_name.rsplit_once('_').map(|(_, c)| c).unwrap_or(port_name)
}

pub fn relink_source(source_name: &str, target_sink_name: &str) -> Result<()> {
    let graph = fetch_graph()?;

    let source_node = graph.nodes.values().find(|n| n.name == source_name)
        .ok_or_else(|| anyhow::anyhow!("source '{}' not found", source_name))?;
    let target_node = graph.nodes.values().find(|n| n.name == target_sink_name)
        .ok_or_else(|| anyhow::anyhow!("sink '{}' not found", target_sink_name))?;

    let output_ports: Vec<&str> = source_node.ports.iter()
        .filter(|p| p.direction == PortDirection::Output)
        .map(|p| p.name.as_str())
        .collect();

    let input_port_names: Vec<&str> = target_node.ports.iter()
        .filter(|p| p.direction == PortDirection::Input)
        .map(|p| p.name.as_str())
        .collect();

    let full_output_ports: Vec<String> = source_node.ports.iter()
        .filter(|p| p.direction == PortDirection::Output)
        .map(|p| p.id.clone())
        .collect();

    for link in &graph.links {
        if full_output_ports.contains(&link.output_port) {
            let _ = run_cmd("pw-link", &["-d", &link.output_port, &link.input_port]);
        }
    }

    for op in &output_ports {
        let ch = channel_name(op);
        if let Some(ip_name) = input_port_names.iter().find(|ip| channel_name(ip) == ch) {
            let src_port = format!("{}:{}", source_name, op);
            let dst_port = format!("{}:{}", target_sink_name, ip_name);
            let _ = run_cmd("pw-link", &[&src_port, &dst_port]);
        }
    }

    Ok(())
}
