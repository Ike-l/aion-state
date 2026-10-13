# Aion State

`aion-state` keeps track of values in a registry and coordinates who may use them. For example, an application could store a report under `report-42`, let one user own it, allow shared reads, and reject a write while readers are using it.

The crate manages that bookkeeping. **Your application supplies the actual storage types and defines what its access modes mean.** There is no built-in `HashMap` registry to construct directly.

## Contents

- [Why I Built This](#why-i-built-this)
- [A Concrete Example](#a-concrete-example)
- [Reading the Report](#reading-the-report)
- [What the Registry Tracks](#what-the-registry-tracks)
- [Defining Access](#defining-access)
- [Operations](#operations)
- [Features](#features)
- [Build and Run](#build-and-run)
- [Tests](#tests)
- [Test One Feature at a Time](#test-one-feature-at-a-time)

## Why I Built This

I built `aion-state` for a game engine where mods and plugins are first-class parts of the game. Two plugins may want to use or change the same resource: for example, one plugin may update a world object while another is reading it. Ownership makes it clear which plugin is responsible for a resource. Reservations let a plugin announce the access it plans to take, so the registry can detect a conflicting request before granting access. Together with shared and exclusive access modes, these rules are intended to catch mistakes and make plugin interactions more predictable.

The credentials, passwords, whitelist, and blacklist are tools for coordinating access. They may look like a general-purpose security system, but **this crate is not a sandbox for hostile plugins** and does not isolate one plugin's code from another. The application supplies credential storage and decides how credentials are checked; the crate does not provide password hashing or a security boundary. Use the game engine's sandboxing and trust controls to protect against malicious code.

## A Concrete Example

Imagine Alice owns `budget.csv`. She replaces its contents, then allows shared reads. The names beginning with `App` below are types your application supplies; the example shows the registry calls and their inputs.

First, choose the seven storage types that make up your registry:

```rust,ignore
use aion_state::prelude::Registry;

type AppRegistry = Registry<
    AppResourceStorage,
    AppReservationStorage,
    AppAccessStorage,
    AppCredentialStorage,
    AppWhitelistStorage,
    AppBlacklistStorage,
    AppControlStorage,
>;

let registry = AppRegistry::new(resource_storage, blacklist_storage);
```

`AppResourceStorage` stores the file value. The other stores track active access, reservations, users, ownership, and access rules. They must implement the crate's storage traits. The constructor takes the resource and blacklist stores; the other stores are created with `Default`.

Now register Alice, give her ownership of the report ID, and store a value there:

```rust,ignore
use aion_state::prelude::{
    RegistryAllow, RegistryOwn, RegistryRegister, RegistryReplacement,
};

let alice = AppUserId::from("alice");
let alice_password = AppPassword::new("secret");
let report = AppResourceId::from("budget.csv");

registry.register(RegistryRegister {
    id: alice.clone(),
    password: alice_password.clone(),
});

registry.own(RegistryOwn {
    id: alice.clone(),
    password: alice_password.clone(),
    resource_id: report.clone(),
});

registry.checked_replace(RegistryReplacement {
    user_details: Some((&alice, &alice_password)),
    access: &AppAccess::Replace,
    resource_id: report.clone(),
    resource: Some(AppReport::from("Q4 budget")),
    password: None,
});

registry.allow_whitelist(RegistryAllow {
    id: alice.clone(),
    password: alice_password.clone(),
    resource_id: report.clone(),
    access: AppAccess::Shared(1),
});
```

The whitelist entry allows that access mode for non-owners of this resource; it is not a per-user list. Alice, as owner, can use her resource without an allowance. An application can instead use a blacklist entry when access should require a matching password.

Each operation returns a result enum. Check the result before relying on a registration, ownership change, replacement, or allowance. The input structs make each operation's arguments explicit; result enums let callers distinguish cases such as a missing resource, denied access, or a reservation conflict.

### Reading the Report

An application implements `AppAccessResult` to describe how a successful shared or exclusive access returns its value. The registry records each successful acquisition. The caller must release that record after it has stopped using the returned references:

```rust,ignore
use aion_state::prelude::{RegistryAcquireAccess, RegistryReleaseAccess};

let access = AppAccess::Shared(1);
let result = registry.acquire_access::<AppAccessResult>(RegistryAcquireAccess {
    user_details: None,
    resource_id: report.clone(),
    access: access.clone(),
    password: None,
});

if let Ok(value) = result {
    read_report(&value);
    drop(value); // Stop using the borrowed report before releasing its access.

    unsafe {
        registry.release_access(&RegistryReleaseAccess {
            resource_id: &report,
            access: &access,
        });
    }
}
```

The exact value returned by `AppAccessResult` depends on your storage and `Accessor` implementations. With the `releaser` feature, the registry can instead return a guard that releases the recorded access when dropped.

## What the Registry Tracks

| Storage trait | In plain terms |
| --- | --- |
| `RegistryStorage` | The values and their IDs. |
| `AccessStorage` | Which access mode is currently recorded for each ID. |
| `ReservationStorage` | Access requests waiting on an ID. |
| `CredentialStorage` | Which users are registered and whether their passwords match. |
| `ControlStorage` | Which user owns each ID. |
| `WhitelistStorage` | Which access modes are allowed for an owned ID. |
| `BlacklistStorage` | Which access modes require a matching password for an owned ID. |

These stores must agree on their ID and access types. The crate's trait bounds enforce those relationships at compile time. The application chooses the concrete stores, which could use an in-memory collection or another backing system.

### Defining Access

Your `Accessor` implementation gives each access mode its rules. It answers questions such as:

- Can a new reader enter while another reader is active?
- Can a writer enter while readers are active?
- Can this mode insert or remove a value?
- What value/reference does a successful access return?
- How should concurrent access counts be combined and released?

For example, an application might define `Shared(1)` as one reader and `Replace` as an operation that can insert, replace, or remove a value. Those meanings come from the application's `Accessor`; they are not built into the registry.

The application also implements `AccessorResult` for the result of an acquisition, and `StoreValue` if the type held by its storage differs from the value exposed through the registry.

## Operations

The main synchronous methods on `Registry` are:

- Users and ownership: `register`, `unregister`, `update_password`, `own`, `is_owned`, `check_owner`, `release_resource`, `release_resource_all`.
- Access rules: `allow_whitelist`, `unallow_whitelist`, `allow_blacklist`, `unallow_blacklist`, `check_access`.
- Waiting and active use: `reserve`, `unreserve`, `drain_reservations`, `acquire_access`, `acquire_view`, `release_access`, `get_access`.
- Values and inspection: `checked_replace`, `reallocating_replace`, `contains_resource`, `len`, `keys`, `registered`.

`checked_replace` can refuse an insert/remove if the storage reports that it would need to reallocate. `reallocating_replace` is for storage whose owned value implements `StableDeref`, such as `Box<T>`. It is available only when the storage type meets that bound.

## Features

By default, Cargo enables `sync`, `releaser`, `notifier`, and `async`. The `stable_deref` feature is off by default and only changes the brute test storage.

| Feature | What changes |
| --- | --- |
| `sync` | The `Registry` name in the prelude refers to `SynchronisedRegistry`, which locks registry operations. Without it, `Registry` refers to `UnsynchronisedRegistry`; callers must coordinate access themselves when sharing it across threads. |
| `releaser` | Adds `Releaser` and `ViewReleaser` traits. Their guards release a single access or every access in a view when dropped. |
| `notifier` | Adds waiters that can retry an acquisition after a conflicting access is released. Use `sync` when you want this on the `Registry` alias; notifier support is implemented for `SynchronisedRegistry`. |
| `async` | Adds async methods to `SynchronisedRegistry` and enables Tokio's `sync` dependency. It does not turn on `sync` or change the `Registry` alias. |
| `stable_deref` | Changes the brute integration test's stored value from `StoredResource` to `Box<Resource>`, allowing that test to call `reallocating_replace`. It does not change the library API. |

The convenience types provided by combinations are:

| Enable these features | You also get |
| --- | --- |
| `releaser`, `notifier` | `NotifiedReleaser` |
| `async`, `releaser` | `AsyncReleaser` and `AsyncViewReleaser` |
| `async`, `notifier` | `AsyncNotifier` and async wait/retry helpers |
| `async`, `notifier`, `releaser` | `AsyncNotifiedReleaser` |

`sync` is independent of these combinations. For example, async methods exist on `SynchronisedRegistry` when `async` is enabled, but the shorter `Registry` name selects that type only when `sync` is enabled too.

## Build and Run

From this crate's directory:

```sh
cargo build
cargo check
```

To compile with every optional feature off:

```sh
cargo check --no-default-features
```

To use the synchronized registry without the other default features:

```sh
cargo check --no-default-features --features sync
```

Feature names can be combined with commas:

```sh
cargo check --no-default-features --features sync,async,releaser
```

The library sends structured events through `tracing` but does not install a log subscriber. An application can install one and choose how much registry detail to display; trace-level output can be very verbose.

## Tests

Run the full test suite with the default features:

```sh
cargo test
```

Run only the randomized brute test:

```sh
cargo test --test integration_test brute::run
```

The brute test sends randomly generated operations through the synchronous registry and checks the results against a model of users, values, ownership, and access. It also generates commands for each enabled feature: automatic release, notifier wakeups, async registry operations, and stable-deref replacement. The test asserts that every command enabled in that build was exercised.

To make the brute test exercise the `reallocating_replace` path, enable the test-only storage switch:

```sh
cargo test --features stable_deref
```

Without `stable_deref`, the corresponding brute command uses checked replacement instead. The full integration test harness requires `sync`, so use this command—not plain `cargo test --no-default-features`—to run the base suite with only `sync`:

```sh
cargo test --no-default-features --features sync
```

### Test One Feature at a Time

These commands run the full integration suite with each optional feature selected by itself. `sync` stays enabled because the test harness uses `SynchronisedRegistry`.

```sh
cargo test --no-default-features --features sync,releaser
cargo test --no-default-features --features sync,notifier
cargo test --no-default-features --features sync,async
cargo test --no-default-features --features sync,stable_deref
```

Focused API tests live in `tests/feature_tests/`, grouped by feature. Direct tests of the Brute feature commands live in `tests/brute/feature_commands.rs`; the randomized multi-command run remains in `tests/brute/mod.rs`.

Run a focused test by its distinctive name:

| Feature or combination | Test name |
| --- | --- |
| `releaser` access guard | `releaser_access_guard_releases_when_dropped` |
| `releaser` view guard | `releaser_view_guard_releases_every_id_when_dropped` |
| `notifier` retry after release | `notifier_future_retries_after_the_current_access_is_released` |
| `async` access | `async_registry_can_store_acquire_and_release_a_resource` |
| `async` view | `async_registry_can_acquire_a_view` |
| `stable_deref` replacement | `stable_deref_storage_allows_reallocating_replacement` |
| `notifier` + `releaser` | `released_view_deaccesses_and_notifies_on_drop` |
| `async` + `releaser` | `async_released_view_deaccesses_on_drop` |
| `async` + `notifier` | `async_notifier_retries_acquisition_after_release` |
| `async` + `notifier` + `releaser` | `async_notified_releaser_resumes_after_guard_drop` |

For example, run the notifier retry test with:

```sh
cargo test --test integration_test notifier_future_retries_after_the_current_access_is_released
```

The default `cargo test` command also runs the feature-combination tests with all default features enabled. Adding `--features stable_deref` runs that suite with stable-deref storage too.

These commands check useful feature combinations without running the integration tests:

```sh
cargo check --no-default-features
cargo check --no-default-features --features sync
cargo check --no-default-features --features releaser
cargo check --no-default-features --features notifier
cargo check --no-default-features --features async
cargo check --no-default-features --features async,releaser
cargo check --no-default-features --features async,notifier
cargo check --no-default-features --features notifier,releaser
cargo check --no-default-features --features async,notifier,releaser
```

If `cargo-hack` is installed, it can check the feature combinations automatically:

```sh
cargo hack check --feature-powerset
```

The registry uses unsafe code for parts of the access lifecycle. With a nightly toolchain and Miri installed, run:

```sh
cargo +nightly miri test
```

On PowerShell, if Miri needs isolation disabled:

```powershell
$env:MIRIFLAGS = "-Zmiri-disable-isolation"
cargo +nightly miri test
```
