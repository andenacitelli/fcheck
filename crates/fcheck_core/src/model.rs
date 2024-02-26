/// Custom structs that may be more generally applicable than the file they're defined in.

pub struct Issue {
    pub filepath: String,
    pub line: i32,
    pub message: String,
}
