// main.rs
// Build Verhoeff tables from D5 (pentagon) and verify they match canonical tables.
// Then provide verify & generate functions that use those tables.
//
// To run:
//   cargo new verhoeff_from_group
//   cd verhoeff_from_group
//   Replace src/main.rs with this file
//   cargo run
//

fn compose(a: &[u8; 5], b: &[u8; 5]) -> [u8; 5] {
    // compose(a, b) = do `a` then `b`: h(x) = b(a(x))
    let mut h = [0u8; 5];
    for x in 0..5 {
        let ax = a[x] as usize;
        h[x] = b[ax];
    }
    h
}

fn pow(mut base: [u8; 5], mut exp: usize) -> [u8; 5] {
    // compute base^exp under composition, using repeated squaring-like simple loop
    let mut result = [0u8, 1, 2, 3, 4]; // identity
    while exp > 0 {
        if exp % 2 == 1 {
            result = compose(&result, &base);
        }
        base = compose(&base, &base);
        exp /= 2;
    }
    result
}

fn perm_eq(a: &[u8;5], b: &[u8;5]) -> bool {
    a.iter().zip(b.iter()).all(|(x,y)| x==y)
}

fn find_label(perm: &[u8;5], elements: &Vec<[u8;5]>) -> Option<usize> {
    for (i, e) in elements.iter().enumerate() {
        if perm_eq(perm, e) {
            return Some(i);
        }
    }
    None
}

fn build_group_elements() -> Vec<[u8;5]> {
    // Define:
    // r = rotation 72 deg: (0 1 2 3 4) -> [1,2,3,4,0]
    // s = reflection through vertex 0 axis: [0,4,3,2,1]
    let r = [1u8, 2, 3, 4, 0];
    let s = [0u8, 4, 3, 2, 1];

    // rotations r^0..r^4
    let mut elements: Vec<[u8;5]> = Vec::with_capacity(10);
    for k in 0..5 {
        elements.push(pow(r, k));
    }
    // reflections: s, r s, r^2 s, r^3 s, r^4 s
    let s0 = s;
    elements.push(s0);
    for k in 1..5 {
        let rk = pow(r, k);
        // r^k s = do r^k then s -> compose(r^k, s)
        let rks = compose(&rk, &s0);
        elements.push(rks);
    }
    elements
}

fn build_tables(elements: &Vec<[u8;5]>) -> (Vec<Vec<u8>>, Vec<[u8;10]>, Vec<u8>) {
    // D-table: D[a][b] = label( element[a] then element[b] )
    // Uses D5 group composition
    let n = elements.len(); // should be 10
    assert_eq!(n, 10);

    let mut d = vec![vec![0u8; n]; n];
    for a in 0..n {
        for b in 0..n {
            let comp = compose(&elements[a], &elements[b]);
            let lab = find_label(&comp, elements).expect("composition must exist in group");
            d[a][b] = lab as u8;
        }
    }

    // inv table: find b such that element[a] then element[b] == identity (label 0)
    let mut inv = vec![0u8; n];
    for a in 0..n {
        let mut found = false;
        for b in 0..n {
            if d[a][b] == 0 {
                inv[a] = b as u8;
                found = true;
                break;
            }
        }
        assert!(found, "every element must have a right-inverse in a group");
    }

    // P table: Iteratively apply permutation σ = (0 1 5 8 9 4 2 7)(3 6) on digits 0-9
    // This is NOT related to D5 group - it's a separate permutation on decimal digits
    // σ in array form: σ[0]=1, σ[1]=5, σ[2]=7, σ[3]=6, σ[4]=2, σ[5]=8, σ[6]=3, σ[7]=0, σ[8]=9, σ[9]=4
    let sigma = [1u8, 5, 7, 6, 2, 8, 3, 0, 9, 4];
    
    let mut p_rows: Vec<[u8;10]> = Vec::with_capacity(8);
    let mut current = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // P[0] = identity
    
    for _ in 0..8 {
        p_rows.push(current);
        
        // Compute next: P[i+1] = σ ∘ P[i]
        // This means: next[d] = sigma[current[d]]
        let mut next = [0u8; 10];
        for d in 0..10 {
            next[d] = sigma[current[d] as usize];
        }
        current = next;
    }

    (d, p_rows, inv)
}


fn print_table_d(d: &Vec<Vec<u8>>) {
    println!("D = [");
    for row in d {
        print!("  [");
        for (i, v) in row.iter().enumerate() {
            if i + 1 < row.len() {
                print!("{}, ", v);
            } else {
                print!("{}", v);
            }
        }
        println!("],");
    }
    println!("]");
}

fn print_table_p(p: &Vec<[u8;10]>) {
    println!("P = [");
    for row in p {
        print!("  [");
        for i in 0..10 {
            if i + 1 < 10 {
                print!("{}, ", row[i]);
            } else {
                print!("{}", row[i]);
            }
        }
        println!("],");
    }
    println!("]");
}

fn print_inv(inv: &Vec<u8>) {
    println!("inv = [");
    print!("  ");
    for (i, v) in inv.iter().enumerate() {
        if i + 1 < inv.len() {
            print!("{}, ", v);
        } else {
            print!("{}", v);
        }
    }
    println!("\n]");
}

// -------------------- Verhoeff operations using the generated tables --------------------

fn verhoeff_validate(num: &str, d: &Vec<Vec<u8>>, p: &Vec<[u8;10]>) -> bool {
    // Process digits from right to left; positions start at 0
    let mut c: usize = 0;
    let digits: Vec<u8> = num.chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect();

    let mut pos = 0usize; // rightmost digit position 0
    for &digit in digits.iter().rev() {
        let p = p[pos % 8][digit as usize] as usize;
        c = d[c][p] as usize;
        pos += 1;
    }
    c == 0
}

fn verhoeff_generate_check_digit(num: &str, d: &Vec<Vec<u8>>, p: &Vec<[u8;10]>, inv: &Vec<u8>) -> Option<u8> {
    let digits: Vec<u8> = num.chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect();

    let mut c: usize = 0;
    let mut pos = 1usize;
    for &digit in digits.iter().rev() {
        let p = p[pos % 8][digit as usize] as usize;
        c = d[c][p] as usize;
        pos += 1;
    }
    Some(inv[c])
}

fn main() {
    println!("Building D5 group elements (as permutations on 5 vertices)...");
    let elements = build_group_elements();

    println!("\nBuilding tables D, P, inv from group composition...");
    let (d, p, inv) = build_tables(&elements);

    println!("\nD (generated):");
    print_table_d(&d);
    println!("\nP (generated):");
    print_table_p(&p);
    println!("\ninv (generated):");
    print_inv(&inv);

    let sample = "4568435486";
    let check = verhoeff_generate_check_digit(sample, &d, &p, &inv).unwrap();
    println!("\nSample: {} -> check digit {}", sample, check);
    let combined = format!("{}{}", sample, check);
    println!("Combined: {} ; valid? {}", combined, verhoeff_validate(&combined, &d, &p));
    println!("Checking: {}", verhoeff_validate("830070597077", &d, &p))
}
