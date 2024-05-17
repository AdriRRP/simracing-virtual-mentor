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
    }
    pub mod domain {
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

pub mod api {
    pub mod infrastructure {
        pub mod controller {
            pub mod upload_file;
        }
    }
}

use crate::api::infrastructure::controller::upload_file::upload_file;
use axum::extract::DefaultBodyLimit;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", post(upload_file))
        .layer(DefaultBodyLimit::disable());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    //tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
