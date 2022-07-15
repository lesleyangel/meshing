use crate::meshing::{self, block_info, nomal_info, MeshStyle};
use libc::{c_double, size_t};

// 输入信息
#[repr(C)]
pub struct c_nomal_info {
    eta_num: size_t,
    fai_num: size_t,
}

#[repr(C)]
pub struct c_block_info {
    pub eta_delta: c_double, // eta方向结构间隔
    pub eta_num: size_t,     // eta方向每个block间网格数
    pub fai_delta: c_double, // fai方向结构间隔
    pub fai_num: size_t,     // fai方向每个block间网格数
}

// 输出信息
#[repr(C)]
pub struct c_mesh_mat {
    pub fai_mesh: *const *const c_double,
    pub eta_mesh: *const *const c_double,
    pub eta_num: size_t,
    pub fai_num: size_t,
}

// 将meshing::MeshMat 转换为c接口的c_mesh_mat
unsafe fn mesh_mat_r2c(res: *mut c_mesh_mat, mat_r: meshing::MeshMat) {
    
    let  _p : *mut [u8;3];
    let mesh_eta_num = mat_r.eta_mesh[0].len();
    let mesh_fai_num = mat_r.fai_mesh.len();
    let mut eta_res: Vec<*const c_double> = Vec::new();
    for i in mat_r.eta_mesh {
        eta_res.push(i.as_ptr() as *const c_double);
        std::mem::forget(i);
    }
    let mut fai_res: Vec<*const c_double> = Vec::new();
    for i in mat_r.fai_mesh {
        fai_res.push(i.as_ptr() as *const c_double);
        std::mem::forget(i);
    }
    // let fai_pointer = 
    (*res).fai_mesh = fai_res.as_ptr();
    (*res).eta_mesh = eta_res.as_ptr();
    (*res).eta_num = mesh_eta_num;
    (*res).fai_num = mesh_fai_num;
    std::mem::forget(fai_res);
    std::mem::forget(eta_res);

    // c_mesh_mat {
    //     fai_mesh: fai_res.as_ptr(),
    //     eta_mesh: eta_res.as_ptr(),
    //     eta_num: mesh_eta_num,
    //     fai_num: mesh_fai_num,
    // }
}

#[no_mangle]
pub unsafe extern "C" fn get_mesh_nomal(res: *mut c_mesh_mat, info: c_nomal_info)  {
    print_my_name();
    mesh_mat_r2c(res,
        match MeshStyle::Nomal(nomal_info::NomalInfo {
            eta_num: (info).eta_num as usize,
            fai_num: (info).fai_num as usize,
        })
        .meshing_points()
        {
            None => meshing::MeshMat {
                fai_mesh: vec![vec![0.]; 0],
                eta_mesh: vec![vec![0.]; 0],
            },
            Some(res) => res,
        },
    )
}

#[no_mangle]
pub unsafe extern "C" fn get_mesh_block(res: *mut c_mesh_mat, info: c_block_info) {
    print_my_name();
    mesh_mat_r2c(res,
        match MeshStyle::Block(block_info::BlockInfo {
            eta_delta: info.eta_delta,
            eta_num: info.eta_num as usize,
            fai_delta: info.fai_delta,
            fai_num: info.fai_num as usize,
        })
        .meshing_points()
        {
            None => meshing::MeshMat {
                fai_mesh: vec![vec![0.]; 0],
                eta_mesh: vec![vec![0.]; 0],
            },
            Some(_res) => {
                println!("{:?}", _res.eta_mesh);
                println!("{:?}", _res.fai_mesh);
                _res
            }
        },
    );
}

fn print_my_name() {
    println!("Update in 2022/7/13");
    println!("Creat by YANG Yucheng");
}
