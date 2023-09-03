/*
----> ЗАДАНИЕ 2 "Площадь квадрата"

Создать структуру Rect (квадрат), которая задается координатами левого верхнего угла и длиной стороны.
Добавить для этой структуры методы new(top_left: (f32, f32), width: f32) -> Rect
                                   bottom_right -> (f32, f32), // Выводит координаты правого нижнего угла
                                   area -> f32 // Вычисляет площадь квадрата
                                   perimeter -> f32 // Вычисляет периметр квадрата

 */

struct Rect {
    top_left: (f32, f32),
    width: f32,
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        Rect {
            top_left,
            width,
        }
    }

    fn bottom_right(&self) -> (f32, f32) {
        let (x, y) = self.top_left;
        let bottom_right_x = x + self.width;
        let bottom_right_y = y + self.width;
        (bottom_right_x, bottom_right_y)
    }

    fn area(&self) -> f32 {
        self.width * self.width
    }

    fn perimeter(&self) -> f32 {
        4.0 * self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!((6., 7.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(20., rect.perimeter())
    }
}
