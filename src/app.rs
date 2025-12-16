mod physics;

use physics::geodesic::{RayState,rk4_step};

fn main(){
    let mut state=RayState{
        r:20.0,
        phi:0.0,
        pr:-1.0, // moving inward
    };

    let l=4.0; // angular momentum (impact parameter)
    let h=0.01; // setupsize
    let r_max=100.0;

mod physics;
mod physics;

use physics::geodesic::{RayState, rk4_step};

fn main() {
    let mut state = RayState {
        r: 20.0,
        phi: 0.0,
        pr: -1.0,   // moving inward
    };

    let l = 4.0;      // angular momentum (impact parameter)
    let h = 0.01;     // step size
    let r_max = 100.0;

    for step in 0..2000 {
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

use physics::geodesic::{RayState, rk4_step};

fn main() {
    let mut state = RayState {
        r: 20.0,
        phi: 0.0,
        pr: -1.0,   // moving inward
    };

    let l = 4.0;      // angular momentum (impact parameter)
    let h = 0.01;     // step size
    let r_max = 100.0;

    for step in 0..2000 {
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
    for step in 0..2000{
        println!("
        step {:4} | r= {:8.4} | phi ={:8.4}
            ",step,state.r,state.phi);
        if state.r<=2.0{
            println!("-> Hit Event Horizon ");
            break;
        }
        if state.r>=r_max{
            println!("-> Escaped To INFINTY");
            break;

        }

        state=rk4_step(state,1,h);
    }
}
