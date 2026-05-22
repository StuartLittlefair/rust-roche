use std::f64::consts::PI;
use pyo3::prelude::*;
use crate::errors::RocheError;
use crate::{Vec3, Star, fblink, rpot, set_earth_iangle, x_l1};
use crate::lobes::{rtsafe, LineRoche};

#[pyfunction]
#[pyo3(name = "shadow", signature = (q, iangle, phi, n=200, dist=5.0, acc=1.0e-4))]
pub fn roche_shadow(q: f64, iangle: f64, phi: f64, n: i32, dist: f64, acc: f64) -> Result<(Vec<f64>, Vec<f64>, Vec<bool>), RocheError> {

    if q <= 0.0 {
        return Err(RocheError::ParameterError("q must be positive.".to_string()));
    }

    if n < 1 {
        return Err(RocheError::ParameterError("n must be > 1.".to_string()));
    }

    let mut x: Vec<f64> = Vec::with_capacity(n as usize);
    let mut y: Vec<f64> = Vec::with_capacity(n as usize);
    let mut shade: Vec<bool> = Vec::with_capacity(n as usize);

    // Compute L1 point and critical potential there.
    let rl1: f64 = x_l1(q)?;
    let earth: crate::Vec3 = set_earth_iangle(iangle, phi);
    let cofm: Vec3 = Vec3::cofm2();
    let mut p: Vec3 = Vec3::new(rl1, 0.0, 0.0);
    let cpot: f64 = rpot(q, &p)?;
    let mut dirn = Vec3::new(0.0, 0.0, 0.0);

    // Limits for Roche lobe computation.
    let upper: f64 = 1.0 - rl1;
    let lower: f64 = upper / 4.0;

    // Now compute Roche lobe in regular steps of angle looking
    // from centre of Roche lobe, then search fro the shadow in between
    // the Roche lobe and the maximum distance

    let mut r1: f64;
    let mut r2: f64;

    for i in 0..n {

        // L1 point is a special case because derivative becomes zero there.
        // lambda is set so that after i=0, there is a decent starting 
        // multiplier
    
        let theta: f64 = 2.0 * PI * (i as f64) / (n as f64 - 1.0);
        let (sin_theta, cos_theta) = theta.sin_cos();
        let dx: f64 = -cos_theta;
        let dy: f64 = sin_theta;
        if i == 0 || i == n - 1 {
            r1 = 1.0 - rl1 + acc;
        } else {
            // Locate critical surface using rtsafe.
            // Based on assuming that rl1 is maximum distance
            // from centre of mass and that at no point is the
            // surface closer than 1/4 of this.
            let line: LineRoche = LineRoche::new(q, Star::Secondary, dx, dy, cpot);
            r1 = rtsafe(lower, upper, |lam| line.cost(lam), acc)? + acc;
        }
        r2 = dist;
        
        // First check status of end points
        dirn.set(dx, dy, 0.0);
        let mut p1: Vec3 = cofm + r1 * dirn;
        let mut p2: Vec3 = cofm + r2 * dirn;

        if !fblink(q, crate::Star::Secondary, 1.0, 1.0, acc, &earth, &p1)? {
            x.push(p1.x);
            y.push(p1.y);
            shade.push(false)
        } else if fblink(q, crate::Star::Secondary, 1.0, 1.0, acc, &earth, &p2)? {
            x.push(p2.x);
            y.push(p2.y);
            shade.push(true)
        } else {
            while r2 - r1 > acc {
                p = (p1 + p2) / 2.0;
                if fblink(q, Star::Secondary, 1.0, 1.0, acc, &earth, &p)? {
                    p1 = p;
                    r1 = (r1 + r2) / 2.0;
                } else {
                    p2 = p;
                    r2 = (r1 + r2) / 2.0;
                }
            }
            p = (p1 + p2) / 2.0;
            x.push(p.x);
            y.push(p.y);
            shade.push(true);
        }
    }

    Ok((x, y, shade))

}