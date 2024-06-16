slint::slint! {
    export component HelloWorld {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

pub fn run_ui() {
    HelloWorld::new().unwrap().run().unwrap();
}
