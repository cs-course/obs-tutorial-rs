use rusty_s3::{Bucket, Credentials};

pub struct SingleTask {
    pub(crate) bucket: Bucket,
    pub(crate) credentials: Credentials,
    pub(crate) object: String,
}

impl SingleTask {
    pub fn new<S: Into<String>>(bucket: Bucket, credentials: Credentials, object: S) -> Self {
        Self {
            bucket,
            credentials,
            object: object.into(),
        }
    }
}
