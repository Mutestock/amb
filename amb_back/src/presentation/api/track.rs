 // Use macros to ignore type problems and make them sharable to tests/

#[macro_export]
macro_rules! list_tracks {
    () => {
        track_route::list()
            .and_then(track_handler::list)
    };
}

#[macro_export]
macro_rules! get_track {
    () => {
        track_route::get()
            .and_then(track_handler::get)
    };
}

#[macro_export]
macro_rules! create_track {
    () => {
        track_route::create()
            .and_then(track_handler::create)
    };
}


#[macro_export]
macro_rules! update_track {
    () => {
        track_route::update()
            .and_then(track_handler::update)
    };
}


#[macro_export]
macro_rules! delete_track {
    () => {
        track_route::delete()
            .and_then(track_handler::delete)
    };
}


