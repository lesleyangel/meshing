use super::MeshMat;

pub struct NomalInfo {
    pub eta_num: usize,
    pub fai_num: usize,
}

impl NomalInfo {
    pub fn calc_info(&self) -> Option<MeshMat> {
        let mut fai_res = vec![vec![0.0 as f64; self.eta_num]; self.fai_num];
        let mut eta_res = vec![vec![0.0 as f64; self.eta_num]; self.fai_num];
        for i in 0..self.fai_num {
            for j in 0..self.eta_num {
                fai_res[i][j] = i as f64 / (self.fai_num as f64 - 1.0);
                eta_res[i][j] = j as f64 / (self.eta_num as f64 - 1.0);
            }
        }
        Some(MeshMat {
            fai_mesh: fai_res,
            eta_mesh: eta_res,
        })
    }
}

