use super::Manager;
use super::value::{ConnectorType, ConnectorTypeId};

#[cfg(test)]
mod tests {
    use super::*;

    //
    #[test]
    fn io_connection() {
        let mut manager = Manager::new();
        
        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 0, ConnectorTypeId::INT, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, ConnectorTypeId::INT, true);

        assert_eq!(manager.get_value(connector2), 5, "Connector 2 should have value 5");

        // Connect them
        manager.connect(connector1, connector2);

        // Check if the value is passed
        assert_eq!(manager.get_value(connector1), 5, "Connector 2 should have value 5");
        assert_eq!(manager.get_value(connector2), 5, "Connector 2 should have value 5");

        // Edge check
        assert_eq!(manager.get_connector(connector1).unwrap().get_connected().first().unwrap(), &connector2, "Connector 1 should be connected to connector 2");
        assert_eq!(manager.get_connector(connector2).unwrap().get_edge().is_none(), false,  "Connector 2 should have edge to connector 1");

        println!("Connector 1 edge: {:?}", manager.get_connector(connector1).unwrap().get_connected());
        println!("Connector 2 edge: {:?}", manager.get_connector(connector2).unwrap().get_edge());

        // Disconnect
        manager.disconnect(connector1);

        assert_eq!(manager.get_value(connector1), 0, "Connector 2 should have value 0");

        assert_eq!(manager.get_connector(connector1).unwrap().get_connected().first(), None, "Connector 1 should be disconnected");
        assert_eq!(manager.get_connector(connector2).unwrap().get_edge().is_none(), true,  "Connector 2 should be disconnected");

