#include <iostream>
#include <string>
using namespace std;

/*
    VERHOEFF ALGORITHM TABLES
    --------------------------
    These 3 tables are fixed (mathematically generated using D5 group)
    1. d[10][10]  -> multiplication table
    2. p[8][10]   -> permutation table
    3. inv[10]    -> inverse table for checksum generation
*/

int d[10][10] = {
    {0,1,2,3,4,5,6,7,8,9},
    {1,2,3,4,5,6,7,8,9,0},
    {2,3,4,5,6,7,8,9,0,1},
    {3,4,5,6,7,8,9,0,1,2},
    {4,5,6,7,8,9,0,1,2,3},
    {5,6,7,8,9,0,1,2,3,4},
    {6,7,8,9,0,1,2,3,4,5},
    {7,8,9,0,1,2,3,4,5,6},
    {8,9,0,1,2,3,4,5,6,7},
    {9,0,1,2,3,4,5,6,7,8}
};

int p[8][10] = {
    {0,1,2,3,4,5,6,7,8,9},
    {1,5,7,6,2,8,3,0,9,4},
    {5,8,0,3,7,9,6,1,4,2},
    {8,9,1,6,0,4,3,5,2,7},
    {9,4,5,3,1,2,6,8,7,0},
    {4,2,8,6,5,7,3,9,0,1},
    {2,7,9,3,8,0,6,4,1,5},
    {7,0,4,6,9,1,3,2,5,8}
};

int inv[10] = {0,4,3,2,1,5,6,7,8,9};


/*
    FUNCTION: Validate Aadhaar (or any Verhoeff number)
    ---------------------------------------------------
    Steps:
    1. Reverse number
    2. For each position i:
       c = d[c][ p[i % 8][digit] ]
    3. If c == 0 → VALID
*/
bool verhoeffValidate(string num) {
    int c = 0;

    // Process digits from RIGHT → LEFT
    for (int i = num.size() - 1, pos = 0; i >= 0; i--, pos++) {
        int digit = num[i] - '0';  // convert char to real int
        c = d[c][ p[pos % 8][digit] ];
    }
    return (c == 0);
}


/*
    FUNCTION: Generate Checksum Digit
    ---------------------------------
    Steps:
    1. Reverse number
    2. Use p[(pos+1)%8] instead of p[pos%8]
    3. After loop, checksum = inv[c]
*/
int verhoeffGenerate(string num) {
    int c = 0;

    for (int i = num.size() - 1, pos = 0; i >= 0; i--, pos++) {
        int digit = num[i] - '0';
        c = d[c][ p[(pos + 1) % 8][digit] ];
    }

    return inv[c];  // final checksum digit
}


/*
    MAIN PROGRAM
*/
int main() {
    string aadhar;

    cout << "Enter Aadhaar number (without spaces): ";
    cin >> aadhar;

    if (verhoeffValidate(aadhar)) {
        cout << "VALID Aadhaar Number (Checksum correct)\n";
    } 
    else {
        cout << "INVALID Aadhaar Number (Checksum failed)\n";
    }

    // Example: Generate valid Aadhaar number from 11-digit input
    string partial = "82351974062";
    int check = verhoeffGenerate(partial);
    cout << "Generated checksum for " << partial << " = " << check << "\n";
    cout << "Valid Aadhaar-like number = " << partial << check << endl;

    return 0;
}
