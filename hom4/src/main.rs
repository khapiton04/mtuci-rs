struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Vector { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, Default::default());
    }
}
fn main() {
    let mut vec = Vector::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec.pop()); // Выводит: Some(3)

    println!("{:?}", vec.remove(0)); // Выводит: Some(1)

    println!("{:?}", vec.get(0)); // Выводит: Some(&2)

    vec.resize(5);

    for i in 0..5 {
        println!("{:?}", vec.get(i)); // Выводит: Some(&2), None, None, None, None
    }
}
