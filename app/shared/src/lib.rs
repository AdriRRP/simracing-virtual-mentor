pub mod analysis {
    pub mod application {
        pub mod create {
            pub mod service;
        }
        pub mod delete {
            pub mod service;
        }
        pub mod find {
            pub mod by_id {
                pub mod service;
            }
            pub mod by_criteria {
                pub mod service;
            }
        }
    }
    pub mod domain {
        pub mod analyses;
        pub mod analysis;
        pub mod repository;
    }
    pub mod infrastructure {
        pub mod repository {
            pub mod in_memory;
        }
    }
}

pub mod common {
    pub mod domain {
        pub mod event;
    }
}

pub mod file {
    pub mod application {
        pub mod create {
            pub mod service;
        }
        pub mod delete {
            pub mod service;
        }
        pub mod find {
            pub mod by_id {
                pub mod service;
            }
            pub mod by_criteria {
                pub mod service;
            }
        }
        pub mod validate {
            pub mod service;
        }
    }
    pub mod domain {
        pub mod event {
            pub mod created;
            pub mod deleted;
            pub mod validated;
        }
        pub mod file;
        pub mod files;
        pub mod repository;
    }
    pub mod infrastructure {
        pub mod repository {
            pub mod in_memory;
        }
    }
}

pub mod ibt {
    pub mod domain {
        pub mod file;
    }
}

pub mod lap {
    pub mod application {
        pub mod create {
            pub mod service;
        }
        pub mod delete {
            pub mod service;
        }
        pub mod find {
            pub mod by_id {
                pub mod service;
            }
            pub mod by_criteria {
                pub mod service;
            }
            pub mod header_by_id {
                pub mod service;
            }
            pub mod headers_by_criteria {
                pub mod service;
            }
        }
    }
    pub mod domain {
        pub mod lap;
        pub mod laps;
        pub mod repository;
    }
    pub mod infrastructure {
        pub mod repository {
            pub mod in_memory;
        }
    }
}