        println!("Connector 1 edge: {:?}", manager.get_connector(connector1).unwrap().get_connected());
        println!("Connector 2 edge: {:?}", manager.get_connector(connector2).unwrap().get_edge());
    }

    #[test]
    fn io_fail_connection_output() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 0, ConnectorTypeId::INT, true);
        let connector2 = manager.add_connector("connector2".to_string(), 5, ConnectorTypeId::INT, true);

        // Connect them
        manager.connect(connector1, connector2);

        // Check if edge exist
        assert_eq!(manager.get_connector(connector1).unwrap().get_edge().is_none(), true, "Connector 1 should be disconnected");
        assert_eq!(manager.get_connector(connector2).unwrap().get_edge().is_none(), true,  "Connector 2 should be disconnected");
    }

    #[test]
    fn io_fail_connection_input() {
        let mut manager = Manager::new();

        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 0, ConnectorTypeId::INT, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, ConnectorTypeId::INT, false);

        // Connect them
        manager.connect(connector1, connector2);

        // Check if edge exist
        assert_eq!(manager.get_connector(connector1).unwrap().get_connected().first().is_none(), true, "Connector 1 should be disconnected");
        assert_eq!(manager.get_connector(connector2).unwrap().get_connected().first().is_none(), true,  "Connector 2 should be disconnected");
    }

    #[test]
    fn io_connection_conversion() {
        let mut manager = Manager::new();
        
        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 0, ConnectorTypeId::INT, false);
        let connector2 = manager.add_connector("connector2".to_string(), 3.14, ConnectorTypeId::FLOAT, true);
        let connector3 = manager.add_connector("connector3".to_string(), "Hello, World!", ConnectorTypeId::STRING, true);
        let connector4 = manager.add_connector("connector4".to_string(), false, ConnectorTypeId::BOOL, true);
        let connector5 = manager.add_connector("connector5".to_string(), true, ConnectorTypeId::BOOL, true);

        // Connect them
        manager.connect(connector1, connector2);

        assert_eq!(manager.get_value(connector1), 3, "Connector 1 should have value 3");

        println!("Connector 1: {:?}", manager.get_value(connector1));
        println!("Connector 2: {:?}", manager.get_value(connector2));
        
        manager.disconnect(connector1);
        manager.connect(connector1, connector3);
        
        assert_eq!(manager.get_value(connector1), 0, "Connector 1 should have value 'Hello, World!'");

        manager.set_value(connector3, "42");

        assert_eq!(manager.get_value(connector1), 42, "Connector 1 should have value 42");

        manager.disconnect(connector1);
        manager.connect(connector1, connector4);

        assert_eq!(manager.get_value(connector1), 0, "Connector 1 should have value false");

        manager.disconnect(connector1);
        manager.connect(connector1, connector5);

        assert_eq!(manager.get_value(connector1), 1, "Connector 1 should have value true");
    }

    #[test]
    fn io_connection_multiple_input() {
        let mut manager = Manager::new();
        
        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 0, ConnectorTypeId::INT, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, ConnectorTypeId::INT, true);
        let connector3 = manager.add_connector("connector3".to_string(), 10, ConnectorTypeId::INT, true);
        let connector4 = manager.add_connector("connector4".to_string(), 15, ConnectorTypeId::INT, true);

        // Connect them
        manager.connect(connector1, connector2);

        assert_eq!(manager.get_value(connector1), 5, "Connector 1 should have value 5");

        manager.connect(connector1, connector3);

        assert_eq!(manager.get_value(connector1), 10, "Connector 1 should have value 10");

        manager.connect(connector1, connector4);

        assert_eq!(manager.get_value(connector1), 15, "Connector 1 should have value 15");
    }

    #[test]
    fn io_connection_multiple_output() {
        let mut manager = Manager::new();
        
        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 0, ConnectorTypeId::INT, true);
        let connector2 = manager.add_connector("connector2".to_string(), 5, ConnectorTypeId::INT, false);
        let connector3 = manager.add_connector("connector3".to_string(), 10, ConnectorTypeId::INT, false);
        let connector4 = manager.add_connector("connector4".to_string(), 15, ConnectorTypeId::INT, false);

        // Connect them
        manager.connect(connector1, connector2);
        manager.connect(connector1, connector3);
        manager.connect(connector1, connector4);

        assert_eq!(manager.get_value(connector2), 0, "Connector 2 should have value 0");
        assert_eq!(manager.get_value(connector3), 0, "Connector 3 should have value 0");
        assert_eq!(manager.get_value(connector4), 0, "Connector 4 should have value 0");

        // Set value
        manager.set_value(connector1, 5);

        assert_eq!(manager.get_value(connector2), 5, "Connector 2 should have value 5");
        assert_eq!(manager.get_value(connector3), 5, "Connector 3 should have value 5");
        assert_eq!(manager.get_value(connector4), 5, "Connector 4 should have value 5");

        // Disconnect
        manager.disconnect(connector1);

        assert_eq!(manager.get_value(connector2), 0, "Connector 2 should have value 0");
        assert_eq!(manager.get_value(connector3), 0, "Connector 3 should have value 0");
        assert_eq!(manager.get_value(connector4), 0, "Connector 4 should have value 0");
    }

    #[test]
    fn io_reconnection() {
        let mut manager = Manager::new();
        
        // Create connectors
        let connector1 = manager.add_connector("connector1".to_string(), 0, ConnectorTypeId::INT, false);
        let connector2 = manager.add_connector("connector2".to_string(), 5, ConnectorTypeId::INT, true);
        let connector3 = manager.add_connector("connector3".to_string(), 10, ConnectorTypeId::INT, false);
        let connector4 = manager.add_connector("connector4".to_string(), 15, ConnectorTypeId::INT, true);

        // Connect them
        manager.connect(connector1, connector2);
        manager.connect(connector3, connector4);

        assert_eq!(manager.get_value(connector1), 5, "Connector 1 should have value 5");
        assert_eq!(manager.get_value(connector3), 15, "Connector 3 should have value 15");

        manager.connect(connector4, connector1);

        assert_eq!(manager.get_value(connector1), 15, "Connector 1 should have value 15");
        assert_eq!(manager.get_value(connector3), 15, "Connector 3 should have value 15");
        assert_eq!(manager.get_connector(connector2).unwrap().get_connected().first().is_none(), true, "Connector 2 should be disconnected");
    }
}