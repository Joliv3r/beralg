use std::{sync::Arc, time::Duration};
use std::time::Instant;
use crate::{algebraic_structure::{finite_field::FiniteField, Element}, integer_computations::naive_pow};
use rug::Integer;
use plotters::prelude::*;


pub fn check_timing_against_rug(a: &Integer, b: &Integer, p: &Integer) -> (Duration, Duration) {
    let f = Arc::new(FiniteField::new(p.clone()).unwrap());
    let a_elem = Element::new(f.clone(), a.clone());
    let b_exp = b.clone();

    let now = Instant::now();
    a_elem.pow(&b_exp);
    let elapsed_elem = now.elapsed();

    let now = Instant::now();
    a.clone().pow_mod(&b, &p).unwrap();
    let elapsed = now.elapsed();

    (elapsed_elem, elapsed)
}


pub fn check_timing_against_naive(a: &Integer, b: &Integer, p: &Integer) -> (Duration, Duration) {
    let f = Arc::new(FiniteField::new(p.clone()).unwrap());
    let a_elem = Element::new(f.clone(), a.clone());
    let b_exp = b.clone();

    let now = Instant::now();
    a_elem.pow(&b_exp);
    let elapsed_square = now.elapsed();

    let now = Instant::now();
    naive_pow(&a, &b, &p);
    let elapsed_naive = now.elapsed();

    (elapsed_naive, elapsed_square)
}


