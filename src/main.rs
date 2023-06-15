fn main() {
    // let items = vec![1, 2, 3].iter().map(|x| x+1);
    // println!("{:?}", items);

    /*
    This code doesn't work

    lets go line by line here

    - A place to hold our vector which will eventually come
    let items =

    - A temporary place in memory with [1,2,3] values arranged in a vector
    vec![1,2,3]

    - creating an iterator with reference to items in vector
    - let items: Iter<'_, i32>
    .iter()

    - Now for the crazy part..
    - Map: Map is basically a data structure which holds
        = the function to apply and
        = the data to apply it on

    - and once we start iterating over it the iterator
    - then it will start to execute the mapped function for us.
    */

    // Now the working data

    let data = vec![1, 2, 3];
    let item_mapped_iter = data.iter().map(|x| x * 2);

    // let item_mapped_iter: Map<Iter<'_, i32>, impl Fn(&i32) -> i32>

    // MAP IS JUST A MAP!!!!
    // ARE YOU FUCKING KIDDING ME!?!?!?!?!?!?

    for item in item_mapped_iter {
        println!("{item}")
    }
}
