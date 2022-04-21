// Can be run with `cargo test`

// Example tests to base our own off of later

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_good_room() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
       // let mut test_rooms = Vec::<MapItemData>::new();
    }

    // Lets us use ? to return quicker when failing
    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}