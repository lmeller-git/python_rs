#[cfg(test)]
mod tests {
    use std::{any::Any, cell::RefCell, rc::Rc};

    //use super::*;
    use python_macros::{comp, lambda, list, walrus};

    #[test]
    fn it_works() {
        let result = comp![x for x in [1, 2, 3]].collect::<Vec<_>>();
        let list = Vec::from([(1, 2), (2, 2), (3, 3)]);
        let res2 = comp![x * y for (x, y) in list if x * y > 2 && x < 3].collect::<Vec<_>>();
        assert_eq!(result, [1, 2, 3]);
        assert_eq!(res2, [4]);
    }
    #[test]
    fn lambda_fn() {
        let add = lambda! {lambda x, y, z: x + y + z  if x == 1 else 5};
        assert_eq!(add(1, 2, 3), 6);
    }

    #[test]
    fn test_combined() {
        //let add = lambda! {lambda x, y: x + y if x < y else y};
        let list = Vec::from([(1, 2), (2, 1), (3, 3), (1, 5)]);
        let res1 = comp![lambda! {lambda x, y: x + y if x < y else y}(x, y) for (x, y) in list]
            .collect::<Vec<_>>();
        assert_eq!(res1, [3, 1, 3, 6]);
    }

    #[test]
    fn test_large() {
        let list = Vec::from([vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        let res1 = comp![
            comp![
                lambda!{lambda x: x * 2 if x % 2 == 0 else 10 if x < 1000 else 0}(x) for x in l
            ].sum::<i32>() for l in list
        ]
        .sum::<i32>();
        assert_eq!(res1, 74);
    }

    #[test]
    fn test_multi_comp() {
        let list = vec![vec![1, 2, 3]; 3];
        let res: Vec<_> = comp![x for v in list if v.len() <= 3 for x in v if x > 1 ].collect();
        assert_eq!(res, vec![2, 3, 2, 3, 2, 3]);
    }

    #[test]
    fn test_list() {
        //let l: Vec<Rc<RefCell<dyn Any>>> =
        /*
        let l = list![
            1,
            "hello".to_string(),
            comp![x for x in [1, 2, 3] if x % 2 != 0].collect::<Vec<_>>()
        ];
        */
        let l = list![
            comp![x for x in [1, 2, 3]].collect::<Vec<i32>>(),
            1 + 2,
            "hello_world".to_string(),
            lambda! {lambda x, y: x * y if x > 0 else y}(0, 5),
        ];
        let expected: Vec<Rc<RefCell<dyn Any>>> = vec![
            Rc::new(RefCell::new(
                comp![x for x in [1, 2, 3]].collect::<Vec<i32>>(),
            )),
            Rc::new(RefCell::new(3)),
            Rc::new(RefCell::new("hello_world".to_string())),
            Rc::new(RefCell::new(5)),
        ];

        assert_eq!(l.len(), expected.len());
        for (item, expected_item) in l.iter().zip(expected.iter()) {
            let item = item.borrow();
            let expected_item = expected_item.borrow();

            if let Some(item) = item.downcast_ref::<i32>() {
                assert_eq!(item, expected_item.downcast_ref::<i32>().unwrap());
            } else if let Some(item) = item.downcast_ref::<String>() {
                assert_eq!(item, expected_item.downcast_ref::<String>().unwrap());
            } else if let Some(item) = item.downcast_ref::<Vec<i32>>() {
                assert_eq!(item, expected_item.downcast_ref::<Vec<i32>>().unwrap());
            } else {
                panic!("Unexpected type in list!");
            }
        }
    }
    /*
    #[test]
    fn test_walrus() {
        assert_eq!(walrus!(x = 5), 5)
    }

    #[test]
    fn list_walrus() {
        let l = list![
            walrus!(x = vec![1, 2, 3]),
            comp![i for i in x if i % 2 == 0].collect::<Vec<_>>(),
        ];
        let expectded = vec![
            Rc::new(RefCell::new(walrus!(x = vec![1, 2, 3]))),
            Rc::new(RefCell::new(vec![2])),
        ];
    }
    */
}
