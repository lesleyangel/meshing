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

// // 输出信息
// #[repr(C)]
// pub struct CMeshMat {
//     pub fai_mesh: *mut *mut c_double,
//     pub eta_mesh: *mut *mut c_double,
//     pub eta_num: size_t,
//     pub fai_num: size_t,
// }

// 将meshing::MeshMat 转换为c接口的c_mesh_mat
// unsafe fn mesh_mat_r2c(mat_r: meshing::MeshMat) -> CMeshMat {
//     // let  _p : *mut [u8;3];
//     let mesh_eta_num = mat_r.eta_num();
//     let mesh_fai_num = mat_r.fai_num();
//     let mut eta_res: Vec<*mut c_double> = Vec::new();
//     for i in mat_r.eta_mesh {
//         eta_res.push(i.as_ptr() as *mut f64);
//         std::mem::forget(i);
//     }
//     let mut fai_res: Vec<*mut c_double> = Vec::new();
//     for i in mat_r.fai_mesh {
//         fai_res.push(i.as_ptr() as *mut f64);
//         std::mem::forget(i);
//     }
//     // let fai_pointer =
//     let res = CMeshMat {
//         fai_mesh: fai_res.as_ptr() as *mut *mut f64,
//         eta_mesh: eta_res.as_ptr() as *mut *mut f64,
//         eta_num: mesh_eta_num,
//         fai_num: mesh_fai_num,
//     };

//     std::mem::forget(fai_res);
//     std::mem::forget(eta_res);
//     res
//     // c_mesh_mat {
//     //     fai_mesh: fai_res.as_ptr(),
//     //     eta_mesh: eta_res.as_ptr(),
//     //     eta_num: mesh_eta_num,
//     //     fai_num: mesh_fai_num,
//     // }
// }

// #[no_mangle]
// pub extern "C" fn delete_mesh(res: CMeshMat) {
//     let eta_ptr = unsafe { std::slice::from_raw_parts_mut(res.eta_mesh, res.fai_num) };
//     let fai_ptr = unsafe { std::slice::from_raw_parts_mut(res.fai_mesh, res.fai_num) };

//     for i in 0..res.fai_num {
//         //
//         let eptr = eta_ptr[i];
//         let fptr = fai_ptr[i];
//         unsafe {
//             Box::from_raw(eptr);
//             Box::from_raw(fptr);
//         }
//     }

//     unsafe {
//         // println!("fai_ptr = {:?}", *eta_ptr);
//         Box::from_raw(res.eta_mesh); // 让rust自动管理这块堆内
//         Box::from_raw(res.fai_mesh); // 让rust自动管理这块堆内
//     }
// }
// #[no_mangle]
// pub extern "C" fn delete_mesh_row(res: CMeshMat) {
//     // unsafe {  println!("res.eta_mesh = {:?}", **res.eta_mesh) };
//     let eta_ptr = unsafe { std::slice::from_raw_parts_mut(res.eta_mesh, res.fai_num) };
//     let fai_ptr = unsafe { std::slice::from_raw_parts_mut(res.fai_mesh, res.fai_num) };

//     for i in 0..res.fai_num {
//         let eta_in_ptr = unsafe { std::slice::from_raw_parts_mut(eta_ptr[i], res.eta_num) };
//         // println!("fai_in_ptr_{} = {:?}", i, eta_in_ptr);
//         let eta_in_ptr = eta_in_ptr.as_mut_ptr();
//         //

//         let fai_in_ptr = unsafe { std::slice::from_raw_parts_mut(fai_ptr[i], res.fai_num) };

//         let fai_in_ptr = fai_in_ptr.as_mut_ptr(); // as *mut [f64;len];

//         //
//         unsafe {
//             // println!("fai_in_ptr_{} = {:?}", i, *eta_in_ptr);
//             Box::from_raw(eta_in_ptr); // 让rust自动管理这块堆内存
//             Box::from_raw(fai_in_ptr); // 让rust自动管理这块堆内存
//         }
//     }
//     let eta_ptr = eta_ptr.as_mut_ptr();
//     let fai_ptr = fai_ptr.as_mut_ptr();

//     unsafe {
//         // println!("fai_ptr = {:?}", *eta_ptr);
//         Box::from_raw(eta_ptr); // 让rust自动管理这块堆内
//         Box::from_raw(fai_ptr); // 让rust自动管理这块堆内
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn get_mesh_nomal(info: CNomalInfo) -> CMeshMat {
//     print_my_name();
//     mesh_mat_r2c(
//         match MeshStyle::Nomal(nomal_info::NomalInfo {
//             eta_num: (info).eta_num as usize,
//             fai_num: (info).fai_num as usize,
//         })
//         .meshing_points()
//         {
//             None => meshing::MeshMat {
//                 fai_mesh: vec![vec![0.]; 0],
//                 eta_mesh: vec![vec![0.]; 0],
//             },
//             Some(_res) => _res,
//         },
//     )
// }

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

// #[no_mangle]
// pub unsafe extern "C" fn get_mesh_block(info: CBlockInfo) -> CMeshMat {
//     print_my_name();
//     mesh_mat_r2c(
//         match MeshStyle::Block(block_info::BlockInfo {
//             eta_delta: info.eta_delta,
//             eta_num: info.eta_num as usize,
//             fai_delta: info.fai_delta,
//             fai_num: info.fai_num as usize,
//         })
//         .meshing_points()
//         {
//             None => meshing::MeshMat {
//                 fai_mesh: vec![vec![0.]; 0],
//                 eta_mesh: vec![vec![0.]; 0],
//             },
//             Some(_res) => _res,
//         },
//     )
// }

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
pub unsafe extern "C" fn delete_mesh(ptr: *mut meshing::MeshMat) {
    if ptr.is_null() {
        println!("void ptr!");
        return;
    }
    Box::from_raw(ptr);
    println!("delete it!");
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
