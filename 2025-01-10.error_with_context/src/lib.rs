pub trait ErrorContextI {
    fn new_error_context(e: &dyn std::error::Error) -> Self;
}
