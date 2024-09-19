use num_rational::Rational64;
use std::vec::Vec;

fn lagrange_interpolation(points: &[(Rational64, Rational64)]) -> Vec<Rational64> {
    let mut result = vec![Rational64::from(0); points.len()];
    
    for j in 0..points.len() {
        let mut lj = vec![Rational64::from(1)];
        
        for i in 0..points.len() {
            if i != j {
                let xi = points[i].0;
                let xj = points[j].0;
                
                // lj(x) = (x - xi) / (xj - xi)
                lj = multiply_polynomials(
                    &lj, 
                    &[Rational64::from(-xi), Rational64::from(1)]
                );
                
                lj = scale_polynomial(&lj, Rational64::from(1) / (xj - xi));
            }
        }
        
        let yj = points[j].1;
        lj = scale_polynomial(&lj, yj);
        result = add_polynomials(&result, &lj);
    }
    
    result
}

fn multiply_polynomials(p1: &[Rational64], p2: &[Rational64]) -> Vec<Rational64> {
    let mut result = vec![Rational64::from(0); p1.len() + p2.len() - 1];
    
    for i in 0..p1.len() {
        for j in 0..p2.len() {
            result[i + j] += p1[i] * p2[j];
        }
    }
    
    result
}

fn scale_polynomial(p: &[Rational64], scalar: Rational64) -> Vec<Rational64> {
    p.iter().map(|&coef| coef * scalar).collect()
}

fn add_polynomials(p1: &[Rational64], p2: &[Rational64]) -> Vec<Rational64> {
    let len = p1.len().max(p2.len());
    let mut result = vec![Rational64::from(0); len];
    
    for i in 0..p1.len() {
        result[i] += p1[i];
    }
    
    for i in 0..p2.len() {
        result[i] += p2[i];
    }
    
    result
}

fn main() {
    // Define the set S = {(0, 4), (-2, 1), (2, 3)}
    let points = [
        (Rational64::from(0), Rational64::from(4)),
        (Rational64::from(-2), Rational64::from(1)),
        (Rational64::from(2), Rational64::from(3)),
    ];
    
    let polynomial = lagrange_interpolation(&points);
    
    println!("Lagrange Interpolating Polynomial:");
    for (i, coef) in polynomial.iter().enumerate() {
        println!("x^{}: {}", i, coef);
    }
}
