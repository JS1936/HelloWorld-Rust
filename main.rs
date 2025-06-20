//! This is a doc comment.
/// This is also a doc comment.

fn main()
{
    // This is a line comment.
    println!("Hello, world!");
    iterate();
}

fn iterate()
{
    /*
     * Example block comment
     */
    let items = vec![1,2,3,4,5];
    for i in items.iter() {
        println!("{}", i);
    }
}

