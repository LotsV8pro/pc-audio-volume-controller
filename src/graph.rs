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
                n.media_class == "Audio/Sink"
                    || n.media_class == "Audio/Sink/Internal"
                    || n.media_class == "Stream/Input/Audio"
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
}
