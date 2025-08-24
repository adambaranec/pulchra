import * as THREE from "three";
let input = document.getElementById('input');
import { interpret } from "./interpreter.js";
let welcomeDialog = document.getElementById('welcome');
let gotIt = document.getElementById('got-it');

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
document.body.appendChild( renderer.domElement );

/* MAIN FUNCTION FOR ANIMATION */ 
const animate = () => {
  requestAnimationFrame( animate );
    renderer.setViewport(0,0,width,height); 
    renderer.render(scene,camera);
}

animate();

/* ALL JS THINGS */
      window.onload = () => {
      welcomeDialog.showModal();
      }
      window.onresize = (e) =>{
      renderer.setSize(window.innerWidth, window.innerHeight );
      camera.aspect = window.innerWidth / window.innerHeight;
      renderer.setViewport(0,0,window.innerWidth,window.innerHeight);
      }
      gotIt.onclick = (e) => {welcomeDialog.close();}
      input.addEventListener('keydown', (e)=>{
        if (e.ctrlKey && e.key == 'Enter'){
          interpret(scene,input.value);
        }
      });