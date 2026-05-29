Testing:
Flag for Tracing
$env:MIRIFLAGS="-Zmiri-disable-isolation" 
cargo +nightly miri test 
Miri since uses unsafe

# Aion-State

* Tracks Accesses
* Enables Ownership of resources
    * Enables Owners to restrict accesses
    * Enables Owners to lock resources
* Keeps reservations which should guarantee a successful access in the future
* Multi-thread safe
* Each resource can be borrowed uniquely, `Accessor` should be built to adhere to rust's aliasing rules
* Using a sync primitive allows for atomic batch operations
* Tracing shows fine grain steps through the program- making it extremely verbose but what can you do :shrug:
* Storage containers are user provided via traits
    * Should be trivially implemented by HashMap(s) or Vec<(key, value)>
* Functions are "intelligently" handled to share common associated types to "minimise" parameter count 
    * sorry there is still gonna be a lot

### Future
* Want to track the reserver to limit who can "unreserve"
    * either a layer beneath Door, or a layer in reservations
* Want to exhaustively test like TigerBeetle
* Want to give a default implementation (will need for testing anyways)
* Want a tokio feature flag to transform the entire thing into async-aware
* Already have loom integration using feature flag but no tests 
* Want Miri testing 

### Future Considerations
* Passing a "config" associated type- like in `PasswordStorage`
* Allow locking of *all* resources- Processor can guarantee no background threads can block

## Motivation
This is a helper crate for my big project `Aion`. 
This will feed directly to `Aion-Program`


## Noticable Design Patterns
- Inputs and Results are concrete types. This hopefully will make extending 
- Results contain nested results. This should make the result explainable

- Each struct in the "hierarchy" is responsible for 2 things, and 2 things only, unless it 
interfaces a Storage trait. This should make each function simple and easily testable

- Storages by generics- each should be almost trivially implemented by a HashMap but maybe would work for a DB (when tokio flag?)

- Results do NOT use associated types for returns.

- Stays away from 'toggle' booleans on functions


## FAQ
- Q: Why is are results so verbose
    - A: I like always being able to match exhaustively on all "choices" the program made during a function call


