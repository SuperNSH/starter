# Add a service

A service is anything outside the app's own memory: a backend, a file, a system API that draws nothing. It goes in `src/services/`.

1. **Make `src/services/<name>.rs`** and list it in `src/services/mod.rs`.

2. **Return `Result<T, AppError>`** (`crate::error::AppError`) with a stable `code` (`"network"`, `"storage"`, `"invalid"`), so one error type flows through the app. A page shows a failure with `store::errors::report_error(error)`.

3. **If a real library will replace it, make it a seam:** one small trait, one local implementation, one function that hands out the shared instance. `src/services/auth.rs` is the model. Tests then never need the real thing.

4. **Read the clock through `services::time`**, never `std::time`: tests run on a manual clock.

5. **If it reaches outside the sandbox** (a host, the file system), declare the capability in `nsh.json` and read `nsh build`'s output: a misspelled capability is a warning, not an error.

6. **Do not import `store`, `ui`, `pages` or `shell`.** A service may use `error` and other services.

7. **Check:** `just rules && just test`.
