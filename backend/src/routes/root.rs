use rocket::response::Redirect;

#[get("/")]
pub fn redirect() -> Redirect {
    Redirect::to("/public")
}