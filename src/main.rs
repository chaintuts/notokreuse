/* This file contains code that demonstrates the dangers of k reuse in ECDSA signatures
* This code calculates k and d (the private key) given two sample message hashes and signatures
* This code uses the secp256k1 curve used in Bitcoin and other major cryptocurrencies
* The sample data is taken from Coinbase's DEFCON capture the coin exercises
*/
use num::BigInt;

/* This function is the main entry point for the program
* For this demonstration, just calls a sample calculation with hard-coded data
*/
fn main()
{
    //sample_calculation();
    let data = read_from_config();
    let result = calculate_reuse(&data.0, &data.1, &data.2, &data.3, &data.4);

    output_results(result);
}

/* Output calculation results to stdout */
fn output_results(result : (BigInt, BigInt))
{
    println!("Successfully calculated reused k and d (private key)!");
    println!("Reused k (hex): {}", result.0.to_str_radix(16));
    println!("Found d (private key) (hex): {}", result.1.to_str_radix(16));
}

/* Read the data from the configuration file
* Must have the data in hexadecimal format, NO preceding 0x
* One item per line
* In order: s1, s2, r, h1, h2
*/
fn read_from_config() -> (BigInt, BigInt, BigInt, BigInt, BigInt)
{
    let raw_data = std::fs::read_to_string("config.txt");

    match raw_data
    {
        Ok(raw_data) => 
        {
            let lines = raw_data.split("\n").collect::<Vec<&str>>();
            let s1 = BigInt::parse_bytes(lines[0].as_bytes(), 16).unwrap();
            let s2 = BigInt::parse_bytes(lines[1].as_bytes(), 16).unwrap();
            let r = BigInt::parse_bytes(lines[2].as_bytes(), 16).unwrap();
            let h1 = BigInt::parse_bytes(lines[3].as_bytes(), 16).unwrap();
            let h2 = BigInt::parse_bytes(lines[4].as_bytes(), 16).unwrap();

            return (s1, s2, r, h1, h2);

        },
        
        Err(err) =>
        {
            panic!(err);
        }
    }
}

/* This function calculates k and d (private key) for a set of signatures and message hashes
* This assumes k is reused, because r is the same point for both signatures (r, s)
*/
fn calculate_reuse(s1 : &BigInt, s2 : &BigInt, r : &BigInt, h1 : &BigInt, h2 : &BigInt) -> (BigInt, BigInt)
{
    let k = calculate_k(h1, h2, s1, s2);
    let d = calculate_d(&k, s1, h1, r);

    return (k, d);
}

/* Calculate and return the reused k value 
* This uses the ECDSA calculation solved out for k when it is reused
*
*     k = h1 - h2
*        -----------
*         s1 - s2
*
* Division is *modulo division* on the curve order
* h1 and h2 are message hashes
* s1 and s2 are the message signatures (s only, no r)
* Remember that an ECDSA signature is actually a set of (r, s) points
* If k is reused, r will be the same for both signatures
*/
fn calculate_k(h1 : &BigInt, h2 : &BigInt, s1 : &BigInt, s2 : &BigInt) -> BigInt
{
    let numerator = h1 - h2;
    let demoninator = s1 - s2;
    let k = modulo_division(&numerator, &demoninator);

    return k;
}

/* This function calculates d (the private key)
* 
*     d = (k * s1) - h1
*          ---------------
*                r
*
* Division is *modulo division* on the curve order
* k is calculated using the calculate_k function
* s1 is the first signature s only
* h1 is the first message hash
* r is the second point of the ECDSA signature from the (r, s) point set
*/
fn calculate_d(k : &BigInt, s1 : &BigInt, h1 : &BigInt, r : &BigInt) -> BigInt
{
    let numerator = (k * s1) - h1;
    let d = modulo_division(&numerator, r);

    return d;
}

/* This function performs modulo division for the curve */
fn modulo_division(a : &BigInt, b : &BigInt) -> BigInt
{
    let curve_order = get_curve_order().clone();
    let ret = ( a * modulo_inverse(&b, &curve_order) ) % curve_order;

    return ret;
}

/* This function performs modulo inverse for the curve */
fn modulo_inverse(a : &BigInt, curve_order : &BigInt) -> BigInt
{
    let exponent = curve_order - BigInt::parse_bytes(b"2", 10).unwrap();
    let modinv = a.modpow(&exponent, curve_order);

    return modinv;
}

/* This function returns the order (cardinality) of the secp256k1 elliptic curve
* This is needed for modular multiplication and division calculations
*/
fn get_curve_order() -> BigInt
{
    let curve_order = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();
    return curve_order;
}