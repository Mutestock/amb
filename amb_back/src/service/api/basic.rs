#[macro_export]
macro_rules! health {
    () => {
        basic_routes::health()
            .and_then(basic_handler::health)
    }
}

#[macro_export]
macro_rules! check_conn_string {
    () => {
        basic_routes::check_conn_string()
            .and_then(basic_handler::check_conn_string)
    };
}

#[macro_export]
macro_rules! check_basic_connection {
    () => {
        basic_routes::basic_connection()
            .and_then(basic_handler::basic_connection)
    };
}

#[macro_export]
macro_rules! home {
    () => {
        basic_routes::home()
            .and_then(basic_handler::home)
    };
}