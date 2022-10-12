use crate::meshing::{self, block_info, nomal_info, MeshStyle, free_info::FreeInfo};
use libc::{c_double, size_t,c_int};

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

#[no_mangle]
pub extern "C" fn get_mesh_nomal(info: CNomalInfo) -> *mut meshing::MeshMat {
    // print_my_name();
    // mesh_mat_r2c(
    let res = match MeshStyle::Nomal(nomal_info::NomalInfo {
        eta_num: (info).eta_num as usize,
        fai_num: (info).fai_num as usize,
    })
    .meshing_points()
    {
        None => meshing::MeshMat {
            fai_mesh: vec![vec![0.]; 0],
            eta_mesh: vec![vec![0.]; 0],
            is_print: false,
        },
        Some(_res) => _res,
    };
    let res = Box::new(res);
    Box::into_raw(res)
    // )
}

#[no_mangle]
pub extern "C" fn get_mesh_block(info: CBlockInfo) -> *mut meshing::MeshMat {
    // mesh_mat_r2c(
    let res = match MeshStyle::Block(block_info::BlockInfo {
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
            is_print: false,
        },
        Some(_res) => _res,
    };
    let res = Box::new(res);
    Box::into_raw(res)
    // )
}



#[no_mangle]
pub extern "C" fn get_mesh_free(ptr:*mut FreeInfo) -> *mut meshing::MeshMat {
    let info = unsafe{Box::from_raw(ptr)};
    let res = match MeshStyle::Free(*info)
    .meshing_points()
    {
        None => meshing::MeshMat {
            fai_mesh: vec![vec![0.]; 0],
            eta_mesh: vec![vec![0.]; 0],
            is_print: false,
        },
        Some(_res) => _res,
    };
    let res = Box::new(res);
    Box::into_raw(res)
}

#[no_mangle]
pub unsafe extern "C" fn delete_mesh(ptr: *mut meshing::MeshMat) -> c_int {
    if ptr.is_null() {
        println!("void ptr!");
        return -1;
    }
    Box::from_raw(ptr);
    println!("delete it!");
    return 0;
}

#[no_mangle]
pub extern "C" fn get_eta(fai_num: size_t, eta_id: size_t, res: *mut meshing::MeshMat) -> c_double {
    unsafe { (*res).eta_mesh[fai_num][eta_id] }
}
#[no_mangle]
pub extern "C" fn get_fai(fai_num: size_t, eta_id: size_t, res: *mut meshing::MeshMat) -> c_double {
    unsafe { (*res).fai_mesh[fai_num][eta_id] }
}
#[no_mangle]
pub extern "C" fn get_eta_num(res: *mut meshing::MeshMat) -> size_t {
    match unsafe { (*res).eta_num() } {
        Ok(_res) => _res,
        Err(_err) => panic!("{}", _err),
    }
}
#[no_mangle]
pub extern "C" fn get_fai_num(res: *mut meshing::MeshMat) -> size_t {
    match unsafe { (*res).fai_num() }{
        Ok(_res) => _res,
        Err(_err) => panic!("{}", _err),
    }
}
#[no_mangle]
pub extern "C" fn print_license(res: *mut meshing::MeshMat) {
    unsafe {
        (*res).print_license();
    }
}
