/*
Stack And Heap

Stack:
1.The stack stores values in the order it gets them and removes the values in the opposite order.
  LIFO(last in, first out).
2.All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time
  or a size that might change must be stored on the heap instead.

Heap:
1.The heap is less organized: when you put data on the heap, you request a certain amount of space.
2.The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use,
  and returns a pointer, which is the address of that location.


Summary:
1.Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.
2.The processor can do its job better if it works on data that’s close to other data (as it is on the stack)
  rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can also take time.


Ownership Rules
1.Each value in Rust has a variable that's called its owner.
2.There can only be one owner at a time.
3.When the owner goes out of scope, the value will be dropped.


Variable Scope
1.When s comes into scope, it is valid.
2.It remains valid until it goes out of scope.


Memory and Allocation
With the String type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap,
unknown at compile time, to hold the contents. This means:
1.The memory must be requested from the memory allocator at runtime.
2.We need a way of returning this memory to the allocator when we're done with our String.

That first part is done by us: when we call String::from, its implementation requests the memory it needs.
This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector (GC), the GC keeps track
and cleans up memory that isn’t being used anymore, and we don’t need to think about it. Without a GC,
it’s our responsibility to identify when memory is no longer being used and call code to explicitly return it,
just as we did to request it. Doing this correctly has historically been a difficult programming problem.
If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too.
We need to pair exactly one allocate with exactly one free.

Rust takes a different path: the memory is automatically returned once the variable that owns it
goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop.


Ways Variables and Data Interact: Move
--------------  s1
|name | value|
--------------              -------------
|ptr  |  ----|------------->|    heap   |
--------------              -------------
|len  |   5  |                   |
--------------                   |
|cap  |   5  |                   |
--------------                   |
                                 |
--------------  s2               |
|name | value|                   |
--------------                   |
|ptr  |  ----|-------------------|
--------------
|len  |   5  |
--------------
|cap  |   5  |
--------------


Ownership and Functions
The semantics for passing a value to a function are similar to those for assigning a value to a variable.
Passing a variable to a function will move or copy, just as assignment does.


Return Values and Scope
Returning values can also transfer ownership.


References and Borrowing
The &s syntax lets us create a reference that refers to the value of s1 but does not own it.
Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

The scope in which the variable s is valid is the same as any function parameter’s scope,
but we don’t drop what the reference points to when it goes out of scope because we don’t have ownership.
When functions have references as parameters instead of the actual values,
we won’t need to return the values in order to give back ownership,
because we never had ownership.

We call having references as function parameters borrowing.


Mutable References


*/

#![allow(unused_variables)]
fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = String::from("hello"); // s is valid from this point forward
                                       // do something with s
    } // this scope is now over, and s is no longer valid

    {
        let mut s = String::from("hello,");
        s.push_str(" world!");
        println!("{}", s);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1; // make a copy of the value in s1 and bind it to s2
                     // println!("{}, world!", s1); // borrow of moved value: `s1`
    }

    // deep clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1= {}, s2= {}", s1, s2);
    }

    // shallow clone
    {
        // The reason is that types such as integers that have a known size
        // at compile time are stored entirely on the stack,
        // so copies of the actual values are quick to make.
        let x = 5;
        let y = x;
        println!("x= {}, y= {}", x, y);
    }

    // Ownership and Functions
    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        // println!("{}, world!", s); // borrow of moved value: `s`

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it’s okay to still
                       // use x afterward
        println!("x: {}", x);
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    // Return Values and Scope
    {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1
        println!("s1: {}", s1);

        let s2 = String::from("world"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3

        // println!("s2: {}", s2); // borrow of moved value: `s2`

        println!("s3: {}", s3);
    }
    // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
    // moved, so nothing happens. s1 goes out of scope and is dropped.

    // References and Borrowing
    {
        let s = String::from("hello");
        let len = calculate_length(&s);
        println!("The length of '{}' is {}", s, len);
    }

    // Mutable References
    {
        let mut s = String::from("hello");

        change_string(&mut s);
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("some_string: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("some_integer: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    return some_string;
    // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope
    return a_string;
    // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    return s.len();
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change_string(s: &mut String) {
    s.push_str(", world!");
}
