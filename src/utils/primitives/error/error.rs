use web_sys::*;
pub fn send_err(error: &str){
    let document = web_sys::window().unwrap().document().unwrap();
    let error_p = document.get_element_by_id("error").unwrap();
    error_p.set_inner_html(error);
  }