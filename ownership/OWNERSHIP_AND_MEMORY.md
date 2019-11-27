# OWNERSHIP
Rust doesn't have garbage collection, because of it's way of handling ownership.
* Each value in Rust has a variable called it's owner.
* There can be only one owner at a time.
* When the owner goes out of scope the value is dropped.

# THE STACK AND THE HEAP
Rust has two places where to store data: stack and heap.

### Stack
The stack can be imagined as a stack of plates. The data stored in the stack is piled on top of each other, the latest always on the top. Thus the last one inserted in the stack will also be the first one to be pulled out. "Last in, first out."
* Data in the stack always has to have exact size
* Stack is faster to add and pull data from

### Heap
The heap is more like a restaurant. The data is of an unknown size and is allocated where ever the operating system finds space from.