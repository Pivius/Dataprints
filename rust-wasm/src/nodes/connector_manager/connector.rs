use super::edge::ConnectorEdge;
use super::value::ConnectorValue;
use super::{ConnectorType, Manager};

pub struct Connector {
    index: usize,
    name: String,
    value: Box<dyn ConnectorValue>,
    edge: Option<usize>,
    is_output: bool,
}

impl Connector {
    pub fn new<T: ConnectorValue>(index: usize, name: String, value: T, is_output: bool) -> Connector {
        Connector {
            index,
            name: name,
            value: value.box_self(),
            edge: None,
            is_output: is_output,
        }
    }

    pub fn get_value_type(&self) -> ConnectorType {
        self.value.get_connector_type()
    }

    pub fn get_actual_value(&self) -> Box<dyn ConnectorValue> {
        self.value.clone()
    }

    pub fn get_value(&self, manager: &Manager) -> Box<dyn ConnectorValue> {
        if self.get_is_output() {
            self.value.clone()
        } else {
            let edge = self.get_edge();
            
            if edge.is_none() {
                return self.value.clone();
            }

            let edge = manager.get_edge(edge.unwrap());
            let output = edge.unwrap().output_connector(manager);

            if output.is_some() {
                output.unwrap().get_actual_value()
            } else {
                self.value.clone()
            }
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_is_output(&self) -> bool {
        self.is_output
    }

    pub fn set_value<T: ConnectorValue>(&mut self, value: T) {
        self.value = value.box_self();
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn remove_edge(&mut self) {
        self.edge = None;
    }
    
    pub fn set_edge(&mut self, edge: usize) {
        self.edge = Some(edge);
    }

    pub fn get_edge(&self) -> Option<usize> {
        self.edge
    }
}

impl core::fmt::Debug for Connector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Connector {{ index: {}, name: {}, value: {:?}, edge: {:?}, is_output: {} }}", self.index, self.name, self.value.stringify(), self.edge, self.is_output)
    }
}
