use three_d::core::*;
use three_d::CpuMesh;
use crate::utils::primitives::conversion::conversion::vec3_to_array;
pub fn sphere_vertices(subdivisions: u32, range: f32)->Vec<f32>{
    let mut positions:Vec<f32> = vec![];
    let sphere = CpuMesh::sphere(subdivisions);
    positions = vec3_to_array(&sphere.positions.into_f32());
    if range < 1.0 {
      for num in 0..positions.len() - 1{
        positions[num] *= range;
      }
    }
   
    positions
  }
  pub fn sphere_indices(latitudes: u16, longitudes: u16) -> Vec<u16>{
  let mut indices:Vec<u16> = vec![];
  for l in 0..longitudes-2{
  indices.push(0);
  indices.push(l+1);
  indices.push(l+2);
  }
  indices.push(0);
  indices.push(longitudes);
  indices.push(1);
  for la in 0..latitudes{
   for lo in 1..longitudes{
    indices.push(longitudes+lo*(la+1));
    indices.push(longitudes+lo*la);
    indices.push(longitudes+(lo+1)*la);
    indices.push(longitudes+lo*(la+1));
    indices.push(longitudes+(lo+1)*la);
    indices.push(longitudes+(lo+1)*(la+1));
  }
  }
  for l in 0..longitudes-2{
    indices.push((latitudes*longitudes)+1);
    indices.push(latitudes*longitudes - (l+1));
    indices.push(latitudes*longitudes - (l+2));
  }
  indices.push((latitudes*longitudes)+1);
  indices.push(latitudes*longitudes);
  indices.push((latitudes*longitudes) - longitudes);

  indices
  }

  pub fn sphere_normals(subdivisions: u32)->Vec<f32>{
  let mut normals:Vec<f32> = vec![];

  let sphere = CpuMesh::sphere(subdivisions);
  normals = vec3_to_array(&sphere.normals.unwrap());
  
  normals
  }