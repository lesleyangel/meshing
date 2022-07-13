pub mod block_info;
pub mod nomal_info;

use block_info::BlockInfo;
use nomal_info::NomalInfo;

// 暴露的接口主函数
pub fn run(mesh_style: MeshStyle) {
    print_my_name();

    match mesh_style.meshing_points() {
        Option::None => {}
        Option::Some(res) => {
            println!("{:?}", res.eta_mesh); //{:#?}
            println!("{:?}", res.fai_mesh);
        }
    }
}

// 网格类型枚举
pub enum MeshStyle {
    Nomal(NomalInfo), // 指定纵横网格数字
    Block(BlockInfo), // 通过分块指定网格划分
    None,             // 无效的模式
}

// 输出结果
pub struct MeshMat {
    fai_mesh: Vec<Vec<f32>>,
    eta_mesh: Vec<Vec<f32>>,
}

//
impl MeshStyle {
    fn meshing_points(&self) -> Option<MeshMat> {
        match self {
            MeshStyle::None => None,
            MeshStyle::Nomal(_info) => _info.calc_info(),
            MeshStyle::Block(_info) => _info.calc_info(),
        }
    }
}

fn print_my_name() {
    println!("Update in 2022/7/13");
    println!("Creat by YANG Yucheng");
}
