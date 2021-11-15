// a trait to implement for a gui object
pub trait Draw {
    fn draw(&self);
}

// a struct to represent a gui
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // a function to draw all of a screens objects
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// a struct to store a button
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    // implement the gui object trait
    fn draw(&self) {
        println!(
            "Button {}, {}, is labelled {}.",
            self.width, self.height, self.label
        );
    }
}
