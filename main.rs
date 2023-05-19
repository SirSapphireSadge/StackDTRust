#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
#sst}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

// Example usage:
fn main() {
    let mut stack: Stack<i32> = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Is stack empty? {}", stack.is_empty()); // Output: false

    println!("Stack peek: {:?}", stack.peek()); // Output: Some(3)

    println!("Popped value: {:?}", stack.pop()); // Output: Some(3)

    println!("Popped value: {:?}", stack.pop()); // Output: Some(2)

    println!("Popped value: {:?}", stack.pop()); // Output: Some(1)

    println!("Is stack empty? {}", stack.is_empty()); // Output: true
}
