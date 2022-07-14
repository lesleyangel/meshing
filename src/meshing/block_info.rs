use super::MeshMat;

pub struct BlockInfo {
    pub eta_delta: f64, // eta方向结构间隔
    pub eta_num: usize, // eta方向每个block间网格数
    pub fai_delta: f64, // fai方向结构间隔
    pub fai_num: usize, // fai方向每个block间网格数
}

impl BlockInfo {
    pub fn calc_info(&self) -> Option<MeshMat> {
        if self.check_info() {
            return None;
        }
        let eta_size = (1. / self.eta_delta) as usize * self.eta_num; //block部分网格数
        let fai_size = (1. / self.fai_delta) as usize * self.fai_num; //block部分网格数
        let eta_mesh_delta = self.eta_delta / self.eta_num as f64; //每个block中网格间距
        let fai_mesh_delta = self.fai_delta / self.fai_num as f64; //每个block中网格间距
        let eta_margin_size = ((1. - eta_size as f64 * self.eta_delta) / eta_mesh_delta) as usize;
        let fai_margin_size = ((1. - fai_size as f64 * self.fai_delta) / fai_mesh_delta) as usize;
        let eta_margin_delta = (1. - eta_size as f64 * self.eta_delta) / eta_margin_size as f64;
        let fai_margin_delta = (1. - fai_size as f64 * self.fai_delta) / fai_margin_size as f64;

        let mut res = MeshMat {
            fai_mesh: vec![vec![0.; eta_size + eta_margin_size]; fai_size + fai_margin_size],
            eta_mesh: vec![vec![0.; eta_size + eta_margin_size]; fai_size + fai_margin_size],
        };
        for i in 0..res.fai_mesh.len() {
            for j in 0..res.eta_mesh.len() {
                res.fai_mesh[i][j] = if i < fai_size {
                    fai_mesh_delta * i as f64 
                } else {
                    res.fai_mesh[i - 1][j] + fai_margin_delta
                };
                res.eta_mesh[i][j] = if j < eta_size {
                    eta_mesh_delta * j as f64
                } else {
                    res.eta_mesh[i][j - 1] + eta_margin_delta
                };
            }
        }
        Some(res)
    }

    fn check_info(&self) -> bool {
        if 0.0 < self.eta_delta || self.eta_delta < 1.0 {
            return false;
        }
        if 0.0 < self.fai_delta || self.fai_delta < 1.0 {
            return false;
        }
        return true;
    }
}
