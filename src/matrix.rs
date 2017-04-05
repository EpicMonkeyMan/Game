extern crate cgmath;

use cgmath::{Matrix4, Deg, vec3};

pub struct Matrix {
    translate: Matrix4<f32>,
    rotate: Matrix4<f32>,
    scale: Matrix4<f32>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32
}

#[allow(dead_code)]
impl Matrix {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, rotation: f32) -> Matrix {
        let translate = Matrix4::from_translation(vec3(x, y, z));
        let rotate = Matrix4::from_angle_z(Deg(rotation));
        let scale = Matrix4::from_nonuniform_scale(width, height, 1.0);

        Matrix {
            translate: translate,
            rotate: rotate,
            scale: scale,
            x: x,
            y: y,
            z: z,
            width: width,
            height: height,
            rotation: rotation
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        (self.translate * self.rotate * self.scale).into()
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.x = self.x + x;
        self.y = self.y + y;
        self.z = self.z + z;

        self.translate = Matrix4::from_translation(vec3(x, y, z)) * self.translate;
    }
 
    pub fn set_translation(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;

        self.translate = Matrix4::from_translation(vec3(x, y, z));
    }
    
    pub fn rotate(&mut self, deg: f32) {
        self.rotation = self.rotation + deg;

        if self.rotation > 359.0 {
            self.rotation = 0.0;
        } else if self.rotation < 0.0 {
            self.rotation = 359.0;
        }

        self.rotate = Matrix4::from_angle_z(Deg(deg)) * self.rotate;
    }

    pub fn set_rotation(&mut self, deg: f32) {
        self.rotation = deg;

        if self.rotation > 359.0 {
            self.rotation = 0.0;
        } else if self.rotation < 0.0 {
            self.rotation = 359.0;
        }

        self.rotate = Matrix4::from_angle_z(Deg(deg));
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.width = scale;
        self.height = scale;
        
        self.scale = Matrix4::from_scale(scale);
    }

    pub fn set_nonuniform_scale(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;

        self.scale = Matrix4::from_nonuniform_scale(width, height, 1.0);
    }
}