import * as THREE from "three";
import { interpret } from "./interpreter.js";

let input = document.getElementById('input');
let welcomeDialog = document.getElementById('welcome');
let gotIt = document.getElementById('got-it');
//const inputBounds = input.getBoundingClientRect();
//let clickedForRendering = false;
const width = window.innerWidth;
const height = window.innerHeight;
const scene = new THREE.Scene();
scene.background = new THREE.Color('black');
const aspect = width / height;
const camera = new THREE.PerspectiveCamera( 45, aspect, 0.1, 1000 );
camera.position.x = 0;
camera.position.y = 0;
camera.position.z = 4;
camera.lookAt(new THREE.Vector3(0,0,0));
const renderer = new THREE.WebGLRenderer();
renderer.setSize( width, height);
renderer.domElement.style.backgroundColor = "rgba(0,0,0,0.5)";
document.body.appendChild( renderer.domElement );
let meshInfos = new Array();

/* MAIN FUNCTION FOR ANIMATION */ 
const animate = () => {
  requestAnimationFrame( animate );
  renderer.setViewport(0,0,width,height); 
  renderer.render(scene,camera);

  meshInfos.forEach((meshInfo)=>{
    meshInfo.mesh.rotation.x -= meshInfo.rotation.xSpeed/60;
    meshInfo.mesh.rotation.y -= meshInfo.rotation.ySpeed/60;
    meshInfo.mesh.rotation.z -= meshInfo.rotation.zSpeed/60;
  })
}

animate();


window.onload = () => {
welcomeDialog.showModal();
}
window.onresize = (e) =>{
renderer.setSize(window.innerWidth, window.innerHeight);
camera.aspect = window.innerWidth / window.innerHeight;
renderer.setViewport(0,0,window.innerWidth,window.innerHeight);
}
gotIt.onclick = (e) => {welcomeDialog.close();}
input.addEventListener('keydown', (e)=>{
  if (e.ctrlKey && e.key == 'Enter'){
    meshInfos.length = 0;
    interpret(scene,meshInfos,input.value);
          //clickedForRendering = true;
          //const inputBounds = input.getBoundingClientRect();
          //const imageData = ctx.getImageData(inputBounds.x, inputBounds.y, inputBounds.width, inputBounds.height);
          //const [r, g, b] = imageData.data;
          //const isWhite = r > 210 && g > 210 && b > 210;
          //const fontColor = isWhite ? "black" : "white";
          //input.style.color = fontColor;
  }
});