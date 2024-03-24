pub struct ConnectorEdge {
    input: Vec<usize>,
    output: usize,
}

impl ConnectorEdge {
    pub fn new(input: Vec<usize>, output: usize) -> ConnectorEdge {
        ConnectorEdge {
            input,
            output,
        }
    }

    pub fn has_connection(&self, connector: usize) -> bool {
        self.output == connector || self.input.contains(&connector)
    }

    pub fn get_input(&self) -> Vec<usize> {
        self.input.clone()
    }

    pub fn get_output(&self) -> usize {
        self.output
    }

    pub fn add_input(&mut self, input: usize) {
        if !self.input.contains(&input) {
            self.input.push(input);
        }
    }

    pub fn remove_input(&mut self, input: usize) {
        if self.input.contains(&input) {
            self.input.retain(|&x| x != input);
        }
    }
}

impl Clone for ConnectorEdge {
    fn clone(&self) -> ConnectorEdge {
        ConnectorEdge {
            output: self.output.clone(),
            input: self.input.clone(),
        }
    }
}

impl core::fmt::Debug for ConnectorEdge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ConnectorEdge {{ input: {:?}, output: {:?} }}", self.get_input(), self.get_output())
    }
}