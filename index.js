import init from '/pkg/pulchra.js';
window.onload = (e) =>{
init()
.then((m)=>console.table(m))
.catch('An error occurred. Try again later.')
document.getElementById('input').addEventListener('keydown', (pressed)=>{
    if (pressed.altKey && pressed.key == 'Enter'){}
});
}