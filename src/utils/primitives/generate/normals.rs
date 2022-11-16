fn cross(first: [f32; 3], second: [f32; 3]) -> [f32; 3]{
    [(first[1]*second[2])-(first[2]*second[1]),
    (first[2]*second[0])-(first[0]*second[2]),
    (first[0]*second[1])-(first[1]*second[0])]
}
    
fn sub(first: [f32; 3], second: [f32; 3]) -> [f32; 3]{
    [first[0]-second[0],
    first[1]-second[1],
    first[2]-second[2]
    ]
}

pub fn generate_normals(positions: &Vec<f32>, indices: &Vec<u32>)->Vec<f32>{
let mut normals:Vec<f32> = vec![];
let mut triangle:[[f32; 3]; 3] = [[0.0,0.0,0.0],[0.0,0.0,0.0],[0.0,0.0,0.0]];
let mut position:[f32; 3] = [0.0,0.0,0.0];
let mut tri_index:i8 = -1;
for i in indices{
let beginning = ((indices[*i as usize]*3)-1) as usize;
tri_index += 1;
position[0] = positions[beginning];
position[1] = positions[beginning+1];
position[2] = positions[beginning+2];
triangle[tri_index as usize] = position;
if tri_index == 2{
    let first_pos = triangle[0];
    let second_pos = triangle[1];
    let third_pos = triangle[2];
    let normal:[f32; 3] = cross(sub(second_pos, first_pos),sub(third_pos, first_pos));
    normals.push(normal[0]);
    normals.push(normal[1]);
    normals.push(normal[2]);
    tri_index = -1;
}
}
normals
}