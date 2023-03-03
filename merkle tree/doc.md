# Merkle Tree


Here merkle proof uses `PhantomData`
Why?

PhantomData is a type of data that appears to exist, but actually does not. This term is often used in computer programming, especially in Rust programming language.

In Rust, when a type implements the Drop trait, it means that when the variable of that type goes out of scope, some code will be executed to clean up resources associated with that variable. However, sometimes the Rust compiler needs to know the size of a type at compile time in order to allocate the right amount of memory for it on the stack. If a type has a Drop implementation, the size of the type cannot be known until runtime, because the size of the type might depend on the specific resources that need to be cleaned up.

To solve this problem, Rust introduces a special type called PhantomData<T>. This type does not actually hold any data, but it has a size of 0. By using PhantomData<T> as a field in a type that implements Drop, the Rust compiler can determine the size of the type at compile time, because the size of PhantomData<T> is always 0. This allows the type to be allocated on the stack with the correct size, even though the size of the type might depend on resources that are only known at runtime.

In summary, PhantomData is a Rust language construct used to indicate the presence of a type parameter that does not actually have any data associated with it, but is used to help the Rust compiler reason about the layout and size of a type.