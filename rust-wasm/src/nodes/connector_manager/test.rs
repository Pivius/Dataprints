use super::Manager;
use super::ConnectorType;

#[cfg(test)]
mod tests {
    use super::*;

    //
    #[test]
    fn io_connection() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 3, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, true);

        // Connect them
        manager.connect(connector1, connector2);

        // Check if they are connected
        let edge = manager.get_connector_edge(connector1).unwrap();
        assert_eq!(edge.get_input().unwrap(), connector1, "Edge input should be connector 1");
        assert_eq!(edge.get_output().unwrap(), connector2, "Edge output should be connector 2");
    }

    #[test]
    fn io_fail_connection() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 3, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, false);

        // Connect them
        manager.connect(connector1, connector2);

        // Check if edge exist
        let edge = manager.get_connector_edge(connector1);
        assert_eq!(edge.is_none(), true, "Edge should not exist");
    }

    #[test]
    fn io_disconnect() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 3, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, true);

        // Connect them
        manager.connect(connector1, connector2);
        
        // Check connection
        let edge = manager.get_connector_edge(connector1).unwrap();
        assert_eq!(edge.get_input().unwrap(), connector1, "Edge input should be connector 1");
        assert_eq!(edge.get_output().unwrap(), connector2, "Edge output should be connector 2");

        // Disconnect them
        manager.disconnect(connector1);

        // Check if edge exist
        let edge = manager.get_connector_edge(connector1);
        assert_eq!(edge.is_none(), true, "Edge should not exist");
    }

    #[test]
    fn io_connect_third() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 3, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, true);
        let connector3 = manager.add_connector("connector3".to_string(), 7, false);

        // Connect them
        manager.connect(connector1, connector2);

        // Check if they are connected
        let edge = manager.get_connector_edge(connector1).unwrap();
        assert_eq!(edge.get_input().unwrap(), connector1, "Edge input should be connector 1");
        assert_eq!(edge.get_output().unwrap(), connector2, "Edge output should be connector 2");

        manager.connect(connector2, connector3);

        // Edge should be from connector 2 to connector 3
        let edge = manager.get_connector_edge(connector2).unwrap();
        assert_eq!(edge.get_input().unwrap(), connector3, "Edge input should be connector 1");
        assert_eq!(edge.get_output().unwrap(), connector2, "Edge output should be connector 2");
        
        // Edge from connector 1 should not exist
        let edge = manager.get_connector_edge(connector1);
        assert_eq!(edge.is_none(), true, "Edge should not exist");
    }

    #[test]
    fn value_getter() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 3, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, true);

        assert_eq!(manager.get_connector(connector1).unwrap().get_value(&manager), 3, "Connector 1 value should be 3");
        assert_eq!(manager.get_connector(connector2).unwrap().get_value(&manager), 5, "Connector 2 value should be 5");
        // Set value
        manager.set_connector_value(connector1, 7);
        assert_eq!(manager.get_connector(connector1).unwrap().get_value(&manager), 7, "Connector 1 value should be 7");

        // Connect them
        manager.connect(connector1, connector2);

        // Check if they are connected
        let edge = manager.get_connector_edge(connector1).unwrap();
        assert_eq!(edge.get_input().unwrap(), connector1, "Edge input should be connector 1");
        assert_eq!(edge.get_output().unwrap(), connector2, "Edge output should be connector 2");

        // Check value of input connector
        assert_eq!(manager.get_connector(connector1).unwrap().get_value(&manager), 5, "Connector 1 value should be equal to connector 2 value");

        // Set value of connector 2 and check if connector 1 value is equal to connector 2 value
        manager.set_connector_value(connector2, 11);
        assert_eq!(manager.get_connector(connector1).unwrap().get_value(&manager), 11, "Connector 1 value should be equal to connector 2 value");

        // Disconnect them
        manager.disconnect(connector1);

        // Check value of connector1
        assert_eq!(manager.get_connector(connector1).unwrap().get_value(&manager), 7, "Connector 1 value should be 7");
    }

    #[test]
    fn variable_type() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 3, false);

        assert_eq!(manager.get_connector(connector1).unwrap().get_value_type(), ConnectorType::Integer, "Connector 1 value type should be Integer");

        // Set value
        manager.set_connector_value(connector1, true);
        assert_eq!(manager.get_connector(connector1).unwrap().get_value_type(), ConnectorType::Boolean, "Connector 1 value type should be Boolean");
    }

    #[test]
    fn variable_type_connection() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 3, false);
        let connector2 = manager.add_connector("connector2".to_string(), 6, true);

        // Connect them
        manager.connect(connector1, connector2);

        // Check if they are connected
        let edge = manager.get_connector_edge(connector1).unwrap();
        assert_eq!(edge.get_input().unwrap(), connector1, "Edge input should be connector 1");
        assert_eq!(edge.get_output().unwrap(), connector2, "Edge output should be connector 2");

        // Check value of input connector
        assert_eq!(manager.get_connector(connector1).unwrap().get_value(&manager), 6, "Connector 1 value should be equal to connector 2 value");

        manager.set_connector_value(connector2, true);

        // Check if edge exist
        let edge = manager.get_connector_edge(connector1);
        assert_eq!(edge.is_none(), true, "Edge should not exist");
    }
}