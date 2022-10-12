use super::MeshMat;
use libc::{c_double, size_t,c_int};
pub struct FreeInfo {
    pub eta_delta: Vec<f64>, // eta方向结构间隔
    pub eta_num: Vec<usize>, // eta方向每个block间网格数
    pub fai_delta: Vec<f64>, // fai方向结构间隔
    pub fai_num: Vec<usize>, // fai方向每个block间网格数
}

impl FreeInfo {
    pub fn calc_info(&self) -> Option<MeshMat> {
        if !self.check_info() {
            return None;
        }
        // 计算两个方向的网格个数
        let mut eta_size = 1;
        for num in &self.eta_num {
            eta_size += num;
        }
        let mut fai_size = 1;
        for num in &self.fai_num {
            fai_size += num;
        }

        // 计算网格
        let mut res = MeshMat {
            fai_mesh: vec![vec![1.; eta_size]; fai_size],
            eta_mesh: vec![vec![1.; eta_size]; fai_size],
            is_print: false,
        };
        let mut i = 0;
        let mut eta_del = self.eta_delta.clone();
        let mut fai_del = self.fai_delta.clone();
        eta_del.push(1.0);
        fai_del.push(1.0);
        for i_block in 0..self.fai_num.len() {
            let fai_block_delta =
                (fai_del[i_block + 1] - fai_del[i_block]) / self.fai_num[i_block] as f64;
            for i_num in 0..self.fai_num[i_block] {
                let mut j = 0;
                for j_block in 0..self.eta_num.len() {
                    let eta_block_delta =
                        (eta_del[j_block + 1] - eta_del[j_block]) / self.eta_num[j_block] as f64;
                    for j_num in 0..self.eta_num[j_block] {
                        res.fai_mesh[i][j] = fai_del[i_block] + i_num as f64 * fai_block_delta;
                        res.eta_mesh[i][j] = eta_del[j_block] + j_num as f64 * eta_block_delta;
                        j += 1;
                    }
                }
                i += 1;
            }
        }

        res.eta_mesh[fai_size - 1] = res.eta_mesh[fai_size - 2].clone();
        for i in 0..fai_size {
            res.fai_mesh[i][eta_size - 1] = res.fai_mesh[i][eta_size - 2];
        }

        Some(res)
    }

    fn check_info(&self) -> bool {
        if self.eta_delta.len() != self.eta_num.len() {
            return false;
        }
        if self.fai_delta.len() != self.fai_num.len() {
            return false;
        }
        true
    }
}

#[no_mangle]
pub extern  "C" fn creat_free_info() -> *mut FreeInfo{
    let info = FreeInfo {
        eta_delta: vec![],
        eta_num: vec![],
        fai_delta: vec![],
        fai_num: vec![],
    };
    let res = Box::new(info);
    Box::into_raw(res)
}

#[no_mangle]
pub extern  "C" fn add_free_eta_info(delta:c_double,num:size_t,ptr:*mut FreeInfo) -> *mut FreeInfo {
    if ptr.is_null() {
        println!("void ptr!");
    }
    let mut box_ptr = unsafe{Box::from_raw(ptr)};
    box_ptr.eta_delta.push(delta);
    box_ptr.eta_num.push(num);
    Box::into_raw(box_ptr)
}

#[no_mangle]
pub extern  "C" fn add_free_fai_info(delta:c_double,num:size_t,ptr:*mut FreeInfo) -> *mut FreeInfo {
    if ptr.is_null() {
        println!("void ptr!");
    }
    let mut box_ptr = unsafe{Box::from_raw(ptr)};
    box_ptr.fai_delta.push(delta);
    box_ptr.fai_num.push(num);
    Box::into_raw(box_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn delete_free_info(ptr:*mut FreeInfo) -> c_int {
    if ptr.is_null() {
        println!("void ptr!");
        return -1;
    }
    Box::from_raw(ptr);
    println!("delete it!");
    return 0;
}