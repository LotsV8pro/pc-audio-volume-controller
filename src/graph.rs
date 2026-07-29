use std::collections::HashMap;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AudioPort {
    pub id: String,
    pub name: String,
    pub node_id: String,
    pub direction: PortDirection,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PortDirection {
    Output,
    Input,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AudioNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub media_class: String,
    pub ports: Vec<AudioPort>,
    pub volume: f32,
    pub muted: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AudioLink {
    pub output_port: String,
    pub input_port: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AudioGraph {
    pub nodes: HashMap<String, AudioNode>,
    pub links: Vec<AudioLink>,
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: Vec::new(),
        }
    }

    pub fn sources(&self) -> Vec<&AudioNode> {
        let mut v: Vec<_> = self
            .nodes
            .values()
            .filter(|n| {
                n.media_class == "Audio/Source"
                    || n.media_class == "Stream/Output/Audio"
            })
            .collect();
        v.sort_by(|a, b| a.name.cmp(&b.name));
        v
    }

    pub fn sinks(&self) -> Vec<&AudioNode> {
        let mut v: Vec<_> = self
            .nodes
            .values()
            .filter(|n| {
                n.media_class == "Audio/Sink" || n.media_class == "Audio/Sink/Internal"
            })
            .collect();
        v.sort_by(|a, b| a.name.cmp(&b.name));
        v
    }

    pub fn output_ports(&self, node_id: &str) -> Vec<&AudioPort> {
        self.nodes
            .get(node_id)
            .map(|n| {
                n.ports
                    .iter()
                    .filter(|p| p.direction == PortDirection::Output)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn input_ports(&self, node_id: &str) -> Vec<&AudioPort> {
        self.nodes
            .get(node_id)
            .map(|n| {
                n.ports
                    .iter()
                    .filter(|p| p.direction == PortDirection::Input)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn links_for_port(&self, port_id: &str) -> Vec<&AudioLink> {
        self.links
            .iter()
            .filter(|l| l.output_port == port_id || l.input_port == port_id)
            .collect()
    }

    pub fn sources_for_sink(&self, sink_id: &str) -> Vec<&AudioNode> {
        let sink = match self.nodes.get(sink_id) {
            Some(n) => n,
            None => return Vec::new(),
        };
        let input_port_ids: Vec<&str> = sink.ports.iter()
            .filter(|p| p.direction == PortDirection::Input)
            .map(|p| p.id.as_str())
            .collect();

        let mut source_ids: Vec<String> = Vec::new();
        for link in &self.links {
            if input_port_ids.contains(&link.input_port.as_str()) {
                if let Some((node_name, _)) = link.output_port.split_once(':') {
                    let candidate = self.nodes.values().find(|n| n.name == node_name);
                    if let Some(c) = candidate {
                        if !source_ids.contains(&c.id) {
                            source_ids.push(c.id.clone());
                        }
                    }
                }
            }
        }

        source_ids.iter().filter_map(|id| self.nodes.get(id)).collect()
    }

    pub fn sink_for_source(&self, source_id: &str) -> Option<&AudioNode> {
        let source = self.nodes.get(source_id)?;
        let output_port_ids: Vec<&str> = source.ports.iter()
            .filter(|p| p.direction == PortDirection::Output)
            .map(|p| p.id.as_str())
            .collect();

        for link in &self.links {
            if output_port_ids.contains(&link.output_port.as_str()) {
                if let Some((node_name, _)) = link.input_port.split_once(':') {
                    return self.nodes.values().find(|n| n.name == node_name);
                }
            }
        }
        None
    }

    pub fn source_output_ports(&self, source_id: &str) -> Vec<&AudioPort> {
        self.nodes.get(source_id)
            .map(|n| n.ports.iter()
                .filter(|p| p.direction == PortDirection::Output)
                .collect())
            .unwrap_or_default()
    }

    pub fn sink_input_ports(&self, sink_id: &str) -> Vec<&AudioPort> {
        self.nodes.get(sink_id)
            .map(|n| n.ports.iter()
                .filter(|p| p.direction == PortDirection::Input)
                .collect())
            .unwrap_or_default()
    }
}
