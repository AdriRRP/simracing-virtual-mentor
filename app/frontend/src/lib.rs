extern crate symracing_virtual_mentor_shared as shared;
pub mod infrastructure {
    pub mod components {
        pub mod analyses;
        pub mod analysis_creator;
        pub mod app;
        pub mod dashboard;
        pub mod files;
        pub mod home;
        pub mod laps;
        pub mod navbar;
        pub mod repository_context;
        pub mod routes;
    }
    pub mod repository {
        pub mod analysis {
            pub mod http;
        }
        pub mod file {
            pub mod http;
        }
        pub mod lap {
            pub mod http;
        }
        pub mod ibt {
            pub mod http;
        }
    }
    pub mod settings;
}

pub mod utils;
