pub fn dangle() -> &i32 {
  let a = 5;
  &a // You cannot return references to stack memory (as it will get deallocated once this function ends)
}