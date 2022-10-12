pub mod block_info;
pub mod nomal_info;
pub mod free_info;

use block_info::BlockInfo;
use nomal_info::NomalInfo;

use self::free_info::FreeInfo;

// 网格类型枚举
pub enum MeshStyle {
    Nomal(NomalInfo), // 指定纵横网格数字
    Block(BlockInfo), // 通过分块指定网格划分
    Free(FreeInfo),   // 通过指定每个分块的位置和网格数划分网格
    None,             // 无效的模式
}

// 输出结果

pub struct MeshMat {
    pub fai_mesh: Vec<Vec<f64>>,
    pub eta_mesh: Vec<Vec<f64>>,
    pub is_print: bool,
}

impl MeshMat {
    pub fn check(&self) -> bool {
        if self.eta_mesh.len() != self.fai_mesh.len() {
            return false;
        } else {
            for i in 0..self.eta_mesh.len() {
                if self.eta_mesh[i].len() != self.fai_mesh[i].len() {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn eta_num(&self) -> Result<usize, String> {
        if self.is_print {
            Ok(self.eta_mesh[0].len())
        } else {
            Err("Error: Never print license!".to_string())
        }
    }
    pub fn fai_num(&self) -> Result<usize, String> {
        if self.is_print {
            Ok(self.eta_mesh.len())
        } else {
            Err("Error: Never print license!".to_string())
        }
    }
    pub fn print_license(&mut self) {
        println!("Update in 2022/7/20");
        println!("Creat by YYC ---v1.2.1");
        self.is_print = true;
    }
}

//
impl MeshStyle {
    pub fn meshing_points(&self) -> Option<MeshMat> {
        match self {
            MeshStyle::None => None,
            MeshStyle::Nomal(_info) => _info.calc_info(),
            MeshStyle::Block(_info) => _info.calc_info(),
            MeshStyle::Free(_info) => _info.calc_info(),
        }
    }
}
