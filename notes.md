# Stateful components

A child is a un-stateful component  
If you make #[derive(IntoElement)] component, it's unstateful,and you need to use RenderOnce and RenderOnce is RenderOnce(self, ...)  


we need to implement Render trait, which allows to mutate self
```rust
struct MyComponent {
    value: Rc<RefCell<i32>>,
}

impl RenderOnce for MyComponent {
    fn render(&self) ...  {
        on_click(|_| {
            *self.value.borrow_mut() += 1;
        })
    }
}
```

render() {
    div().child(MyComponent { value: Rc::new(RefCell::new(0)) })
}

***

Pointers: https://github.com/MatthiasGrandl/Loungy