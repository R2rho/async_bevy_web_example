#[cfg(feature = "ssr")]
fn main() {
    use bevy::prelude::*;
    use async_bevy_web::prelude::Config;
    use async_bevy_web::prelude::LeptosAppPlugin;
    use example_abw_app_lib::app::*;
    use example_abw_app_lib::fileserv::file_and_error_handler;
 
    App::new()
        .with_default_config()
        .with_frame_rate(60.0)
        .add_plugins(LeptosAppPlugin::new(MyApp))
        .run();   
}


#[cfg(not(feature = "ssr"))]
fn main() {
    // use example_abw_app_lib::app::*;
    // Alternative main function for non-ssr features
    // println!("Non-SSR version does not have a server component.");
    // leptos::mount_to_body(MyApp)
}
