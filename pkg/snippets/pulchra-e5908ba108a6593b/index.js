import start from '/pkg/pulchra.js';
export function get_input(){
 return String(document.getElementById('input').value);
}
window.onload = (e) =>{
start()
.catch('An error occurred. Try again later.')
document.getElementById('input').addEventListener('keydown', (pressed)=>{
    if (pressed.ctrlKey && pressed.key == 'Enter'){
        console.log(get_input());
    }
});
}