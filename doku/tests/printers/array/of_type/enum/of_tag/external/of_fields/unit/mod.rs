use crate::prelude::*;

type Ty = Vec<Inner>;

#[derive(Document)]
enum Inner {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
}

printer_test! {
    "output.json" => to_json(Ty),
}
