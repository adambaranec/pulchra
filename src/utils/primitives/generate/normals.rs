fn cross(first: [f32; 3], second: [f32; 3]) -> [f32; 3]{
    [(first[1]*second[2])-(first[2]*second[1]),
    (first[2]*second[0])-(first[0]*second[2]),
    (first[0]*second[1])-(first[1]*second[0])]
}

fn dot(first: [f32; 3], second: [f32; 3]) -> [f32; 3] {
[first[0]*second[0],
first[1]*second[1],
first[2]*second[2]]
}
    
fn sub(first: [f32; 3], second: [f32; 3]) -> [f32; 3]{
    [first[0]-second[0],
    first[1]-second[1],
    first[2]-second[2]]
}

pub fn generate_normals(positions: &Vec<f32>, indices: &Vec<u32>)->Vec<f32>{
let mut normals:Vec<f32> = vec![];
let mut triangle:[[f32; 3]; 3] = [[0.0,0.0,0.0],[0.0,0.0,0.0],[0.0,0.0,0.0]];
let mut position:[f32; 3] = [0.0,0.0,0.0];
let mut tri_index:i8 = -1;
let mut indices_iter = indices.iter();
for face in 0..indices.len() / 3{
for vector in 0..2{
let p0 = *indices_iter.next().unwrap() as usize * 3 as usize;
let p1 = p0 + 1 as usize;
let p2 = p0 + 2 as usize;
position[0] = positions[p0];
position[1] = positions[p1];
position[2] = positions[p2];
triangle[vector] = position;
if vector == 2{
let normal = cross(sub(triangle[1],triangle[0]),sub(triangle[2],triangle[0]));
for i in 0..2{
normals.push(normal[0]);
normals.push(normal[1]);
normals.push(normal[2]);
}
}
}
}
normals
}