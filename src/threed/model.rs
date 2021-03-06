use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use gl;
use tobj;
use tobj::Model;

use crate::util::resources::Resources;
use crate::renderer_gl::shader::*;
use crate::threed::mesh::{FGMesh, FGTexture, FGVertex};

#[derive(Default)]
pub struct FGModel {
    /*  Model Data */
    pub meshes: Vec<FGMesh>,
    pub textures_loaded: Vec<FGTexture>, // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    directory: String,
}

impl FGModel {
    /// constructor, expects a filepath to a 3D model.
    pub fn new(gl: &gl::Gl, res: &Resources, path: &str) -> FGModel {
        let mut model = FGModel::default();

        model.load_model(gl, res, path);
        model
    }

    pub fn draw(&self, gl: &gl::Gl, shader: &Program) {
        for mesh in &self.meshes {
            mesh.draw(gl, shader);
        }
    }

    // loads a model from file and stores the resulting meshes in the meshes vector.
    fn load_model(&mut self, gl: &gl::Gl, res: &Resources, path: &str) {

        let obj = res.load_model(path);

        let (models, materials) = obj.unwrap();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // data to fill
            let mut vertices: Vec<FGVertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            println!("Model from {}, mesh: {} with {} vertices {} indices", path, model.name, num_vertices, indices.len());
            for i in 0..num_vertices {
                vertices.push(FGVertex {
                    pos: (p[i * 3], p[i * 3 + 1], p[i * 3 + 2]).into(),
                    nml: (n[i * 3], n[i * 3 + 1], n[i * 3 + 2]).into(),
                    tex: (t[i * 2], t[i * 2 + 1]).into(),
                    ..FGVertex::default()
                })
            }

            // // process material
            // let mut textures = Vec::new();
            // if let Some(material_id) = mesh.material_id {
            //     let material = &materials[material_id];

            //     // 1. diffuse map
            //     if !material.diffuse_texture.is_empty() {
            //         let texture = self.loadMaterialTexture(&material.diffuse_texture, "texture_diffuse");
            //         textures.push(texture);
            //     }
            //     // 2. specular map
            //     if !material.specular_texture.is_empty() {
            //         let texture = self.loadMaterialTexture(&material.specular_texture, "texture_specular");
            //         textures.push(texture);
            //     }
            //     // 3. normal map
            //     if !material.normal_texture.is_empty() {
            //         let texture = self.loadMaterialTexture(&material.normal_texture, "texture_normal");
            //         textures.push(texture);
            //     }
            //     // NOTE: no height maps
            // }

            self.meshes.push(FGMesh::new(gl, vertices, indices));
        }
    }
}

//     fn loadMaterialTexture(&mut self, path: &str, typeName: &str) -> Texture {
//         {
//             let texture = self.textures_loaded.iter().find(|t| t.path == path);
//             if let Some(texture) = texture {
//                 return texture.clone();
//             }
//         }

//         let texture = Texture {
//             id: unsafe { TextureFromFile(path, &self.directory) },
//             type_: typeName.into(),
//             path: path.into()
//         };
//         self.textures_loaded.push(texture.clone());
//         texture
//     }
// }

// unsafe fn TextureFromFile(path: &str, directory: &str) -> u32 {
//     let filename = format!("{}/{}", directory, path);

//     let mut textureID = 0;
//     gl::GenTextures(1, &mut textureID);

//     let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
//     let img = img.flipv();
//     let format = match img {
//         ImageLuma8(_) => gl::RED,
//         ImageLumaA8(_) => gl::RG,
//         ImageRgb8(_) => gl::RGB,
//         ImageRgba8(_) => gl::RGBA,
//     };

//     let data = img.raw_pixels();

//     gl::BindTexture(gl::TEXTURE_2D, textureID);
//     gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
//         0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
//     gl::GenerateMipmap(gl::TEXTURE_2D);

//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

//     textureID
// }

