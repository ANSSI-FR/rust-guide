# Recommendations Checklist

## Development environment

| Name                 | Short description                                    |
|:---------------------|:-----------------------------------------------------|
| **[DENV-STABLE]**    | Using stable compilation toolchain                   |
| **[DENV-CLIPPY]**    | Using Rust linter (cargo-clippy)                     |

## Libraries

| Name                 | Short description                                    |
|:---------------------|:-----------------------------------------------------|
| **[LIBS-OUTDATED]**  | Checked for outdated dependencies versions (cargo-outdated) |
| **[LIBS-AUDIT]**     | Checked for security vulnerabilities report on dependencies (cargo-audit) |
| **[LIBS-UNSAFE]**    | Checked for unsafe code in dependencies              |

## Language

| Name                 | Short description                                    |
|:---------------------|:-----------------------------------------------------|
| **[LANG-UNSAFE]**    | Using unsafe blocks only in predefined cases and justify it |
| **[LANG-ARITH]**     | Using the appropriate arithmetic operations regarding potential overflows |
| **[LANG-ERRWRAP]**   | Implementing custom Error type, wrapping all possible errors |
| **[LANG-ERRDO]**     | Using the `?` operator and do not use the `try!` macro |
| **[LANG-NOPANIC]**   | Avoiding functions that can cause `panic!`           |
| **[LANG-ARRINDEXING]** | Testing properly array indexing or using the `get()` method |
| **[LANG-FFIPANIC]**  | Handling correctly `panic!` in FFI                   |

## Test and fuzzing

| Name                 | Short description                                    |
|:---------------------|:-----------------------------------------------------|
| | |
