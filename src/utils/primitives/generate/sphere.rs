use three_d::core::*;
pub fn sphere_vertices(latitudes: u16, longitudes: u16, range: f32)->Vec<f32>{
    let mut sphere_pos:Vec<f32> = vec![];
    let half_circle = Deg::<f32>::turn_div_2();
    let longitude_distance = half_circle / longitudes as f32;
    let latitude_distance = half_circle / latitudes as f32;
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

  pub fn sphere_normals(latitudes: u16, longitudes: u16)->Vec<f32>{
  let mut normals:Vec<f32> = vec![];
  let turn = Deg::<f32>::full_turn();
  let lo_distance = turn / longitudes as f32;
  //the most upper vector
  normals.push(0.0);
  normals.push(1.0);
  normals.push(0.0);
  //others
  for la in 1..latitudes{
    let y = 1.0;
    let mut x:f32 = 0.0;
    let mut z:f32 = 1.0;
    for lo in 1..longitudes{
      //from forward to right
      if lo < longitudes / 4{
       x = Deg::sin(lo_distance*lo as f32);
       z = Deg::cos(lo_distance*lo as f32);
      }
      //if right
      else if lo == longitudes / 4{
       x = 1.0;
       z = 0.0;
      }
      //from right to away
      else if lo > longitudes / 4 && lo < longitudes / 2{
      x = Deg::sin(lo_distance*lo as f32);
      z = Deg::cos(lo_distance*lo as f32) * -1.0;
      }
      //if away
      else if lo == longitudes / 2{
       x = 0.0;
       z = -1.0;
      }
      //from away to left
      else if lo > longitudes / 2 && lo < (longitudes as f32 * 0.75) as u16{
      x = Deg::sin(lo_distance*lo as f32) * -1.0;
      z = Deg::cos(lo_distance*lo as f32) * -1.0;
      }
      //if left
      else if lo == (longitudes as f32 * 0.75) as u16{
        x = -1.0;
        z = 0.0;
      }
      //from left to forward
      else if lo > (longitudes as f32 * 0.75) as u16 && lo == longitudes - 1{
        x = Deg::sin(lo_distance*lo as f32) * -1.0;
        z = Deg::cos(lo_distance*lo as f32);
      }
    normals.push(x);
    normals.push(y);
    normals.push(z);
    }
  }
  //the most bottom vector
  normals.push(0.0);
  normals.push(-1.0);
  normals.push(0.0);
  
  normals
  }