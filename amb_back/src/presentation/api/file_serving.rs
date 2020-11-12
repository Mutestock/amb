#[macro_export]
macro_rules! get_track {
    () => {
        file_serving_routes::get_track()
            .and_then(file_serving_handler::get_track())
    };
}