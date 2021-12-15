// run: to_json()

#[derive(Document)]
#[doku(untagged)]
pub enum Enum {
    /// This is `Foo`
    Foo {
        /// Some comment
        a: String,
    },

    /// This is `Bar`
    Bar {
        /// Some comment
        a: String,

        /// Some comment
        b: usize,
    },
}

#[derive(Document)]
pub struct Inner {
    /// Some comment
    foo: String,

    /// Another comment
    bar: String,

    /// Payload
    #[doku(tag = "tag")]
    payload: Enum,
}

pub type Ty = Vec<Inner>;