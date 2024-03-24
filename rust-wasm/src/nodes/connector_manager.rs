mod test;
pub mod edge;
pub mod value;
pub mod connector;
use crate::helper::types::Null;
use self::connector::Connector;
use self::value::ConnectorType;
use self::edge::ConnectorEdge;

/// Contains vector of all the connectors.\
/// Used to create, delete, connect and disconnect connectors.\
/// 
/// Input connector is a connector that can have only one connection and has no value.\
/// Output connector is a connector that can have multiple connections and can have value.\
/// 
/// 
/// 
/// # Example
/// ```
/// use crate::nodes::connector_manager::Manager;
/// use crate::nodes::connector_manager::value::{ConnectorType, ConnectorTypeId};
/// 
/// let mut manager = Manager::new();
/// 
/// // Create connectors
/// let input = manager.add_connector("connector1".to_string(), 0, ConnectorTypeId::INT, false);
/// let output = manager.add_connector("connector2".to_string(), 5, ConnectorTypeId::INT, true);
/// 
/// // Connect them
/// manager.connect(input, output);
/// 
/// // Input now has value of output
/// let value = manager.get_value(input);
/// 
/// manager.set_value(output, value);
pub struct Manager {
    connectors: Vec<Connector>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            connectors: Vec::new(),
        }
    }

    pub fn set_value<T>(&mut self, index: usize, value: T) 
    where
        T: Into<ConnectorType>
    {
        let connector = self.get_connector(index);

        if connector.is_none() {
            return;
        }

        let value: ConnectorType = value.into();

        self.connectors[index].set_value(value);
    }

    pub fn get_value(&self, index: usize) -> ConnectorType {
        let connector = self.get_connector(index);

        match connector {
            Some(connector) => {
                match connector {
                    Connector::Input(_, connected, connector_type) => {
                        match connected {
                            Some(connected) => self.get_connector(*connected).unwrap().get_value().get_variant(connector_type.clone()),
                            None => ConnectorType::new(0),
                        }
                    },
                    Connector::Output(_, _, value, _) => value.clone(),
                }
            },
            None => ConnectorType::Null(Null::new()),
        }
    }

    pub fn get_connector(&self, index: usize) -> Option<&Connector> {
        self.connectors.get(index)
    }

    pub fn get_connector_mut(&mut self, index: usize) -> Option<&mut Connector> {
        self.connectors.get_mut(index)
    }

    pub fn get_two_connectors(&self, index1: usize, index2: usize) -> (&Connector, &Connector) {
        let (index1, index2) = if index1 < index2 {
            (index1, index2)
        } else {
            (index2, index1)
        };

        let (left, right) = self.connectors.split_at(index1 + 1);
        (&left[index1], &right[index2 - index1 - 1])
    }

    pub fn get_input_output_mut(&mut self, index1: usize, index2: usize) -> (&mut Connector, &mut Connector) {
        let (index1, index2) = if index1 < index2 {
            (index1, index2)
        } else {
            (index2, index1)
        };

        let (left, right) = self.connectors.split_at_mut(index1 + 1);
        let (input, output) = (&mut left[index1], &mut right[index2 - index1 - 1]);

        let (input, output) = if input.is_output() {
            (output, input)
        } else {
            (input, output)
        };
        
        (input, output)
    }

    pub fn add_connector<T>(&mut self, name: String, value: T, connector_type: i32, is_output: bool) -> usize 
    where
        T: Into<ConnectorType>
    {
        let index = self.connectors.len();

        self.connectors.push(Connector::new(index, name, value, connector_type, is_output));

        index
    }

    pub fn delete_connector(&mut self, index: usize) {
        // delete edge
        self.disconnect(index);
        self.connectors.remove(index);

        for connector in &mut self.connectors {
            if connector.get_index() > index {
                connector.set_index(index - 1);
            }
        }
    }

    pub fn connect(&mut self, input_index: usize, output_index: usize) {
        let (input, output) = self.get_two_connectors(input_index, output_index);
        let (mut input_index, mut output_index) = (input.get_index(), output.get_index());

        match (input.is_output(), output.is_output()) {
            (true, false) => {
                std::mem::swap(&mut input_index, &mut output_index);
            },
            (false, true) => (),
            _ => return,
        }
        
        let (input, output) = self.get_input_output_mut(input_index, output_index);

        match (input.has_edge(), output.has_edge()) {
            (false, true) => {
                let edge = output.get_edge_mut().unwrap();

                if !edge.get_input().contains(&input_index) {
                    edge.add_input(input_index);
                    input.connect_input(output);
                }
            }
            (false, false) => {
                let edge = ConnectorEdge::new(vec![input_index], output_index);
                output.connect_output(edge);
                input.connect_input(output);
            }
            (true, _) => {
                let input_connector = *input.get_connected().first().unwrap();

                if input_connector != output_index {
                    self.disconnect(input_index);

                    let (input, output) = self.get_input_output_mut(input_index, output_index);

                    if output.has_edge() {
                        let edge = output.get_edge_mut().unwrap();
                        edge.add_input(input.get_index());
                        input.connect_input(output);
                    } else {
                        let edge = ConnectorEdge::new(vec![input_index], output_index);
                        output.connect_output(edge);
                        input.connect_input(output);
                    }
                }
            }
        }
    }

    pub fn disconnect(&mut self, index: usize) {
        let connector = self.get_connector(index).unwrap();

        if connector.has_edge() {
            match connector.is_output() {
                true => {
                    for input in self.connectors.iter_mut().filter(|connector| (connector.is_input() || connector.get_index() == index)){
                        if input.get_index() != index {
                            input.remove_edge();
                        } else {
                            input.remove_edge();
                        }
                    }
                },
                false => {
                    let edge = connector.get_connected().first().unwrap().clone();
                    let edge = self.get_connector(edge).unwrap().get_edge().unwrap();
    
                    // If we are input, so check if only 1 connection, if so disconnect edge on both ends
                    if edge.get_input().len() == 1 {
                        let output_index = edge.get_output();
    
                        for connector in self.connectors.iter_mut().filter(|connector| connector.get_index() == output_index || connector.get_index() == index) {
                            connector.remove_edge();
                        }
                    } else {
                        self.connectors.get_mut(index).unwrap().remove_edge();
                    }
                }
            }
        }
    }
}