use crate::meshing::{nomal_info::NomalInfo, MeshStyle};
mod meshing;
mod c_meshing;

// use cxx::vector;

fn main() {
    println!("Hello, world!");

    // unsafe {
    //     let res = c_meshing::get_mesh_block(c_meshing::c_block_info{
    //         eta_delta: 0.3,
    //         eta_num:3,
    //         fai_delta: 0.3,
    //         fai_num:3,
    //     });
    //     println!("{:?}", res.eta_mesh);
    //     println!("{:?}", res.fai_mesh);
    // }
    
    
    pause();
}
use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

//

// echo "# meshing" >> README.md
// git init
// git add README.md
// git commit -m "first commit"
// git branch -M main
// git remote add origin https://github.com/lesleyangel/meshing.git
// git push -u origin main
