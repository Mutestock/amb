#[macro_export]
macro_rules! health {
    () => {
        basic_routes::health()
            .and_then(health_handler::health)
    }
}

#[macro_export]
macro_rules! check_conn_string {
    () => {
        basic_routes::check_conn_string()
            .and_then(health_handler::check_conn_string)
    };
}

#[macro_export]
macro_rules! check_basic_connection {
    () => {
        basic_routes::basic_connection()
            .and_then(health_handler::basic_connection)
    };
}