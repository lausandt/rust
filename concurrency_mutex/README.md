Shared-State Concurrency using mutex locks

Mutex is an abbreviation for mutual exclusion. A mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. 

The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. Therefore, the mutex is described as guarding the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to remember two rules:

 - You must acquire the lock before using the data.
 - When you’re done with the data you must release the acquired lock.

The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. Invoking clone on Arc produces a new Arc instance, which points to the same allocation on the heap as the source Arc, while increasing a reference count. When the last Arc pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as "inner value") is also dropped.

Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot generally obtain a mutable reference to something inside an Arc. You would need the mutex for that.

```
let counter = Arc::new(Mutex::new(0));
```
Atomic types also provide options for shared memory concurrency 

https://doc.rust-lang.org/std/sync/atomic/index.html 

Two concurrency concepts are embedded in the language and not in a library: the std::marker traits Sync and Send.

Sync is "thread-safe", i.e. that a particular piece of data can be safely used by multiple concurrent threads. The reason for having separate Send and Sync traits is that a type can sometimes be one, or both, or neither. For example:

 - The smart pointer Rc<T> is also neither Send nor Sync, for reasons described above.
 - The RefCell<T> type and the family of related Cell<T> types are Send (if T: Send), but they are not Sync. A RefCell can be sent across a thread boundary, but not accessed concurrently because the implementation of borrow checking that RefCell<T> does at runtime is not thread-safe.
 - The smart pointer Mutex<T> is Send and Sync, and can be used to share access with multiple threads 
 - The type MutexGuard<'a, T> that is returned by Mutex::lock is Sync (if T: Sync) but not Send. It is specifically not Send because some platforms mandate that mutexes are unlocked by the same thread that locked them.
