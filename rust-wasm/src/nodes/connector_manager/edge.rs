use super::Manager;
use super::Connector;

pub struct ConnectorEdge {
    index: usize,
    input: Option<usize>,
    output: Option<usize>,
}

impl ConnectorEdge {
    pub fn new(index: usize) -> ConnectorEdge {
        ConnectorEdge {
            index: index,
            input: None,
            output: None,
        }
    }

    pub fn input_connector<'a>(&'a self, manager: &'a Manager) -> Option<&Connector> {
        match self.input {
            Some(index) => manager.get_connector(index),
            None => None,
        }
    }

    pub fn output_connector<'a>(&'a self, manager: &'a Manager) -> Option<&Connector> {
        match self.output {
            Some(index) => manager.get_connector(index),
            None => None,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn has_connection(&self, connector: usize) -> bool {
        match self.input {
            Some(input) => if input == connector { return true; },
            None => (),
        }

        match self.output {
            Some(output) => if output == connector { return true; },
            None => (),
        }

        false
    }

    pub fn get_input(&self) -> Option<usize> {
        self.input
    }

    pub fn get_output(&self) -> Option<usize> {
        self.output
    }

    pub fn set_input(&mut self, input: Option<usize>) {
        self.input = input;
    }

    pub fn set_output(&mut self, output: Option<usize>) {
        self.output = output;
    }

    pub fn connect(&mut self, input: Option<usize>, output: Option<usize>) {
        self.input = input;
        self.output = output;
    }

    pub fn disconnect(&mut self) {
        self.input = None;
        self.output = None;
    }
}

impl core::fmt::Debug for ConnectorEdge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ConnectorEdge {{ input: {:?}, output: {:?} }}", self.get_input(), self.get_output())
    }
}