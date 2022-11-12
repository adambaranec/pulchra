use three_d::core::*;
pub fn sphere_vertices(latitudes: u32, longitudes: u32, range: f32)->Vec<f32>{
    let half_circle = Deg::<f32>::turn_div_2();
    let full_circle = Deg::<f32>::full_turn();
    let longitudes = 30; 
    let latitudes = 30;
    let longitude_distance = half_circle / longitudes as f32;
    let latitude_distance = half_circle / latitudes as f32;
    let mut sphere_pos:Vec<f32> = vec![];
    sphere_pos.push(0.0);
    sphere_pos.push(range);
    sphere_pos.push(0.0);

  for latitude in 1..latitudes{
    let y = Deg::cos(latitude_distance * latitude as f32) * range;
      for longitude in 0..longitudes{
        let x = Deg::sin(longitude_distance * longitude as f32) * range;
        let z = Deg::sin(latitude_distance * latitude as f32) * range;
        sphere_pos.push(x);
        sphere_pos.push(y);
        sphere_pos.push(z);
      }
    }
    sphere_pos.push(0.0);
    sphere_pos.push(-range);
    sphere_pos.push(0.0);

    sphere_pos
  }
  pub fn sphere_indices(latitudes: u32, longitudes: u32) -> Vec<u32>{
  let mut indices:Vec<u32> = vec![];
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