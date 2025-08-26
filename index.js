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
const light = new THREE.DirectionalLight(new THREE.Color(1, 1, 1), 1.0);
light.position.set(0, 0, 2);
scene.add(light);
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
let previousMeshInfos = new Array();
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
    previousMeshInfos = meshInfos;
    const result = interpret(input.value);
    const sceneBackground = result.background;
    if (sceneBackground != undefined){
    scene.background = result.background;
    }
    meshInfos = result.meshInfos;
    if (previousMeshInfos.length === 0){
      meshInfos.forEach((_meshInfo)=>{scene.add(_meshInfo.mesh)});
    } else if (meshInfos.length === 0){
      scene.clear();
    } else {
      if (previousMeshInfos.length > meshInfos.length){
        const redundantMeshesNum = previousMeshInfos.length - meshInfos.length;
      } else if (previousMeshInfos.length < meshInfos.length){
        const newMeshesNum = meshInfos.length - previousMeshInfos.length;
      } else if (previousMeshInfos.length == meshInfos.length){
        
      }
    }
  }
});