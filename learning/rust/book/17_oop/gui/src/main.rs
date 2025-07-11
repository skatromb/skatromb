use gui::{Button, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    "Yes".to_string(),
                    "Maybe".to_string(),
                    "No".to_string(),
                ],
            }),
            
            Box::new(Button {
                width: 50,
                height: 10,
                label: "OK".to_string(),
            }),
        ],
    };

    screen.run();
}
