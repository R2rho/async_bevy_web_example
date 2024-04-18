#[cfg(feature = "ssr")]
mod ssr_dependencies {
    pub use bevy::prelude::*; 
    pub use tokio::runtime::Runtime;
    pub use leptos::*;
    pub use leptos_axum::{generate_route_list, LeptosRoutes, generate_route_list_with_ssg, generate_route_list_with_exclusions, generate_route_list_with_exclusions_and_ssg};
    pub use example_abw_app::app::*;
    pub use example_abw_app::fileserv::file_and_error_handler;
    pub use async_bevy_web::prelude::Config;
    pub use async_bevy_web::prelude::LeptosAppPlugin;

    
}

#[cfg(feature = "ssr")]
use ssr_dependencies::*;

#[cfg(feature = "ssr")]
fn main() {
    
    
    App::new()
        .with_default_config()
        .with_frame_rate(60.0)
        .add_plugins(LeptosAppPlugin::new(MyApp))
        // .add_systems(Update, print_status)
        .run();
    
}


#[cfg(not(feature = "ssr"))]
fn main() {
    // Alternative main function for non-ssr features
    // println!("Non-SSR version does not have a server component.");
}

fn print_status(){
    println!("Running...")
}