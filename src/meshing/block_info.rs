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

        let eta_margin = 1. - eta_size as f64 * eta_mesh_delta;
        let fai_margin = 1. - fai_size as f64 * fai_mesh_delta;
        let eta_margin_size = if eta_margin > eta_mesh_delta {
            (eta_margin / eta_mesh_delta) as usize
        } else if eta_margin > 0. {
            1
        } else if eta_margin == 0. {
            0
        } else {
            0
        }; //+ 1;
        let fai_margin_size = if fai_margin > fai_mesh_delta {
            (fai_margin / fai_mesh_delta) as usize
        } else if fai_margin > 0. {
            1
        } else if fai_margin == 0. {
            0
        } else {
            0
        }; //+ 1;
        let eta_margin_delta = eta_margin / if eta_margin_size != 0 {eta_margin_size as f64}else {1.} ;
        let fai_margin_delta = fai_margin / if fai_margin_size != 0 {fai_margin_size as f64}else {1.} ;

        // println!("eta_size = {}", eta_size);
        // println!("fai_size = {}", fai_size);
        // println!("eta_mesh_delta = {}", eta_mesh_delta);
        // println!("fai_mesh_delta = {}", fai_mesh_delta);
        // println!("eta_margin_size = {}", eta_margin_size);
        // println!("fai_margin_size = {}", fai_margin_size);
        // println!("fai_margin_delta = {}", fai_margin_delta);
        // println!("eta_margin_delta = {}", eta_margin_delta);

        let mut res = MeshMat {
            fai_mesh: vec![
                vec![0.; eta_size + eta_margin_size + 1];
                fai_size + fai_margin_size + 1
            ],
            eta_mesh: vec![
                vec![0.; eta_size + eta_margin_size + 1];
                fai_size + fai_margin_size + 1
            ],
            is_print: false,
        };
        for i in 0..res.fai_mesh.len() {
            for j in 0..res.eta_mesh[0].len() {
                res.fai_mesh[i][j] = if i < fai_size + 1 {
                    fai_mesh_delta * i as f64
                } else {
                    res.fai_mesh[i - 1][j] + fai_margin_delta
                };
                res.eta_mesh[i][j] = if j < eta_size + 1 {
                    eta_mesh_delta * j as f64
                } else {
                    res.eta_mesh[i][j - 1] + eta_margin_delta
                };
            }
        }
        // println!("{:?}",res.eta_mesh);
        // println!("{:?}",res.fai_mesh);
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
