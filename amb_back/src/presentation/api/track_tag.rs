 // Use macros to ignore type problems and make them sharable to tests/

 #[macro_export]
 macro_rules! list_track_tags {
     () => {
         track_tag_route::list()
             .and_then(track_tag_handler::list)
     };
 }
 
 #[macro_export]
 macro_rules! get_track_tag {
     () => {
         track_tag_route::get()
             .and_then(track_tag_handler::get)
     };
 }
 
 #[macro_export]
 macro_rules! create_track_tag {
     () => {
         track_tag_route::create()
             .and_then(track_tag_handler::create)
     };
 }
 
 
 #[macro_export]
 macro_rules! update_track_tag {
     () => {
         track_tag_route::update()
             .and_then(track_tag_handler::update)
     };
 }
 
 
 #[macro_export]
 macro_rules! delete_track_tag {
     () => {
         track_tag_route::delete()
             .and_then(track_tag_handler::delete)
     };
 }
 
 
 