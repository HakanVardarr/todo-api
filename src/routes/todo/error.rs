pub enum Error {
    NotFound,
    InternalServerError(String),
    Unauthorized,
}
