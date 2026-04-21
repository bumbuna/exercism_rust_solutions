/// What should the type of function be?
pub fn map<T,U, F>(input: Vec<T>, mut function: F ) -> Vec<U> where
    F: FnMut(T) -> U {
    let mut v = vec!();
    for i in input {
        v.push(function(i));
    }
    v
}