pub fn check_timings(n: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Integer::from(17);
    // let mut rng = RandState::new();
    let mut naive_vec: Vec<(u128, u128)> = Vec::new();
    let mut square_vec: Vec<(u128, u128)> = Vec::new();
    let mut max_time_naive = 0;
    let mut max_time_square = 0;
    for _ in 0..n {
        for _ in 0..50 {
            p.next_prime_mut();
        }

        let a = p.clone(); //.random_below(&mut rng);
        let b = p.clone(); //.random_below(&mut rng);
        let (elapsed_naive, elapsed_square) = check_timing_against_naive(&a, &b, &p);
        naive_vec.push((p.to_u128().unwrap(), elapsed_naive.as_nanos()));
        square_vec.push((p.to_u128().unwrap(), elapsed_square.as_nanos()));
        if elapsed_naive.as_nanos() > max_time_naive {
            max_time_naive = elapsed_naive.as_nanos();
        }
        if elapsed_square.as_nanos() > max_time_square {
            max_time_square = elapsed_square.as_nanos();
        }
    }

    let root = SVGBackend::new("plots/naive.svg", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Runtime in nanoseconds", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..p.to_u128().unwrap_or(u128::MAX), 0..max_time_naive)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            // (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            naive_vec,
            &RED,
        ))?
        .label("Naive approach")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    let root = SVGBackend::new("plots/square.svg", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Runtime in nanoseconds", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..p.to_u128().unwrap_or(u128::MAX), 0..max_time_square)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            // (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            square_vec,
            &BLUE,
        ))?
        .label("Square approach")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebraic_structure::Element;
    use crate::algebraic_structure::finite_field::MultiplicativeGroup;
    use rug::{Complete, rand::RandState};
    use std::sync::Arc;

    #[test]
    fn test_algebraic_structure_arithmetic() {
        let n: u32 = 200;
        let mut prime: Integer = Integer::from(2);
        let mut rng = RandState::new();

        for _ in 2..n {
            let p = prime.clone();
            let f: Arc<FiniteField> = Arc::new(FiniteField::new(p.clone()).unwrap());
            let a_rand: Integer = Integer::from(rng.bits(32));
            let b_rand: Integer = Integer::from(rng.bits(32));
            let a = Element::new(f.clone(), a_rand.clone());
            let b = Element::new(f.clone(), b_rand.clone());

            assert_eq!(a.get_rep(), &(&a_rand % &p).complete());
            assert_eq!(b.get_rep(), &(&b_rand % &p).complete());

            let added = (a.add_ref(&b)).get_rep().clone();
            let multiplied = (a.mul_ref(&b)).get_rep().clone();

            let added_check: Integer = ((&a_rand % &p).complete() + (&b_rand % &p).complete()) % &p;
            let mul_check: Integer = ((a_rand % &p) * (b_rand % &p)) % &p;

            assert_eq!(added, added_check, "Failed for: {} + {}, and got {} instead of {}", a.get_rep(), b.get_rep(), added, added_check);
            assert_eq!(multiplied, mul_check, "Failed for: {} * {}, and got {} instead of {}", a.get_rep(), b.get_rep(), multiplied, mul_check);

            let g: Arc<MultiplicativeGroup> = Arc::new(MultiplicativeGroup::from_finite_field(&f));

            // To not create zero
            let modification: Integer = Integer::from(rng.bits(32));
            let a_rand: Integer = (Integer::from(rng.bits(32)).modulo(&(&p-&Integer::ONE.clone()).complete())) + 1 + (&p * &modification);
            let b_rand: Integer = (Integer::from(rng.bits(32)).modulo(&(&p-&Integer::ONE.clone()).complete())) + 1 + (&p * modification);

            let a = Element::new(g.clone(), a_rand.clone());
            let b = Element::new(g.clone(), b_rand.clone());

            assert_eq!(a.get_rep(), &(&a_rand % &p).complete(), "Failed representation of {} in Z_{}*,  got {} instead of {}", &a_rand, &p, a.get_rep(), (&a_rand % &p).complete());
            assert_eq!(b.get_rep(), &(&b_rand % &p).complete(), "Failed representation of {} in Z_{}*,  got {} instead of {}", &b_rand, &p, b.get_rep(), (&b_rand % &p).complete());

            let multiplied = a.mul_ref(&b);
            let mul_check: Integer = ((&a_rand % &p).complete() * (&b_rand % &p).complete()) % &p;

            assert_eq!(multiplied.get_rep(), &mul_check);

            let divided = a.div_ref(&b);

            assert_eq!(divided.mul_ref(&b).get_rep(), &(&a_rand % &p).complete(), "Failed division {}/{} in Z_{}*", &a_rand, &b_rand, &p);


            prime.next_prime_mut();
        }
    }

    #[test]
    #[should_panic(expected = "Zero Division")]
    fn test_zero_division() {
        let mut rng = RandState::new();
        let p: Integer = Integer::from(rng.bits(32)).next_prime();
        let a_rand: Integer = Integer::from(rng.bits(32));
        let f: Arc<FiniteField> = Arc::new(FiniteField::new(p).unwrap());
        let a_elem: Element<FiniteField> = Element::new(f.clone(), a_rand);
        let b_elem: Element<FiniteField> = Element::new(f, Integer::ZERO);

        a_elem.div_ref(&b_elem);
    }

    #[test]
    fn test_extended_euclidean() {
        let n: u32 = 200;
        let mut prime: Integer = Integer::from(2);
        let mut rng = RandState::new();

        for _ in 2..n {
            let p = prime.clone();
            let f = Arc::new(FiniteField::new(p.clone()).unwrap());
            let a_rand = (Integer::from(rng.bits(32)).modulo(&(&p-&Integer::ONE.clone()).complete())) + 1 ;
            let a = Element::new(f.clone(), a_rand);

            let a_inv = a.inv();
            assert_eq!(Integer::ONE, a_inv.mul_ref(&a).get_rep(), "We have a: {}, a_inv: {}, p: {}", a.get_rep(), a_inv.get_rep(), p);

            let g = Arc::new(MultiplicativeGroup::from_finite_field(&f));
            let a_rand = (Integer::from(rng.bits(32)).modulo(&(&p-&Integer::ONE.clone()).complete())) + 1 ;
            let a = Element::new(g.clone(), a_rand);
            let a_inv = a.inv();
            assert_eq!(Integer::ONE, a_inv.mul_ref(&a).get_rep(), "We have a: {}, a_inv: {}, p: {}", a.get_rep(), a_inv.get_rep(), p);


            prime.next_prime_mut();

        }
    }


    #[test]
    fn test_exponentiation() {
        let n: u32 = 200;
        let mut rng = RandState::new();
        let mut prime: Integer = Integer::from(2);


        for _ in 2..n {
            let p = prime.clone();
            let f = Arc::new(FiniteField::new(p.clone()).unwrap());

            let a_rand = Integer::from(rng.bits(32));
            let a = Element::new(f.clone(), a_rand.clone());

            let x_rand = Integer::from(rng.bits(32));

            // let a_exp_check = a_rand.exp_residue(&x_rand, &p);
            let a_exp_check = a_rand.pow_mod_ref(&x_rand, &p).unwrap().complete();
            let a_exp = a.pow(&x_rand);

            assert_eq!(a_exp.get_rep(), &a_exp_check);

            prime.next_prime_mut();
        }
    }
}
