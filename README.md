Testing:
Flag for Tracing
$env:MIRIFLAGS="-Zmiri-disable-isolation" 
cargo +nightly miri test 
Miri since uses unsafe

TODO:
parameters -> struct Parameters { required, optional: Default }
+Owner locks resource, generates passwords (has access permissions) for reservations
+Synchroniser above registry
+Spawn span in each function

/*
AccessParameter { 
            resource_id, 
            access 
        }: AccessParameter<'_, ResourceId, Access>
*/


trying pattern where input has the logic?