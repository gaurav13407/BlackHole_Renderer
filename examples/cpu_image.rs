use std::fs::File;
use std::io::{Write,BufWriter};

use blackhole_renderer::physics::geodesic::{RayState,rk4_step};

fn main(){
    // IMAGE Setting
    let width:usize=800;
    let height:usize=800;

    //CAMERA Setting
    let camera_r:f64=20.0;
    let fov:f64=1.0;

   //Ray/Integreation Settings
    let e:f64=1.0;
    let step_size:f64=0.01;
    let r_max:f64=30.0;
    let max_steps:usize=5000;

    //Create Output File
    let file=File::create("blackhole.ppm").expect("Failed to create file ");
    let mut writer=BufWriter::new(file);

    //PPM Header
    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{} {}",width,height).unwrap();
    writeln!(writer, "255").unwrap();

    //Main image loop
     for j in 0..height{
         for i in 0..width{
             //Pixel->Normalized Screen COORDS
             let x_ndc=(i as f64+0.5)/width as f64*2.0-1.0;
             let y_ndc=(j as f64+0.5)/height as f64*2.0-1.0;

             //Apply feild view
             let x=x_ndc*fov;
             let y=y_ndc*fov; // unused(2D Slice)

             // IMPACT PARAMETER (ANGULAR MOMENTUM)
             let l=camera_r*(x*x+y*y).sqrt();

             //INITIAL RADIAL MOMENTUM 
             let term=e*e
             -(1.0-2.0/camera_r)*(l*l)/(camera_r*camera_r);

             // if invalid array ,make it black 
             if term<0.0{
                 let c=255u8;
                 writeln!(writer, "{} {} {}",c,c,c).unwrap();
                 continue;
             }

             let pr0=-term.sqrt();

             let mut state=RayState{
                 r:camera_r,
                 phi:0.0,
                 pr:pr0,
             };

             //Trace The Ray 
             let mut hit_horizon=false;

             for _ in 0..max_steps{
                 if state.r <=2.0{
                     hit_horizon=true;
                     break;
                 }
                 if state.r>=r_max{
                     break;
                 }
                 state=rk4_step(state, l,step_size);
             }
             // Coloring 
             if hit_horizon{
                 writeln!(writer,"0.0.0").unwrap();

             }else{
                 //Simple background gradient
                 let r_screen=(x*x+y*y).sqrt();
                 let brightness=(1.0-r_screen).clamp(0.0,1.0);
                 let c=(brightness*255.0) as u8;
                 writeln!(writer, "{} {} {} ",c,c,c).unwrap();
             }
         }
     }
     println!("Image written to blackhole.ppm");
    
}
