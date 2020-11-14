 // Use macros to ignore type problems and make them sharable to tests/

 #[macro_export]
 macro_rules! list_users {
     () => {
         user_route::list()
             .and_then(user_handler::list)
     };
 }
 
 #[macro_export]
 macro_rules! get_user {
     () => {
         user_route::get()
             .and_then(user_handler::get)
     };
 }
 
 #[macro_export]
 macro_rules! create_user {
     () => {
         user_route::create()
             .and_then(user_handler::create)
     };
 }
 
 
 #[macro_export]
 macro_rules! update_user {
     () => {
         user_route::update()
             .and_then(user_handler::update)
     };
 }
 
 
 #[macro_export]
 macro_rules! delete_user {
     () => {
         user_route::delete()
             .and_then(user_handler::delete)
     };
 }
 
 
 