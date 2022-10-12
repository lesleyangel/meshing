// use crate::meshing::{nomal_info::NomalInfo, MeshStyle};
mod c_meshing;
mod meshing;

use crate::meshing::MeshStyle;
// use cxx::vector;
use crate::meshing::block_info::BlockInfo;
use crate::meshing::free_info::FreeInfo;
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
    let info = BlockInfo {
        eta_delta: 0.1111111111111,
        eta_num: 5,
        fai_delta: 0.166666666666666,
        fai_num: 6,
    };
    let style = MeshStyle::Block(info);
    style.meshing_points();


    let info = FreeInfo {
        eta_delta: vec![0.0,0.2,0.4,0.5,0.9],
        eta_num: vec![1,1,1,1,2],
        fai_delta: vec![0.0,0.2,0.4,0.5,0.9],
        fai_num: vec![1,1,1,1,2],
    };
    let style = MeshStyle::Free(info);
    let res = style.meshing_points();
    match res {
        None => println!("error"),
        Some(data) => {
            println!("{:?}", data.eta_mesh);
            println!("{:?}", data.fai_mesh);
        },
    };
    // pause();
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

// cargo build --release --target i686-pc-windows-msvc