use blackhole_renderer::physics::geodesic::{RayState, rk4_step};

fn main() {
    let r0 = 20.0;
    let phi0 = 0.0;

    let e = 1.0;      // photon energy (natural units)
    let l = 3.5;      // try 3.5 and 6.0
    let h = 0.01;
    let r_max = 30.0;

    // ---- THIS IS THE FIX ----
    let term: f64 = e * e - (1.0 - 2.0 / r0) * (l * l) / (r0 * r0);
    let pr0 = -term.sqrt(); // negative = inward photon
    // ------------------------

    let mut state = RayState {
        r: r0,
        phi: phi0,
        pr: pr0,
    };

    for step in 0..5000 {
        println!(
            "step {:4} | r = {:8.4} | phi = {:8.4}",
            step, state.r, state.phi
        );

        if state.r <= 2.0 {
            println!("→ HIT EVENT HORIZON");
            break;
        }

        if state.r >= r_max {
            println!("→ ESCAPED TO INFINITY");
            break;
        }

        state = rk4_step(state, l, h);
    }
}

