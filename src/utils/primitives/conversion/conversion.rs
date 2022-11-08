use three_d::core::*;
pub fn array_to_vec3(array: &Vec<f32>) -> Vec<Vector3<f32>>{
    let mut vector:Vec<f32> = vec![];
    let mut index = 0;
    let mut vec_to_return:Vec<Vector3<f32>> = vec![];
    for element in array{
      if index != 2{
        vector.push(*element);
      } else {
        vector.push(*element);
        vec_to_return.push(Vector3::<f32>::from((vector[0],vector[1],vector[2])));
        vector.clear();
        index = 0;
      }      
    }
    vec_to_return
  }
  pub fn vec3_to_array(vec: &Vec<Vector3<f32>>) -> Vec<f32>{
  let mut vec_to_return:Vec<f32> = vec![];
  for vector in &*vec{
  vec_to_return.push(vector.x);
  vec_to_return.push(vector.y);
  vec_to_return.push(vector.z);
  }
  vec_to_return
  }