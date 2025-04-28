import * as THREE from "three";
const width = window.innerWidth;
const height = window.innerHeight;

/* RENDERER */
/* MUST-HAVE CODE*/
/* THE ENVIRONMENT WHICH CONTROLS THE SCENE*/
const scene = new THREE.Scene();
scene.background = new THREE.Color(0, 0, 0);
const aspect = width / height;
const camera = new THREE.PerspectiveCamera(45, aspect, 0.1, 1000);
camera.position.x = 0;
camera.position.y = 0;
camera.position.z = 4;
camera.lookAt(new THREE.Vector3(0, 0, 0));
const renderer = new THREE.WebGLRenderer(document.getElementById('canvas'));
renderer.setSize(width, height);

const animate = () => {
	requestAnimationFrame(animate);
	renderer.setViewport(0, 0, width, height);
	renderer.render(scene, camera);
}
