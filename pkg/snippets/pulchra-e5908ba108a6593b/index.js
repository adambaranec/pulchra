export function get_input(){
 return String(document.getElementById('input').value);
}
export function send_err(error){
    document.getElementById('error').innerHTML = error;
}
