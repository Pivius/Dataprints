use super::edge::ConnectorEdge;
use super::ConnectorType;
use crate::helper::types::Null;

pub struct ConnectorInfo {
    index: usize,
    name: String,
}

impl ConnectorInfo {
    pub fn new(index: usize, name: String) -> ConnectorInfo {
        ConnectorInfo {
            index,
            name,
        }
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub enum Connector {
    Input(ConnectorInfo, Option<usize>, i32),
    Output(ConnectorInfo, Option<ConnectorEdge>, ConnectorType, i32),
}

impl Connector {
    pub fn new<T>(index: usize, name: String, value: T, connector_type: i32, is_output: bool) -> Connector
    where
        T: Into<ConnectorType>
    {
        let info = ConnectorInfo::new(index, name);
        let value: ConnectorType = value.into();

        match is_output {
            false => Connector::Input(info, None, connector_type),
            true => Connector::Output(info, None, value.get_variant(connector_type), connector_type),
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            Connector::Input(_, _, _) => true,
            Connector::Output(_, _, _, _) => false,
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            Connector::Input(_, _, _) => false,
            Connector::Output(_, _, _, _) => true,
        }
    }

    pub fn set_index(&mut self, index: usize) {
        match self {
            Connector::Input(info, _, _) => info.index = index,
            Connector::Output(info, _, _, _) => info.index = index,
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            Connector::Input(info, _, _) => info.index,
            Connector::Output(info, _, _, _) => info.index,
        }
    }

    pub fn get_type(&self) -> i32 {
        match self {
            Connector::Input(_, _, connector_type) => connector_type.clone(),
            Connector::Output(_, _, _, connector_type) => connector_type.clone(),
        }
    }

    pub fn set_value<T>(&mut self, new_val: T) 
    where
        T: Into<ConnectorType>
    {
        match self {
            Connector::Output(info, _, value, connector_type) => {
                let new_val = new_val.into();
                *value = new_val.get_variant(connector_type.clone());
            },
            _ => panic!("Cannot set value for input connector"),
        }
    }

    pub fn get_value(&self) -> ConnectorType {
        match self {
            Connector::Output(_, _, value, _) => value.clone(),
            _ => ConnectorType::Null(Null::new()),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Connector::Input(info, _, _) => info.name.clone(),
            Connector::Output(info, _, _, _) => info.name.clone(),
        }
    }

    pub fn has_edge(&self) -> bool {
        match self {
            Connector::Input(_, connected, _) => connected.is_some(),
            Connector::Output(_, edge, _, _) => edge.is_some(),
        }
    }

    pub fn get_edge(&self) -> Option<&ConnectorEdge> {
        match self {
            Connector::Output(_, edge, _, _) => edge.as_ref(),
            _ => panic!("Cannot get edge for input connector, use get_connected instead"),
        }
    }

    pub fn get_edge_mut(&mut self) -> Option<&mut ConnectorEdge> {
        match self {
            Connector::Output(_, edge, _, _) => edge.as_mut(),
            _ => panic!("Cannot get mutable edge for input connector, use get_connected instead"),
        }
    }

    pub fn remove_edge(&mut self) {
        match self {
            Connector::Input(_, connected, _) => *connected = None,
            Connector::Output(_, edge, _, _) => *edge = None,
        }
    }

    pub fn connect_input(&mut self, output: &Connector) {
        match self {
            Connector::Input(_, connected, _) => {
                *connected = Some(output.get_index());
            },
            _ => panic!("Cannot connect output to output connector"),
        }
    }

    pub fn connect_output(&mut self, new_edge: ConnectorEdge) {
        match self {
            Connector::Output(_, edge, _, _) => {
                *edge = Some(new_edge);
            },
            _ => panic!("Cannot connect input to input connector"),
        }
    }

    pub fn disconnect_from_edge(&mut self, input: usize) {
        match self {
            Connector::Output(info, edge, _, _) => {
                if let Some(edge) = edge {
                    if edge.has_connection(input) {
                        edge.remove_input(input);
                    }
                }
            },
            _ => panic!("Cannot disconnect from edge for input connector as outputs own the edges"),
        }
    }

    pub fn get_connected(&self) -> Vec<usize> {
        match self {
            Connector::Input(_, connected, _) => connected.into_iter().map(|x| *x).collect(),
            Connector::Output(_, edge, _, _) => {
                match edge {
                    Some(edge) => edge.get_input(),
                    None => Vec::new(),
                }
            },
        }
    }

    pub fn get_connected_to(&self, connection: usize) -> Option<usize> {
        match self {
            Connector::Input(_, connected, _) => {
                match connected {
                    Some(connected) => {
                        if *connected == connection {
                            Some(*connected)
                        } else {
                            None
                        }
                    },
                    None => None,
                }
            },
            Connector::Output(_, edge, _, _) => {
                match edge {
                    Some(edge) => {
                        if edge.get_input()[0] == connection {
                            Some(edge.get_input()[0])
                        } else {
                            None
                        }
                    },
                    None => None,
                }
            },
        }
    }
}

impl PartialEq for Connector {
    fn eq(&self, other: &Self) -> bool {
        self.get_index() == other.get_index()
    }
}

impl PartialEq for ConnectorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.name == other.name
    }
}

impl Clone for ConnectorInfo {
    fn clone(&self) -> Self {
        ConnectorInfo {
            index: self.index,
            name: self.name.clone(),
        }
    }
}

impl Clone for Connector {
    fn clone(&self) -> Self {
        match self {
            Connector::Input(info, edge, connector_type) => Connector::Input(info.clone(), edge.clone(), connector_type.clone()),
            Connector::Output(info, edges, value, connector_type) => Connector::Output(info.clone(), edges.clone(), value.clone(), connector_type.clone()),
        }
    }
}

impl core::fmt::Debug for Connector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Connector {{ index: {}, name: {}, type: {:?}, edge: {:?}, is_output: {:?}}}", self.get_index(), self.get_name(), self.get_type(), self.has_edge(), self.is_output())
    }
}

impl core::fmt::Debug for ConnectorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ConnectorInfo {{ index: {}, name: {} }}", self.index, self.name)
    }
}