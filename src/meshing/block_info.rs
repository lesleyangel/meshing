use super::MeshMat;

pub struct BlockInfo {
    pub eta_block: Vec<f32>,
    pub fai_block: Vec<f32>,
}

impl BlockInfo {
    pub fn calc_info(&self) -> Option<MeshMat> {
        let v: Vec<f32> = vec![1., 2., 3., 4.];
        let v_v = vec![v];
        let s_s = v_v.clone();
        Some(MeshMat {
            fai_mesh: v_v,
            eta_mesh: s_s,
        })
    }
}
