Testing:
Flag for Tracing
$env:MIRIFLAGS="-Zmiri-disable-isolation" 
cargo +nightly miri test 
Miri since uses unsafe