# Recommendations Checklist

## Development environment

| Name                 | Short description                                    |
|:---------------------|:-----------------------------------------------------|
| **[DENV-STABLE]**    | Use stable compilation toolchain                     |
| **[DENV-LINTER]**    | Use Rust linter (cargo-clippy)                       |

## Libraries

| Name                 | Short description                                    |
|:---------------------|:-----------------------------------------------------|
| **[LIBS-OUTDATED]**  | Check for outdated dependencies versions (cargo-outdated) |
| **[LIBS-AUDIT]**     | Check for security vulnerabilities report on dependencies (cargo-audit) |
| **[LIBS-UNSAFE]**    | Check for unsafe code in dependencies                |
| **[MEM-ZERO]**       | Zeroize memory of sensitive data after use           |

## Language

| Name                 | Short description                                    |
|:---------------------|:-----------------------------------------------------|
| **[LANG-UNSAFE]**    | Use unsafe blocks only in predefined cases and justify it |
| **[LANG-ARITH]**     | Use the appropriate arithmetic operations regarding potential overflows |
| **[LANG-ERRWRAP]**   | Implemente custom Error type, wrapping all possible errors |
| **[LANG-ERRDO]**     | Use the `?` operator and do not use the `try!` macro |
| **[LANG-NOPANIC]**   | Avoid functions that can cause `panic!`              |
| **[LANG-ARRINDEXING]** | Test properly array indexing or using the `get()` method |
| **[LANG-FFIPANIC]**  | Handle correctly `panic!` in FFI                     |

## Test and fuzzing

| Name                 | Short description                                    |
|:---------------------|:-----------------------------------------------------|
| | |
