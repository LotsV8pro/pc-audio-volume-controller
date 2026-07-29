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

    let links_output = run_cmd("pw-link", &["-l"])?;
    let output_ports = run_cmd("pw-link", &["-o", "-v"])?;
    let input_ports = run_cmd("pw-link", &["-i", "-v"])?;
    let pw_cli = run_cmd("pw-cli", &["list-objects"])?;

    parse_nodes(&pw_cli, &mut graph)?;
    parse_ports(&output_ports, &mut graph, PortDirection::Output)?;
    parse_ports(&input_ports, &mut graph, PortDirection::Input)?;
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

fn parse_ports(output: &str, graph: &mut AudioGraph, direction: PortDirection) -> Result<()> {
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("Port") || trimmed.starts_with("id") {
            continue;
        }

        let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
        if parts.len() < 2 {
            continue;
        }

        let port_name = parts[1].to_string();
        let node_name = parts[0].to_string();

        let port_id = format!("{}:{}", node_name, port_name);

        for node in graph.nodes.values_mut() {
            if node.name == node_name || format!("{}:{}", node.name, port_name) == port_id {
                node.ports.push(AudioPort {
                    id: port_id.clone(),
                    name: port_name.clone(),
                    node_id: node.id.clone(),
                    direction: direction.clone(),
                });
                break;
            }
        }
    }
    Ok(())
}

fn parse_links(output: &str, graph: &mut AudioGraph) -> Result<()> {
    let mut current_input: Option<String> = None;

    for line in output.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("|- ") {
            current_input = Some(rest.to_string());
        } else if let Some(rest) = trimmed.strip_prefix("  |<- ") {
            if let Some(ref input) = current_input {
                let output = rest.to_string();
                graph.links.push(AudioLink {
                    output_port: output,
                    input_port: input.clone(),
                });
            }
        } else if trimmed.starts_with("  |-> ") {
            if let Some(ref input) = current_input {
                let target = trimmed.trim_start_matches("  |-> ");
                graph.links.push(AudioLink {
                    output_port: input.clone(),
                    input_port: target.to_string(),
                });
            }
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

pub fn set_node_volume(node_name: &str, volume: f32) -> Result<()> {
    let vol = (volume * 100.0) as u32;
    run_cmd(
        "pw-cli",
        &["set-param", node_name, "Props", &format!("{{ volume: {} }}", vol)],
    )
    .ok();
    Ok(())
}
