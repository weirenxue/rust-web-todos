use hello_rocket::{handler, model};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::init();
    rocket::build().manage(app_data).mount(
        "/api",
        routes![
            handler::health_checker_handler,
            handler::todos_list_handler,
            handler::create_todo_handler,
            handler::get_todo_handler,
            handler::edit_todo_handler,
            handler::delete_todo_handler,
        ],
    )
}
