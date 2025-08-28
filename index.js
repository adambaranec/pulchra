import * as THREE from "three";
import { interpret } from "./interpreter.js";
import { shallowEqual } from "./utils.js";

let input = document.getElementById('input');
let welcomeDialog = document.getElementById('welcome');
let gotIt = document.getElementById('got-it');
let errorP = document.getElementById('error');
//const inputBounds = input.getBoundingClientRect();
//let clickedForRendering = false;
const width = window.innerWidth;
const height = window.innerHeight;
const scene = new THREE.Scene();
const light = new THREE.DirectionalLight(new THREE.Color(1, 1, 1), 1.0);
light.position.set(0, 0, 2);
scene.add(light);
scene.background = new THREE.Color('black');
const aspect = width / height;
const camera = new THREE.PerspectiveCamera( 45, aspect, 0.1, 1000 );
camera.position.x = 0;
camera.position.y = 0;
camera.position.z = 4;
camera.lookAt(new THREE.Vector3(0,0,0));
const renderer = new THREE.WebGLRenderer();
renderer.setSize( width, height);
renderer.domElement.style.backgroundColor = "rgba(0,0,0,0)";
document.body.appendChild( renderer.domElement );
let previousMeshInfos = new Array();
let meshInfos = new Array();
let previousBackground;

/* MAIN FUNCTION FOR ANIMATION */ 
const animate = () => {
  requestAnimationFrame( animate );
  renderer.setViewport(0,0,width,height); 
  renderer.render(scene,camera);
  meshInfos.forEach((meshInfo,i)=>{
    scene.children[i+1].rotation.x -= meshInfo.rotation.xSpeed/60;
    scene.children[i+1].rotation.y -= meshInfo.rotation.ySpeed/60;
    scene.children[i+1].rotation.z -= meshInfo.rotation.zSpeed/60;
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
    const result = interpret(input.value);
    const sceneBackground = result.background;
    if (sceneBackground != undefined || previousBackground != sceneBackground){
    scene.background = result.background;
    }
    meshInfos = result.meshInfos; 
    let meshInfosArrayToIterate;
    if (previousMeshInfos.length > meshInfos.length){
      if (meshInfos.length > 0){
        for (let i = meshInfos.length; i<previousMeshInfos.length; i++){
          const child = scene.children[i+1];
          scene.remove(child);
          if (child.geometry) child.geometry.dispose();
          if (child.material) {
          if (Array.isArray(child.material)) {
           child.material.forEach(m => m.dispose());
          } else {
            child.material.dispose();
          }
          }
        }
        meshInfosArrayToIterate = meshInfos;
      } else if (meshInfos.length == 0) {
        scene.children.forEach(child => {
        if (child instanceof THREE.Mesh){
         scene.remove(child);
         if (child.geometry) child.geometry.dispose();
         if (child.material) {
           if (Array.isArray(child.material)) {
           child.material.forEach(m => m.dispose());
           } else {
            child.material.dispose();
           }
        }
        }
        });
      }
    } else if (previousMeshInfos.length < meshInfos.length){
        if (previousMeshInfos.length > 0){
        for (let i = previousMeshInfos.length; i<=meshInfos.length-1; i++){
          scene.add(meshInfos[i].mesh);
        }
        meshInfosArrayToIterate = previousMeshInfos;
        } else if (previousMeshInfos.length == 0){
          meshInfos.forEach((_mInfo)=>scene.add(_mInfo.mesh));
        }
    } else if (previousMeshInfos.length == meshInfos.length){
      meshInfosArrayToIterate = meshInfos;
    }
    if (meshInfosArrayToIterate != undefined){
    for (let i = 1; i<=meshInfosArrayToIterate.length; i++){
      const typesDiff = scene.children[i].geometry.type != meshInfos[i-1].mesh.geometry.type;
      let positionsDiff = shallowEqual(scene.children[i].position,meshInfos[i-1].mesh.position);
      let scalesDiff = shallowEqual(scene.children[i].scale,meshInfos[i-1].mesh.scale);
      const vertexColorsDiff = (scene.children[i].material.vertexColors != meshInfos[i-1].mesh.material.vertexColors) ||
      (scene.children[i].geometry.attributes.color != meshInfos[i-1].mesh.geometry.attributes.color);
      const materialMapDiff = scene.children[i].material.map != meshInfos[i-1].mesh.material.map;
      let colorDiff = shallowEqual(scene.children[i].material.color,meshInfos[i-1].mesh.material.color);
      let rotationDiff = shallowEqual(previousMeshInfos[i-1].rotation,meshInfos[i-1].rotation);
      positionsDiff = !positionsDiff;
      scalesDiff = !scalesDiff;
      colorDiff = !colorDiff;
      rotationDiff = !rotationDiff;
      if(typesDiff){scene.children[i].geometry = meshInfos[i-1].mesh.geometry;}
      if (positionsDiff){scene.children[i].position.set(meshInfos[i-1].mesh.position.x,meshInfos[i-1].mesh.position.y,meshInfos[i-1].mesh.position.z);}
      if (scalesDiff){scene.children[i].scale.set(meshInfos[i-1].mesh.scale.x,meshInfos[i-1].mesh.scale.y,meshInfos[i-1].mesh.scale.z);}
      if (vertexColorsDiff){
      scene.children[i].geometry.attributes = meshInfos[i-1].mesh.geometry.attributes;
      scene.children[i].material.color = meshInfos[i-1].mesh.material.color;
      scene.children[i].material.vertexColors = meshInfos[i-1].mesh.material.vertexColors;
      scene.children[i].material.needsUpdate = true;
      }
      if (materialMapDiff){
      scene.children[i].material.map = meshInfos[i-1].mesh.material.map;
      scene.children[i].material.needsUpdate = true;
      }
      if (colorDiff){scene.children[i].material.color = meshInfos[i-1].mesh.material.color;}
      if (rotationDiff){
        if (meshInfos[i-1].rotation.xSpeed == 0 && meshInfos[i-1].rotation.ySpeed == 0 && meshInfos[i-1].rotation.zSpeed == 0){
         scene.children[i].rotation.set(0,0,0);
        }
      }
    }
    }
        previousMeshInfos = meshInfos;
        previousBackground = sceneBackground;
  }
});
     