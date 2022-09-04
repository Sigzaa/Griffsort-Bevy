pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        /*
        
        app
        .add_plugin(Synx::<Component>::new())
        .add_plugin(Synx::<Resource>::new())


        */
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
