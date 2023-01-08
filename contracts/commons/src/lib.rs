    #[openbrush::trait_definition]
    pub trait Destination {
        #[ink(message)]
        fn execute_function(&mut self, function_name: String, parameters: String)
            -> Result<(), ()>;
    }
