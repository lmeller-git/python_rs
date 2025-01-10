#[cfg(test)]
mod tests {
    use super::*;
    use python_macros::{comp, lambda};

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
}
