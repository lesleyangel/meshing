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
    eta_delta: c_double, // eta方向结构间隔
    eta_num: size_t,     // eta方向每个block间网格数
    fai_delta: c_double, // fai方向结构间隔
    fai_num: size_t,     // fai方向每个block间网格数
}

// 输出信息
#[repr(C)]
pub struct c_mesh_mat {
    fai_mesh: *const *const c_double,
    eta_mesh: *const *const c_double,
    eta_num: size_t,
    fai_num: size_t,
}

// 将meshing::MeshMat 转换为c接口的c_mesh_mat
unsafe fn mesh_mat_r2c(mat_r: meshing::MeshMat) -> c_mesh_mat {
    let mut eta_res: Vec<*const c_double> = Vec::new();
    let mesh_eta_num = mat_r.eta_mesh[0].len();
    let mesh_fai_num = mat_r.fai_mesh.len();
    for i in mat_r.eta_mesh {
        eta_res.push(i.as_ptr() as *const c_double);
    }
    let mut fai_res: Vec<*const c_double> = Vec::new();
    for i in mat_r.fai_mesh {
        fai_res.push(i.as_ptr() as *const c_double);
    }

    c_mesh_mat {
        fai_mesh: fai_res.as_ptr(),
        eta_mesh: eta_res.as_ptr(),
        eta_num: mesh_eta_num,
        fai_num: mesh_fai_num,
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_mesh_nomal(info: c_nomal_info) -> c_mesh_mat {
    print_my_name();
    mesh_mat_r2c(
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
pub unsafe extern "C" fn get_mesh_block(info: c_block_info) -> c_mesh_mat {
    print_my_name();
    mesh_mat_r2c(
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
            Some(res) => res,
        },
    )
}

fn print_my_name() {
    println!("Update in 2022/7/13");
    println!("Creat by YANG Yucheng");
}