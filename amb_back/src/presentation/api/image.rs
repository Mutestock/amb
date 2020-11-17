#[macro_export]
macro_rules! list_images {
    () => {
        image_routes::list()
            .and_then(image_handler::list)
    };
}

#[macro_export]
macro_rules! get_image {
    () => {
        image_routes::get()
            .and_then(image_handler::get)
    };
}

#[macro_export]
macro_rules! create_image {
    () => {
        image_routes::create()
            .and_then(image_handler::create)
    };
}

 
#[macro_export]
macro_rules! update_image {
    () => {
        image_routes::update()
            .and_then(image_handler::update)
    };
}


#[macro_export]
macro_rules! delete_image {
    () => {
        image_routes::delete()
            .and_then(image_handler::delete)
    };
}


