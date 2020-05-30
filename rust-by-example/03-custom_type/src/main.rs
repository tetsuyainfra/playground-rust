#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
// ユニット
struct Nil;

// A tuple struct
// タプル
struct Pair(i32, f32);

// A struct with two fields
// 2つのフィールドを持つ（クラシックな）構造体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
// 構造体は他の構造体のフィールドになることができる
// #[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point corrdinates: ({x}, {y})", x=point.x, y=point.y);

    let bottom_right = Point { x:5.2, ..point };
    println!("bottom_right corrdinates: ({x}, {y})", x=bottom_right.x, y=bottom_right.y);

    let Point { x: top_edge, y: left_edge } = point;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };

    let _nil = Nil;
    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);


    // 演習

    impl Rectangle {
        fn rect_area (&self) -> f32{
            let Rectangle{top_left, bottom_right} = self;
            return (
              (top_left.x - bottom_right.x).abs()
              *
              (top_left.y - bottom_right.y).abs()
            )

            // (top_left.x - bottom_right.x).abs() * (top_left.y - bottom_right.y).abs()
        }
    }
    println!("Rectangle.rect_area: {}", _rectangle.rect_area());

    fn square(point: Point, width: f32, height: f32) -> Rectangle {
        let x1 = point.x;
        let y1 = point.y;
        let x2 = x1 + width;
        let y2 = y1 + height;
        Rectangle {
            top_left:       Point { x: x1, y: y2 },
            bottom_right:   Point { x: x2, y: y1 }
        }
    }

    let rect = square(Point{x: 2.0, y: 3.0}, 10f32, 5f32);
    println!("rect: {:?}", rect);

}