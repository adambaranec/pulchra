import * as THREE from "three";
import { interpret } from "./interpreter.js";
import { shallowEqual } from "./utils.js";

let input = document.getElementById('input');
let welcomeDialog = document.getElementById('welcome');
let gotIt = document.getElementById('got-it');
let errorP = document.getElementById('error');
//const inputBounds = input.getBoundingClientRect();
//let clickedForRendering = false;
let previousInput = "";
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
    if (errorP.innerHTML == ""){
    previousMeshInfos = meshInfos;
    }
    //scene.clear();
    /*const light = new THREE.DirectionalLight(new THREE.Color(1, 1, 1), 1.0);
    light.position.set(0, 0, 2);
    scene.add(light);*/
    const result = interpret(input.value);
    const sceneBackground = result.background;
    if (sceneBackground != undefined){
    scene.background = result.background;
    }
    meshInfos = result.meshInfos; 
    console.log(previousMeshInfos,meshInfos)
    if (previousMeshInfos.length > meshInfos.length){
        /*const redundantMeshesNum = previousMeshInfos.length - meshInfos.length;
        const meshesToRemove = previousMeshInfos.slice(-redundantMeshesNum);
        meshesToRemove.forEach((_meshInfo)=>{scene.remove(_meshInfo.mesh)});*/
         for (let i=0; i<meshInfos.length; i++){
          const _prevMeshInfo = previousMeshInfos[i];
          const _meshInfo = meshInfos[i];
          const typesDiff = _prevMeshInfo.mesh.geometry.type != _meshInfo.mesh.geometry.type;
          let positionsDiff = shallowEqual(_prevMeshInfo.mesh.position,_meshInfo.mesh.position);
          let scalesDiff = shallowEqual(_prevMeshInfo.mesh.scale,_meshInfo.mesh.scale);
          const vertexColorsDiff = _prevMeshInfo.mesh.material.vertexColors != _meshInfo.mesh.material.vertexColors;
          const materialMapDiff = _prevMeshInfo.mesh.material.map != _meshInfo.mesh.material.map;
          let colorDiff = shallowEqual(_prevMeshInfo.mesh.material.color,_meshInfo.mesh.material.color);
          let rotationDiff = shallowEqual(_prevMeshInfo.rotation,_meshInfo.rotation);
          positionsDiff = !positionsDiff;
          scalesDiff = !scalesDiff;
          colorDiff = !colorDiff;
          rotationDiff = !rotationDiff;
          if (typesDiff || positionsDiff || scalesDiff || vertexColorsDiff || materialMapDiff || colorDiff || rotationDiff){
            scene.children.forEach((child,i)=>{
              if (child == _prevMeshInfo.mesh){
                 scene.children[i] = _meshInfo.mesh;
              }
            });
          }
         }
      } else if (previousMeshInfos.length < meshInfos.length){
        /*const newMeshesNum = meshInfos.length - previousMeshInfos.length;
        const meshesToAdd = meshInfos.slice(previousMeshInfos.length);
        meshesToAdd.forEach((_meshInfo)=>{scene.add(_meshInfo.mesh)});*/
        if (previousMeshInfos.length > 0){
          for (let i=0; i<previousMeshInfos.length; i++){
          const _prevMeshInfo = previousMeshInfos[i];
          const _meshInfo = meshInfos[i];
          const typesDiff = _prevMeshInfo.mesh.geometry.type != _meshInfo.mesh.geometry.type;
          let positionsDiff = shallowEqual(_prevMeshInfo.mesh.position,_meshInfo.mesh.position);
          let scalesDiff = shallowEqual(_prevMeshInfo.mesh.scale,_meshInfo.mesh.scale);
          const vertexColorsDiff = _prevMeshInfo.mesh.material.vertexColors != _meshInfo.mesh.material.vertexColors;
          const materialMapDiff = _prevMeshInfo.mesh.material.map != _meshInfo.mesh.material.map;
          let colorDiff = shallowEqual(_prevMeshInfo.mesh.material.color,_meshInfo.mesh.material.color);
          let rotationDiff = shallowEqual(_prevMeshInfo.rotation,_meshInfo.rotation);
          positionsDiff = !positionsDiff;
          scalesDiff = !scalesDiff;
          colorDiff = !colorDiff;
          rotationDiff = !rotationDiff;
          if (typesDiff || positionsDiff || scalesDiff || vertexColorsDiff || materialMapDiff || colorDiff || rotationDiff){
            scene.children.forEach((child,i)=>{
              if (child == _prevMeshInfo.mesh){
                 scene.children[i] = _meshInfo.mesh;
              }
            });
          }
         }
        }
    } else if (previousMeshInfos.length == meshInfos.length){
         for (let i=0; i<meshInfos.length; i++){
          const _prevMeshInfo = previousMeshInfos[i];
          const _meshInfo = meshInfos[i];
          const typesDiff = _prevMeshInfo.mesh.geometry.type != _meshInfo.mesh.geometry.type;
          let positionsDiff = shallowEqual(_prevMeshInfo.mesh.position,_meshInfo.mesh.position);
          let scalesDiff = shallowEqual(_prevMeshInfo.mesh.scale,_meshInfo.mesh.scale);
          const vertexColorsDiff = _prevMeshInfo.mesh.material.vertexColors != _meshInfo.mesh.material.vertexColors;
          const materialMapDiff = _prevMeshInfo.mesh.material.map != _meshInfo.mesh.material.map;
          let colorDiff = shallowEqual(_prevMeshInfo.mesh.material.color,_meshInfo.mesh.material.color);
          let rotationDiff = shallowEqual(_prevMeshInfo.rotation,_meshInfo.rotation);
          positionsDiff = !positionsDiff;
          scalesDiff = !scalesDiff;
          colorDiff = !colorDiff;
          rotationDiff = !rotationDiff;
          if (typesDiff || positionsDiff || scalesDiff || vertexColorsDiff || materialMapDiff || colorDiff || rotationDiff){
            scene.children.forEach((child,i)=>{
              if (child == _prevMeshInfo.mesh){
                 scene.children[i] = _meshInfo.mesh;
              }
            });
          }
         }
    }
  }
});
     