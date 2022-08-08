pub trait Provider {
    type Type;
}

pub struct MyProvider;

impl Provider for MyProvider {
    type Type = f32;
}
