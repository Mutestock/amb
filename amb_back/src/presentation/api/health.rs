#[macro_export]
macro_rules! health {
    () => {
        basic_routes::health()
        .and_then(health_handler::health)
    }
}