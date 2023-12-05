// ПРИМЕР СТРУКТУРЫ 
struct Person  {
    name: String,   // имя
    age: u8,        // возраст
    height: f32     // рост
}


// В Rust нет классического насследования. Вместо него есть механизм под названием traits.
//  ПРИМЕР ИСПОЛЬЗОВАНИЯ TRAITS (трейтов)
trait Shape {
    // У любой фрормы можно посчитать площадь.
    fn area(&self) -> f32;
}

trait HasAngles: Shape {
    // У любой фигуры с углами можно посчитать количество углов.
    fn angles_count(&self) -> i32;
}

struct Rectangle {
    x: f32,
    y: f32,
}

// Прямоугольник является формой.
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.x * self.y
    }
}

// Прямоугольник является фигурой с углами.
impl HasAngles for Rectangle {
    fn angles_count(&self) -> i32 {
        4
    }
}

struct Circle {
    r: f32,
}

// Круг является формой
impl Shape for Circle {
    fn area(&self) -> f32 {
        self.r.powi(2) * 3.14
    }
}


// ПРИМЕР СТАТИЧЕСКОГО ПОЛИМОРФИЗМА
// Принимаем что угодно, реализующее трейт Shape.
fn areas_sum_stat(shape1: impl Shape, shape2: impl Shape) -> f32 {
    shape1.area() + shape2.area()
}

fn foo_stat(rectangle: Rectangle, circle: Circle) {
    // Можем передать две разные фигуры.
    let sum = areas_sum_stat(rectangle, circle);
}

// ПРИМЕР ДИНАМИЧЕСКОГО ПОЛИМОРФИЗМА
// Принимаем что угодно, реализующее трейт Shape.
// В этот раз принимаем не сами объекты, а ссылки на них,
// так как не зная конкретный тип объекта, мы не знаем и его размер,
// а следовательно, не сможем выделить для него место на стеке.
fn areas_sum_dynamic(shape1: &dyn Shape, shape2: &dyn Shape) -> f32 {
    shape1.area() + shape2.area()
}

fn foo_dynamic(rectangle: Rectangle, circle: Circle) {
    // Можем передать ссылки на две разные фигуры.
    let sum = areas_sum_dynamic(&rectangle, &circle);
}


fn main(){
    let tom = Person{
        name: "Tom".to_string(),
        age: 36,
        height: 1.78
    };
     
    println!("name = {}", tom.name);
    println!("age = {}", tom.age);
    println!("height = {}", tom.height);
}
