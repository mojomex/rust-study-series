pub fn scopes() {
    let mut a = 5;

    {
        // Having multiple immutable references at once is fine:

        let b = &a;   // ----+
        let c = &a;   // --+ |
                            //   | |
        println!("{}", b);  // --+ |  // Lifetime of b ends here
        println!("{}", c);  // ----+  // Lifetime of c ends here

        // Having one mutable reference is fine:

        let d = &mut a; // --+
        *d += 1;                  //   |
        println!("{}", d);        // --+  // Lifetime of d ends here
    }

    {
        // Having multiple immutable references at once is fine:

        let b = &a;         // ----+
        let c = &mut a; // --+ |
        *c += 1;                  //   | |
        println!("{}", c);        // --+ |  // Lifetime of c ends here
        println!("{}", b);        // ----+  // Lifetime of b ends here

    }
}
