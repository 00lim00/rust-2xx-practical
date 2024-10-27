struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    // Обчислює площу прямокутника
    fn area(&self) -> i32 {
        (self.b.x - self.a.x).abs() * (self.a.y - self.b.y).abs()
    }

    // Обчислює площу перетину з іншим прямокутником
    fn overlap(&self, other: &Rectangle) -> i32 {
        let x_overlap = (self.b.x.min(other.b.x) - self.a.x.max(other.a.x)).max(0);
        let y_overlap = (self.a.y.min(other.a.y) - self.b.y.max(other.b.y)).max(0);
        x_overlap * y_overlap
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlap_area = 0;

    for i in 0..xs.len() {
        total_area += xs[i].area();

        for j in (i + 1)..xs.len() {
            overlap_area += xs[i].overlap(&xs[j]);
        }
    }

    total_area - overlap_area
}

// Test data
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

// Test function
#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
