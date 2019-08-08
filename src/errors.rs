#[derive(Debug)]
pub struct Error {
    inner: Box<Inner>,
}

impl Error {
    pub(crate) fn new(kind: Kind) -> Error {
        Error {
            inner: Box::new(Inner { kind }),
        }
    }
}

#[derive(Debug)]
struct Inner {
    kind: Kind,
}

#[derive(Debug)]
pub(crate) enum Kind {
    Reqwest(::reqwest::Error),
    Json(::serde_json::Error),

    EmptyId,
    EmptyQuery,
    EmptyToken,
    ValidationError(&'static str),
}
