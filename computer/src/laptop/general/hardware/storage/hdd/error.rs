//implement the error file

pub struct StorageError {
    message: String,
}

pub struct StorageResult<T> {
    value: T,
}

impl StorageError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl StorageResult<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}




