# python_rs
Python-like syntax in Rust

### Usage:
```
comp![/*python-like list comprehension*/];
```
```
lambda!{/*python like lambda function*/};
```
```
list![/*python-like list holding items of type Rc<RefCell<dyn Any>>*/];
```
```
scoped!{
  set!(/*variable declaration*/) //This behaves similar to the python walrus operator in conjunction with scoped!{}
};
```
