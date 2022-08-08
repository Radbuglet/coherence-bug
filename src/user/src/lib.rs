use def::{MyProvider, Provider};

// === Doesn't work === //

pub trait Trait<T> {}

pub struct Target;

impl Trait<()> for Target {}

impl Trait<<MyProvider as Provider>::Type> for Target {}

// === Works === //

pub struct MyLocalProvider;

impl Provider for MyLocalProvider {
    type Type = f32;
}

pub struct Target2;

impl Trait<()> for Target2 {}

impl Trait<<MyLocalProvider as Provider>::Type> for Target2 {}
