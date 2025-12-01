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

    let mut D = vec![vec![0u8; n]; n];
    for a in 0..n {
        for b in 0..n {
            let comp = compose(&elements[a], &elements[b]);
            let lab = find_label(&comp, elements).expect("composition must exist in group");
            D[a][b] = lab as u8;
        }
    }

    // inv table: find b such that element[a] then element[b] == identity (label 0)
    let mut inv = vec![0u8; n];
    for a in 0..n {
        let mut found = false;
        for b in 0..n {
            if D[a][b] == 0 {
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
    
    let mut P_rows: Vec<[u8;10]> = Vec::with_capacity(8);
    let mut current = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // P[0] = identity
    
    for _ in 0..8 {
        P_rows.push(current);
        
        // Compute next: P[i+1] = σ ∘ P[i]
        // This means: next[d] = sigma[current[d]]
        let mut next = [0u8; 10];
        for d in 0..10 {
            next[d] = sigma[current[d] as usize];
        }
        current = next;
    }

    (D, P_rows, inv)
}


fn print_table_d(D: &Vec<Vec<u8>>) {
    println!("D = [");
    for row in D {
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

fn print_table_p(P: &Vec<[u8;10]>) {
    println!("P = [");
    for row in P {
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

// STANDARD canonical Verhoeff tables (as seen earlier) for comparison:
fn standard_tables() -> (Vec<Vec<u8>>, Vec<[u8;10]>, Vec<u8>) {
    let D = vec![
        vec![0,1,2,3,4,5,6,7,8,9],
        vec![1,2,3,4,0,6,7,8,9,5],
        vec![2,3,4,0,1,7,8,9,5,6],
        vec![3,4,0,1,2,8,9,5,6,7],
        vec![4,0,1,2,3,9,5,6,7,8],
        vec![5,9,8,7,6,0,4,3,2,1],
        vec![6,5,9,8,7,1,0,4,3,2],
        vec![7,6,5,9,8,2,1,0,4,3],
        vec![8,7,6,5,9,3,2,1,0,4],
        vec![9,8,7,6,5,4,3,2,1,0],
    ];
    let P = vec![
        [0,1,2,3,4,5,6,7,8,9],
        [1,5,7,6,2,8,3,0,9,4],
        [5,8,0,3,7,9,6,1,4,2],
        [8,9,1,6,0,4,3,5,2,7],
        [9,4,5,3,1,2,6,8,7,0],
        [4,2,8,6,5,7,3,9,0,1],
        [2,7,9,3,8,0,6,4,1,5],
        [7,0,4,6,9,1,3,2,5,8],
    ];
    let inv = vec![0,4,3,2,1,5,6,7,8,9];
    (D, P, inv)
}

fn compare_and_print(D: &Vec<Vec<u8>>, P: &Vec<[u8;10]>, inv: &Vec<u8>) {
    let (stdD, stdP, stdinv) = standard_tables();

    let mut ok = true;
    if D.len() != stdD.len() || D[0].len() != stdD[0].len() {
        println!("D-table size mismatch.");
        ok = false;
    } else {
        for i in 0..D.len() {
            for j in 0..D[i].len() {
                if D[i][j] != stdD[i][j] {
                    println!("D mismatch at [{},{}]: got {} expected {}", i, j, D[i][j], stdD[i][j]);
                    ok = false;
                }
            }
        }
    }

    if P.len() != stdP.len() {
        println!("P-table row count mismatch.");
        ok = false;
    } else {
        for i in 0..P.len() {
            for j in 0..10 {
                if P[i][j] != stdP[i][j] {
                    println!("P mismatch at row {}, col {}: got {} expected {}", i, j, P[i][j], stdP[i][j]);
                    ok = false;
                }
            }
        }
    }

    for i in 0..inv.len() {
        if inv[i] != stdinv[i] {
            println!("inv mismatch at {}: got {} expected {}", i, inv[i], stdinv[i]);
            ok = false;
        }
    }

    if ok {
        println!("Generated tables match the canonical Verhoeff tables.");
    } else {
        println!("Some tables did NOT match the canonical Verhoeff tables.");
    }
}

// -------------------- Verhoeff operations using the generated tables --------------------

fn verhoeff_validate(num: &str, D: &Vec<Vec<u8>>, P: &Vec<[u8;10]>) -> bool {
    // Process digits from right to left; positions start at 0
    let mut c: usize = 0;
    let digits: Vec<u8> = num.chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect();

    let mut pos = 0usize; // rightmost digit position 0
    for &d in digits.iter().rev() {
        let p = P[pos % 8][d as usize] as usize;
        c = D[c][p] as usize;
        pos += 1;
    }
    c == 0
}

fn verhoeff_generate_check_digit(num: &str, D: &Vec<Vec<u8>>, P: &Vec<[u8;10]>, inv: &Vec<u8>) -> Option<u8> {
    let digits: Vec<u8> = num.chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect();

    let mut c: usize = 0;
    let mut pos = 1usize;
    for &d in digits.iter().rev() {
        let p = P[pos % 8][d as usize] as usize;
        c = D[c][p] as usize;
        pos += 1;
    }
    Some(inv[c])
}

fn main() {
    println!("Building D5 group elements (as permutations on 5 vertices)...");
    let elements = build_group_elements();
    println!("Elements (as permutations of [0..4]) in canonical labeling:");
    for (i, e) in elements.iter().enumerate() {
        println!("  {} -> {:?}", i, e);
    }

    println!("\nBuilding tables D, P, inv from group composition...");
    let (D, P, inv) = build_tables(&elements);

    println!("\nD (generated):");
    print_table_d(&D);
    println!("\nP (generated):");
    print_table_p(&P);
    println!("\ninv (generated):");
    print_inv(&inv);

    println!("\nComparing with canonical Verhoeff tables...");
    compare_and_print(&D, &P, &inv);

    // quick sanity test: generate check digit and validate
    let sample = "89462597507"; // arbitrary short number
    let check = verhoeff_generate_check_digit(sample, &D, &P, &inv).unwrap();
    println!("\nSample: {} -> check digit {}", sample, check);
    let combined = format!("{}{}", sample, check);
    println!("Combined: {} ; valid? {}", combined, verhoeff_validate(&combined, &D, &P));

    // Another test: known sample (digits can be tested further)
    // e.g., test that a known correct number passes if you know one.
}

