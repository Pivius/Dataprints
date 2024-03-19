mod test;
pub mod edge;
pub mod value;
pub mod connector;
use self::connector::Connector;
use self::value::ConnectorValue;
use self::value::ConnectorType;
use self::edge::ConnectorEdge;

pub struct Manager {
    connectors: Vec<Connector>,
    edges: Vec<ConnectorEdge>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            connectors: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn set_connector_value<T: ConnectorValue>(&mut self, index: usize, value: T) {
        let connector = self.get_connector(index);

        if connector.is_none() {
            return;
        }

        let connector = connector.unwrap();

        if connector.get_value_type() != value.get_connector_type() {
            let edge = self.get_connector_edge(index);

            if let Some(edge) = edge {
                let edge_index = edge.get_index();
                self.delete_edge(edge_index)
            }
        }

        self.connectors[index].set_value(value);
    }

    pub fn get_connector(&self, index: usize) -> Option<&Connector> {
        self.connectors.get(index)
    }

    pub fn get_connector_mut(&mut self, index: usize) -> Option<&mut Connector> {
        self.connectors.get_mut(index)
    }

    pub fn get_two_connectors_mut(&mut self, index1: usize, index2: usize) -> (&mut Connector, &mut Connector) {
        let (left, right) = self.connectors.split_at_mut(index1 + 1);
        (&mut left[index1], &mut right[index2 - index1 - 1])
    }

    pub fn get_edge(&self, index: usize) -> Option<&ConnectorEdge> {
        self.edges.get(index)
    }

    pub fn get_edge_mut(&mut self, index: usize) -> Option<&mut ConnectorEdge> {
        self.edges.get_mut(index)
    }

    pub fn get_connector_edge(&self, index: usize) -> Option<&ConnectorEdge> {
        for edge in &self.edges {
            let is_edge = edge.has_connection(index);
            
            if is_edge {
                return Some(edge);
            }
        }

        None
    }

    pub fn get_connector_edge_mut(&mut self, index: usize) -> Option<&mut ConnectorEdge> {
        for edge in &mut self.edges {
            let is_edge = edge.has_connection(index);

            if is_edge {
                return Some(edge);
            }
        }

        None
    }

    pub fn add_connector<T: ConnectorValue>(&mut self, name: String, value: T, is_output: bool) -> usize {
        let index = self.connectors.len();
        self.connectors.push(Connector::new(index, name, value, is_output));
        index
    }

    pub fn delete_connector(&mut self, index: usize) {
        // delete edge
        let edge = self.get_connector_edge(index);

        if let Some(edge) = edge {
            let edge_index = edge.get_index();
            self.delete_edge(edge_index);
        }

        self.connectors.remove(index);

        for connector in &mut self.connectors {
            if connector.get_index() > index {
                connector.set_index(index - 1);
            }
        }
    }

    pub fn add_edge(&mut self, input: Option<usize>, output: Option<usize>) -> usize {
        let index = self.edges.len();

        self.edges.push(ConnectorEdge::new(index));
        self.edges[index].connect(input, output);
        
        if input.is_some() {
            self.get_connector_mut(input.unwrap()).unwrap().set_edge(index);
        }

        if output.is_some() {
            self.get_connector_mut(output.unwrap()).unwrap().set_edge(index);
        }

        index
    }

    pub fn delete_edge(&mut self, index: usize) {
        let edge = self.get_edge(index).unwrap();
        let input = edge.get_input();
        let output = edge.get_output();

        if input.is_some() {
            self.get_connector_mut(input.unwrap()).unwrap().remove_edge();
        }

        if output.is_some() {
            self.get_connector_mut(output.unwrap()).unwrap().remove_edge();
        }

        self.edges.remove(index);

        for edge in &mut self.edges {
            if edge.get_index() > index {
                edge.set_index(index - 1);
            }
        }
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        let from_connector = self.get_connector(from);
        let to_connector = self.get_connector(to);
        
        if from_connector.is_none() || to_connector.is_none() {
            return;
        }
        
        let from_connector = from_connector.unwrap();
        let to_connector = to_connector.unwrap();

        if from_connector.get_is_output() == to_connector.get_is_output() {
            return;
        }

        let (input, output) = if from_connector.get_is_output() {
            (to_connector, from_connector)
        } else {
            (from_connector, to_connector)
        };

        let from_edge = self.get_connector_edge(from).map(|edge| edge.get_index());
        let to_edge = self.get_connector_edge(to).map(|edge| edge.get_index());
        let input = Some(input.get_index());
        let output = Some(output.get_index());

        match (from_edge, to_edge) {
            (Some(from), Some(to)) => {
                if from != to {
                    self.delete_edge(from);
                    self.get_edge_mut(to).unwrap().connect(input, output);
                }
            },
            (Some(from), None) => {
                self.get_edge_mut(from).unwrap().connect(input, output);
            },
            (None, Some(to)) => {
                self.get_edge_mut(to).unwrap().connect(input, output);
            },
            (None, None) => {
                self.add_edge(input, output);
            },
        }
    }

    pub fn disconnect(&mut self, connector: usize) {
        let edge = self.get_connector_edge(connector);

        if let Some(edge) = edge {
            let edge_index = edge.get_index();
            self.delete_edge(edge_index);
        }
    }
}

