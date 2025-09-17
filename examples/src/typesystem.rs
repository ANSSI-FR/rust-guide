mod bad {
    // ANCHOR: add_user_bad
    fn add_user(name: String, city: String) {
        // ...
    }
    // ANCHOR_END: add_user_bad
}

mod good {
    // ANCHOR: add_user_good
    struct Name(String);
    struct City(String);

    fn add_user(name: Name, city: City) {
        // ...
    }
    // ANCHOR_END: add_user_good
}
