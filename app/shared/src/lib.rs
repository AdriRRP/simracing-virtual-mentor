pub mod telemetry {
    pub mod application { }
    pub mod domain {
        pub mod circuit;
        pub mod lap;
        pub mod player;
        pub mod repository;
        pub mod session;
    }
    pub mod infrastructure {
        pub mod repository {
            pub mod in_memory;
        }
    }
}
