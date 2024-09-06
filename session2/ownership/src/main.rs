mod basics;
mod lifetimes;
mod dangle;
mod slices;

fn main() {
    // The Ownership Rules:
    // - Each value in Rust has an owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    basics::basics();
    lifetimes::scopes();
    dangle::dangle();
    slices::slices();
}
