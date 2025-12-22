#[derive(Debug, Copy, Clone)]
pub struct RayState {
    pub r: f64,
    pub phi: f64,
    pub pr: f64,
}

/// Compute derivatives (dr/dλ, dphi/dλ, dpr/dλ)
fn derivatives(state:RayState,l:f64)->RayState{
    let r=state.r;

    let dr=state.pr;
    let dphi=l/(r*r);

    // dV_eff/dr=-2L²/r³ + 6L²/r⁴
    let dv_dr=-2.0*l*l/(r*r*r)
        +6.0*l*l/(r*r*r*r);

    let dpr=-dv_dr;

    RayState{
        r:dr,
        phi:dphi,
        pr:dpr,
    }
}

/// One RK4 integration step
pub fn rk4_step(state: RayState, l: f64, h: f64) -> RayState {
    let k1 = derivatives(state, l);

    let k2 = derivatives(
        RayState {
            r: state.r + 0.5 * h * k1.r,
            phi: state.phi + 0.5 * h * k1.phi,
            pr: state.pr + 0.5 * h * k1.pr,
        },
        l,
    );

    let k3 = derivatives(
        RayState {
            r: state.r + 0.5 * h * k2.r,
            phi: state.phi + 0.5 * h * k2.phi,
            pr: state.pr + 0.5 * h * k2.pr,
        },
        l,
    );

    let k4 = derivatives(
        RayState {
            r: state.r + h * k3.r,
            phi: state.phi + h * k3.phi,
            pr: state.pr + h * k3.pr,
        },
        l,
    );

    RayState{
        r:state.r+(h/6.0)*(k1.r+2.0*k2.r+2.0*k3.r+k4.r),
        phi:state.phi+(h/6.0)*(k1.phi+2.0*k2.phi+2.0*k3.phi+k4.phi),
        pr:state.pr+(h/6.0)*(k1.pr+2.0*k2.pr+2.0*k3.pr+k4.pr),
    }
}
