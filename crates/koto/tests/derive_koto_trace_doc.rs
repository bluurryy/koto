use koto::derive::*;

#[allow(dead_code)]
mod example {
    use koto::Ptr;

    use super::*;

    mod some_dependency {
        pub struct Position;
        pub struct Rotation;
        pub struct Color;
    }

    #[derive(KotoTrace)]
    struct Point {
        x: f64,
        y: f64,
    }

    // Ignore a field that we know has no `Ptr`s inside.
    #[derive(KotoTrace)]
    struct Node {
        parent: Option<Ptr<Node>>,
        children: Vec<Ptr<Node>>,

        #[koto(trace(ignore))]
        color: some_dependency::Color,
    }

    // Ignore all fields.
    #[derive(KotoTrace)]
    #[koto(trace(ignore))]
    struct Ball {
        position: some_dependency::Position,
        velocity: some_dependency::Position,
        rotation: some_dependency::Rotation,
    }
}
