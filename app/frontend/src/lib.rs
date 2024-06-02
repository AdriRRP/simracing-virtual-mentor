extern crate symracing_virtual_mentor_shared as shared;
pub mod infrastructure {
    pub mod components {
        pub mod app;
        //pub mod scatter_plot;
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
    }
    pub mod settings;
}
