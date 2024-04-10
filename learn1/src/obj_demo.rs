// 这限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表。如果只需要同质（相同类型）集合，则倾向于使用泛型和 trait bound，因为其定义会在编译时采用具体类型进行单态化。
// pub struct Screen<T: Draw> {
//   pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//   T: Draw,
// {
//   pub fn run(&self) {
//       for component in self.components.iter() {
//           component.draw();
//       }
//   }
// }
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>, //  vector 的类型是 Box<dyn Draw>，此为一个 trait 对象：它是 Box 中任何实现了 Draw trait 的类型的替身
}

impl Screen {
    fn run(&self) {
        for c in &self.components {
            c.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn run_obj_demo1() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 16,
                height: 9,
                label: String::from("按钮"),
            }),
            Box::new(SelectBox {
                width: 16,
                height: 64,
                options: vec![String::from("选项一")],
            }),
        ],
    };
    screen.run();
}

pub fn run_obj_demo() {
    run_obj_demo1();
}
