/* DO NOT NOTICE THIS FILE. JUST FOR TESTING PURPOSES */

import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );

const renderer = new THREE.WebGLRenderer();
renderer.setSize( window.innerWidth, window.innerHeight );
document.body.appendChild( renderer.domElement );

const textureLoader = new THREE.TextureLoader();
const normalMap = textureLoader.load('https://upload.wikimedia.org/wikipedia/commons/9/9a/Milan_Adam%C4%8Diak.jpg');
const displacementMap = textureLoader.load('https://upload.wikimedia.org/wikipedia/commons/9/9a/Milan_Adam%C4%8Diak.jpg');
const geometry = new THREE.TorusKnotGeometry( 0.2, 1, 3, 6);
const material = new THREE.MeshBasicMaterial( { map: normalMap } );
const cube = new THREE.Mesh( geometry, material );
scene.add( cube );

camera.position.z = 2;

function animate() {
	requestAnimationFrame( animate );
    cube.rotation.x += 0.01;
	renderer.render( scene, camera );
}

animate();

