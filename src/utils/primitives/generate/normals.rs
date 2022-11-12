use three_d::core::prelude::Vector3;
use crate::utils::primitives::conversion::conversion::vec3_to_array;
pub fn generate_normals(positions: &Vec<Vector3<f32>>, indices: &[u32]) -> Vec<f32>{
let mut normals:Vec<Vector3<f32>> = vec![];
let mut triangle:Vec<Vector3<f32>> = vec![];
let mut tri_index = -1;
let mut index = 0;
for pos in positions{
tri_index += 1;
if normals.len() == 0 && triangle.len() == 1{
index += 1;
}
if tri_index != 2{
triangle.push(positions[indices[index] as usize]);
} else {
let normal = (triangle[1]-triangle[0]).cross(triangle[2]-triangle[0]);
for n in 0..2{
normals.push(normal);
}
triangle.clear();
tri_index = -1;
}
}
vec3_to_array(&normals)
}