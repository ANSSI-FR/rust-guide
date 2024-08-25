# Test and fuzzing

## Writing tests

Rust offers two types of test built in by default: internal tests and integration tests. 
In this section, we will discuss these two types of test as well as a rather special type of test, which is the trait implementation test.

> Recommendation {{#check TEST-DRIVEN-DEV | Adopt a test-driven development's method}}
>
> One of the best development habits is to start development by writing the set of tests to which the functionality must respond. 

### Internal

Internal tests define all the tests present in the `src/` folder of a Rust project. They have the great advantage of being able to test all the functions (even private ones) if they are placed in the same file as the project.


> ### Recommendation {{#check TEST-INTERNE | Test all functions}}
>
> It is advisable to test all the functions of your programme, even those that may seem the most trivial.
> 
> This way, if a future modification causes a side-effect that modifies the behaviour of another function, you will notice it much more quickly.
> This also helps to limit the number of bugs as early as possible. 

```rust
// private function
fn my_function(){
	... // Your code here
}

#[cfg(test)]
mod tests{
	#[test]
	fn test_my_function(){
		... // Your tests here
	}
}
```

It is also possible to ignore certain tests if they take too long to run. This can be done by adding `#[ignore]` above the test function.

```rust
#[cfg(test)]
mod tests{
	#[test]
	fn test_non_ignore(){ ... }

	#[ignore]
	#[test]
	fn test_ignore() { ... }
}
```

> Important
>
> If a test is marked as ‘ignore’, it will no longer be possible to run it even if you specify its name using the `cargo test <test_name>` command. 
> 
> The only way it can be run is to remove the `#[ignore]` above it.

> ### Recommendation {{#check TEST-IGNORE | Limit the number of ignored tests}}
>
> It is recommended to limit the number of tests that will be ignored as much as possible. 

Rust has an attribute system that allows part of the code to be compiled only when necessary. 
This makes it possible to define code that will only be compiled when a particular feature is requested. 

One of the basic features of any project is `test`. This allows you to describe code which will only be present when the code is compiled for testing (via the `cargo test` command).

To do this, add the `#[cfg(test)]` attribute to the line above the function or module concerned: 
```rust
#[cfg(test)]
mod test{

	#[test]
	fn test_1(){}
}
```

> ### Rules {{#check TEST-CFG | Wrap tests in a sub-module with the attribute `#[cfg(test)]`}}
>
> All internal tests must be wrapped in a sub-module with the `#[cfg(test)]` attribute.
> 
> Similarly, any potential functions you may develop to help these tests must also have the `#[cfg(test)]` attribute.
### Integration

> Attention
>
> This type of test is only available for crates which are libraries.

The integration tests are the set of tests in the `tests/` folder at the root of the crate. 
In this folder, each `*.rs` file will be compiled as a different crate and the library tested will be used as if an external project were using it.  

For example, if we were developing a library called `example`, we could run the following integration test: 
```rust
use example::method_name;

#[test]
fn test_api(){
	method_name();
}
```

These tests are run at the same time as all the other tests using the following command: 
```bash
cargo test
```

> ### Rule {{#check TEST-IMPL | Check that the public behaviour of the API is as expected}}
>
> Integration tests must ensure that the library's behaviour is as expected. These tests must cover all the solution's public functions (including the import of types, functions, enums, etc.).
> 
> This also ensures that the API is user-friendly.

### Implementing a trait

The example below is used to create a test to ensure that a struct or enum implements a trait.

These tests are a little unusual. If positioned in a project, they can prevent the project from compiling if they are not valid.

Here is an example of code used to ensure that an enum has the Send and Sync traits: 

```rust
enum Example {}

#[cfg(test)]
mod test{
	use super::*;
	
	fn send_sync_trait<T : Sendc + Sync>(){}
	
	#[test]
	fn test_traits_impl(){
		send_sync_trait::<Exemple>();
	}
}
```

> ### Recommendation {{#check TEST-TRAIT | Create tests to ensure that certain traits are implemented for structures/enums}}
>
> In certain contexts, it is necessary for certain struct or enum to implement particular traits. 
> In such cases, it is strongly recommended that you implement this type of test.

<!-- ## Fuzzing

### cargo-fuzz

<mark>TODO</mark>: good practices in fuzzing programs or part of programs.
--> 