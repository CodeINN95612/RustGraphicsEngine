pub mod layers;

use engine::app::app_builder::AppBuilder;
use layers::test_layer::TestLayer;

fn main() {
    if let Some(mut app) = AppBuilder::new("Sandbox").build() {
        app.push_layer(Box::new(TestLayer::new()));
        app.run().unwrap();
    }
}
