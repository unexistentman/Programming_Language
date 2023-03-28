fn main() {
    println!("RAII");
    raii();
    println!("-------------------------");

    println!("ownership and moves");
    ownership_and_moves();
    println!("-------------------------");

    println!("borrowing");
    borrowing();
    println!("-------------------------");
}

fn raii() {
    fn create_box() {
        let _box1 = Box::new(3i32);
    }

    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    // Destructor
    struct ToDrop;

    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    let _x = ToDrop;
    println!("Made a ToDrop!");
}

fn ownership_and_moves() {
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);
    }

    let x = 5u32;
    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.
    let b = a;

    destroy_box(b);

    println!("mutability");

    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;

    println!("mutable_box new contains {}", mutable_box);

    println!("partial moves");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Jeong"),
        age: Box::new(20),
    };

    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    println!("The person's age from person struct is {}", person.age);
}

fn borrowing() {
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);

    println!("mutability");

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    fn borrow_book(book: &Book) {
        println!("I immutably borrowed {} - {} edition", book.title, book.year);
    }
    
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    println!("aliasing");

    struct Point { x: i32, y: i32, z: i32 }

    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!("Point has coordinates: ({}, {}, {})", 
             borrowed_point.x, another_borrow.y, point.z);

    // error! can't borrow 'point' as mutable because it's currently
    // borrowed as immutable.
    // let mutable_borrow = &mut point;

    println!("Point has coordinates: ({}, {}, {})", 
             borrowed_point.x, another_borrow.y, point.z);

    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // error! can't borrow 'point' as immutable because it's currently
    // borrowed as mutable.
    // let y = &point.y;

    // error! Can't print because 'println!' takes an immutable reference.
    // println!("Point Z coordinate is {}", point.z);

    println!("Point has coordinates: ({}, {}, {})",
             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);         
}
