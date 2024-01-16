use bevy::prelude::*;

// https://mrl.cs.nyu.edu/~perlin/noise/
pub struct PerlinNoise {
    grid: Vec<u16>
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let permutation: Vec<u16> = vec![
            151,160,137,91,90,15,					
            131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
            190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
            88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
            77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
            102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
            135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
            5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
            223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
            129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
            251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
            49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
            138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180
        ];

        let mut perlin = PerlinNoise {
            grid: Vec::new()
        };

        for i in 0..256 {
            perlin.grid.push(permutation[i]);
        }

        for i in 0..256 {
            perlin.grid.push(permutation[i]);
        }

        return perlin;
    }

    pub fn gen_noise(&self, x: f32, y: f32) -> f32 {
        //find which rectangle we are pointing at by using the integer part
        //of the coordinates
        let xi = (x.floor() as u16) & 255;
        let yi = (y.floor() as u16) & 255;
        let x_usize = xi as usize;
    
        let p = &self.grid;
        //use permutation table to find gradient vector later
        let val_bottom_left = p[(p[x_usize] + yi) as usize]; //A
        let val_bottom_right = p[(p[x_usize + 1] + yi) as usize]; //B
        let val_top_left = p[(p[x_usize] + yi + 1) as usize]; //AB
        let val_top_right = p[(p[x_usize + 1] + yi + 1) as usize]; //BB
    
        //find distance vectors
        let xf = x - x.floor();
        let yf = y - y.floor();

        //calculate dot product of corners
        let bottom_left = Vec2::new(xf, yf);
        let bottom_right = Vec2::new(xf - 1., yf);
        let top_left = Vec2::new(xf, yf - 1.);
        let top_right = Vec2::new(xf - 1., yf - 1.);

        let dot_bottom_left = PerlinNoise::get_gradient_vector(val_bottom_left, bottom_left);
        let dot_bottom_right = PerlinNoise::get_gradient_vector(val_bottom_right, bottom_right);
        let dot_top_left = PerlinNoise::get_gradient_vector(val_top_left, top_left);
        let dot_top_right = PerlinNoise::get_gradient_vector(val_top_right, top_right);
    
        let u = PerlinNoise::fade(xf);
        let v = PerlinNoise::fade(yf);
        let dot_bottom = PerlinNoise::lerp(u, dot_bottom_left, dot_bottom_right);
        let dot_top = PerlinNoise::lerp(u, dot_top_left, dot_top_right);
        let val = PerlinNoise::lerp(v, dot_bottom, dot_top);
    
        return (1. - val) / 2.; //remap range from [-1, 1] -> [0, 1]
    }
    
    //https://adrianb.io/2014/08/09/perlinnoise.html
    fn get_gradient_vector(hash: u16, v: Vec2) -> f32 {
        //select 1 of 4 gradient vectors using value from permutation table
        return match hash % 4 {
            0 => v.x + v.y,
            1 => -v.x + v.y,
            2 => -v.x - v.y,
            3 => v.x - v.y,
            _ => panic!("not a valid gradient vector!"),
        }
    }
    
    fn fade (t: f32) -> f32 {
        //removes artifacts from lerp
        //6t^5 - 15t^4 + 10t^3
        return 6. * t.powf(5.) - 15. * t.powf(4.) + 10. * t.powf(3.);
    }
    
    fn lerp (t: f32, a: f32, b: f32) -> f32 {
        return a + t * (b - a);
    }
}
