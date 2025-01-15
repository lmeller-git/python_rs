# python_rs
Python-like syntax in Rust

##usage:
```
comp![/*python-like list comprehension*/];
```
```
lambda!{/*python like lambda function*/};
```
```
list![/*python-like list hoilding items of type Rc<RefCell<Any>>*/];
```
```
scoped!{
  set!(/*variable declaration*/) //This behaves similar to the python walrus operator in conjunction with scoped!{}
};
```
