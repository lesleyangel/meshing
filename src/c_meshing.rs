use crate::meshing::{self, block_info, nomal_info, MeshStyle};
use libc::{c_double, size_t};

// 输入信息
#[repr(C)]
pub struct CNomalInfo {
    eta_num: size_t,
    fai_num: size_t,
}

#[repr(C)]
pub struct CBlockInfo {
    pub eta_delta: c_double, // eta方向结构间隔
    pub eta_num: size_t,     // eta方向每个block间网格数
    pub fai_delta: c_double, // fai方向结构间隔
    pub fai_num: size_t,     // fai方向每个block间网格数
}

// 输出信息
#[repr(C)]
pub struct CMeshMat {
    pub fai_mesh: *mut *mut c_double,
    pub eta_mesh: *mut *mut c_double,
    pub eta_num: size_t,
    pub fai_num: size_t,
}


// 将meshing::MeshMat 转换为c接口的c_mesh_mat
unsafe fn mesh_mat_r2c(mat_r: meshing::MeshMat) -> CMeshMat {
    // let  _p : *mut [u8;3];
    let mesh_eta_num = mat_r.eta_num();
    let mesh_fai_num = mat_r.fai_num();
    let mut eta_res: Vec<*mut c_double> = Vec::new();
    for i in mat_r.eta_mesh {
        eta_res.push(i.as_ptr() as *mut f64);
        std::mem::forget(i);
    }
    let mut fai_res: Vec<*mut c_double> = Vec::new();
    for i in mat_r.fai_mesh {
        fai_res.push(i.as_ptr() as *mut f64);
        std::mem::forget(i);
    }
    // let fai_pointer =
    let res = CMeshMat {
        fai_mesh: fai_res.as_ptr() as *mut *mut f64,
        eta_mesh: eta_res.as_ptr() as *mut *mut f64,
        eta_num: mesh_eta_num,
        fai_num: mesh_fai_num,
    };

    std::mem::forget(fai_res);
    std::mem::forget(eta_res);
    res
    // c_mesh_mat {
    //     fai_mesh: fai_res.as_ptr(),
    //     eta_mesh: eta_res.as_ptr(),
    //     eta_num: mesh_eta_num,
    //     fai_num: mesh_fai_num,
    // }
}

#[no_mangle]
pub extern "C" fn delete_mesh(res: CMeshMat) {
    let eta_ptr = unsafe { std::slice::from_raw_parts_mut(res.eta_mesh, res.fai_num) };
    let fai_ptr = unsafe { std::slice::from_raw_parts_mut(res.fai_mesh, res.fai_num) };

    for i in 0..res.fai_num {
        let eta_in_ptr = unsafe { std::slice::from_raw_parts_mut(eta_ptr[i], res.eta_num) };
        let eta_in_ptr = eta_in_ptr.as_mut_ptr();
        //
        let fai_in_ptr = unsafe { std::slice::from_raw_parts_mut(fai_ptr[i], res.fai_num) };
        let fai_in_ptr = fai_in_ptr.as_mut_ptr();
        //
        unsafe {
            Box::from_raw(eta_in_ptr); // 让rust自动管理这块堆内存
            Box::from_raw(fai_in_ptr); // 让rust自动管理这块堆内存
        }
    }
    let eta_ptr = eta_ptr.as_mut_ptr();
    let fai_ptr = fai_ptr.as_mut_ptr();
    unsafe {
        Box::from_raw(eta_ptr); // 让rust自动管理这块堆内
        Box::from_raw(fai_ptr); // 让rust自动管理这块堆内
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_mesh_nomal(info: CNomalInfo) -> CMeshMat {
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
            Some(_res) => _res,
        },
    )
}

#[no_mangle]
pub unsafe extern "C" fn get_mesh_block(info: CBlockInfo) -> CMeshMat {
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
            Some(_res) => _res
        },
    )
}

fn print_my_name() {
    println!("Update in 2022/7/17");
    println!("Creat by YYC ---v1.1.0");
}
