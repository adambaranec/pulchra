import * as THREE from "three";
import {createNoise2D} from "simplex-noise";
import { randFloat, randInt } from 'three/src/math/MathUtils';
const noise = createNoise2D();
let canvas = document.getElementById('canvas');
let input = document.getElementById('input');
let canvasRecDialog = document.getElementById('canvas-rec');
let recSaveDialog = document.getElementById('record-save');
let welcomeDialog = document.getElementById('welcome');
let closeCanvasRecD = document.getElementById('close-canvas-rec');
let closeRecSaveD = document.getElementById('close-record-save');
let approveRecord = document.getElementById('approve');
let saveFile = document.getElementById('save-file');
let gotIt = document.getElementById('got-it');
let fileNameInput = document.getElementById('file-name');
let errorP = document.getElementById('error');

sessionStorage.setItem('sessions', '-1');
sessionStorage.setItem('photos', '-1');
const width = window.innerWidth;
const height = window.innerHeight;

/* RENDERER */
/* MUST-HAVE CODE*/
/* THE ENVIRONMENT WHICH CONTROLS THE SCENE*/
let environment = {
  background: new THREE.Color(0,0,0),
  models: new Array(),
  oscillators: new Array()
};
const scene = new THREE.Scene();
scene.background = environment.background;
const aspect = width / height;
const camera = new THREE.PerspectiveCamera( 45, aspect, 0.1, 1000 );
camera.position.x = 0;
camera.position.y = 0;
camera.position.z = 4;
camera.lookAt(new THREE.Vector3(0,0,0));
const renderer = new THREE.WebGLRenderer();
renderer.setSize( width, height);
document.body.appendChild( renderer.domElement );

/* RECORDING THE CANVAS */
let stream = renderer.domElement.captureStream(60);
let mediaRecorder = new MediaRecorder(stream);
let chunks = [];
mediaRecorder.ondataavailable = function(e) {
  chunks.push(e.data);
}
mediaRecorder.onstop = function(e) {
  let blob = new Blob(chunks, { 'type' : 'video/webm' });
  let videoURL = URL.createObjectURL(blob);
  let a = document.createElement('a');
  a.href = videoURL;
  a.download = `PulchraVideo${sessionStorage.getItem('sessions')}.webm`;
  a.click();
  a.remove();
  chunks = [];
}

/* ANALYSING AUDIO */
let audioCtx;
let analyser;
let source;
let bufferLength;
let dataArray;
const getAverageVolume = (array) => {
  let values = 0;
  let average;
  let length = array.length;
  for (let i = 0; i < length; i++) {
    values += array[i];
  }
  average = values / length;
  return average;
}

const isValidUrl = (url) => {
  try {
      new URL(url);
      return true;
  } catch (e) {
      return false;
  }
}
/*SOME BASIC MATHS */
const posFromPixels = (x,y) => {
  let vector = new THREE.Vector3();
  vector.set(
    ( x / width ) * 2 - 1,
    - ( y / height ) * 2 + 1,
    0.5 );
  vector.unproject( camera );
  let dir = vector.sub( camera.position ).normalize();
  let distance = - camera.position.z / dir.z;
  let pos = camera.position.clone().add( dir.multiplyScalar( distance ) );
  return pos;
}
const pixelsFromPos = (pos) => {
  let vector = new THREE.Vector3();
  vector.set(pos.x,pos.y,pos.z);
  vector.project(camera);
  vector.x = Math.round((vector.x + 1) * width / 2);
  vector.y = Math.round((-vector.y + 1) * height / 2);
  return vector;
}
const posFromUV = (u,v) => {
  let vector = new THREE.Vector3();
  vector.set(
    ( u ) * 2 - 1,
    - ( v ) * 2 + 1,
    0.5 );
  vector.unproject( camera );
  let dir = vector.sub( camera.position ).normalize();
  let distance = - camera.position.z / dir.z;
  let pos = camera.position.clone().add( dir.multiplyScalar( distance ) );
  return pos;
}
const uvFromPixels = (x,y) => {
  let vector = new THREE.Vector3();
  vector.set(
    ( x / width ) * 2 - 1,
    - ( y / height ) * 2 + 1,
    0.5 );
  vector.unproject( camera );
  let dir = vector.sub( camera.position ).normalize();
  let distance = - camera.position.z / dir.z;
  let pos = camera.position.clone().add( dir.multiplyScalar( distance ) );
  let uv = new THREE.Vector2();
  uv.x = (pos.x + 1) / 2;
  uv.y = (pos.y + 1) / 2;
  return uv;
}

const getNormalizedScale = (geometry, targetSize) => {
  geometry.computeBoundingBox();
  const boundingBox = geometry.boundingBox;
  const width = boundingBox.max.x - boundingBox.min.x;
  const height = boundingBox.max.y - boundingBox.min.y;
  const depth = boundingBox.max.z - boundingBox.min.z;
  const maxDimension = Math.max(width, height, depth);
  const scaleFactor = targetSize / maxDimension;
  return new THREE.Matrix4().makeScale(scaleFactor, scaleFactor, scaleFactor);
}

/* MAIN FUNCTION FOR ANIMATION */ 
let angle = 0.0;
const animate = () => {
	requestAnimationFrame( animate );
  if (environment.globalMul != null){
    for (let c=0; c<=environment.globalMul.columns; c++){
      for (let r=0; r<=environment.globalMul.rows; r++){
        let x = ((width / environment.globalMul.columns) * c);
        let y = ((height / environment.globalMul.rows) * r);
        let w = (width / environment.globalMul.columns);
        let h = (height / environment.globalMul.rows);
        renderer.setViewport(x,y,w,h);
        renderer.setScissor(x,y,w,h);
        renderer.setScissorTest( true );
        renderer.render(scene,camera);
      }
    }
  } else {
    renderer.setScissorTest(false);
    renderer.setViewport(0,0,width,height); 
    renderer.render(scene,camera);
  }
  if (environment.background.functions && environment.background.parameters && environment.background.multipliers && environment.background.speeds){
    angle += (360/60) * environment.background.speeds[0];
    if (angle >= 360.0){ angle = 0.0;}
    let domain = environment.background;
    domain.functions.forEach((f,i)=>{
      switch(f){
        case "sin":
          switch(domain.parameters[i]){
            case "red": scene.background.r = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            case "green": scene.background.g = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            case "blue": scene.background.b = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            default: break;
          }
          break;
        case "cos":
          switch(domain.parameters[i]){
            case "red": scene.background.r = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            case "green": scene.background.g = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            case "blue": scene.background.b = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            default: break;
          }
          break;
        case "tan":
          switch(domain.parameters[i]){
            case "red": scene.background.r = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            case "green": scene.background.g = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            case "blue": scene.background.b = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i]; break;
            default: break;
          }
          break;
        case "amp":
          analyser.getFloatTimeDomainData(dataArray);
          const amp = (getAverageVolume(dataArray) + 1.0 / 2.0) * domain.multipliers[i];
          switch(domain.parameters[f]){
            case "red": scene.background.r = amp; break;
            case "green": scene.background.g = amp; break;
            case "blue": scene.background.b = amp; break;
            default: break;
          }
          break;
        default: break;
    }
    });
 }
  if (environment.models.length != 0){
     for (let m in environment.models){
      if (environment.models[m].rotation){
        switch (environment.models[m].rotation.axis){
          case "X": environment.models[m].mesh.rotation.x -= environment.models[m].rotation.speed / 10.0; break;
          case "Y": environment.models[m].mesh.rotation.y -= environment.models[m].rotation.speed / 10.0; break;
          case "Z": environment.models[m].mesh.rotation.z -= environment.models[m].rotation.speed / 10.0; break;
          default: break;
        }
        const speed = environment.models[m].rotation.orbitSpeed;
        if (environment.models[m].rotation.around){
          angle += (360/60)*speed;
          if (angle == 360.0){ angle = 0.0;}
        }
        let translation;
        switch (environment.models[m].rotation.around){
          case "X": 
          translation = environment.models[m].transform.elements[12];
          environment.models[m].mesh.position.y = Math.sin(angle * Math.PI / 180.0) * translation;
          environment.models[m].mesh.position.z = Math.cos(angle * Math.PI / 180.0) * translation;
          break;
          case "Y": 
          translation = environment.models[m].transform.elements[13];
          environment.models[m].mesh.position.x = Math.cos(angle * Math.PI / 180.0) * translation;
          environment.models[m].mesh.position.z = Math.sin(angle * Math.PI / 180.0) * translation;
          break;
          case "Z":  
          //translation = environment.models[m].transform.elements[14];
          environment.models[m].mesh.position.x = Math.sin(angle * Math.PI / 180.0);
          environment.models[m].mesh.position.y = Math.cos(angle * Math.PI / 180.0);
          break;
          default: break;
     }
  }
  if (environment.models[m].domains.length > 0){
    angle += (360/60);
    if (angle >= 360.0){ angle = 0.0;}
    for (let d in environment.models[m].domains){
      let domain = environment.models[m].domains[d];
      domain.functions.forEach((f,i)=>{
        switch(f){
          case "sin":
            switch(domain.parameters[i]){
              case "red": environment.models[m].mesh.material.color.r = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "green": environment.models[m].mesh.material.color.g = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "blue": environment.models[m].mesh.material.color.b = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "scale": 
              environment.models[m].mesh.scale.x = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i]; 
              environment.models[m].mesh.scale.y = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i];
              environment.models[m].mesh.scale.z = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              case "x":
              environment.models[m].mesh.position.x = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              case "y":
              environment.models[m].mesh.position.y = Math.sin(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              default: break;
            }
            break;
          case "cos":
            switch(domain.parameters[f]){
              case "red": environment.models[m].mesh.material.color.r = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "green": environment.models[m].mesh.material.color.g = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "blue": environment.models[m].mesh.material.color.b = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "scale":
              environment.models[m].mesh.scale.x = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i];
              environment.models[m].mesh.scale.y = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i];
              environment.models[m].mesh.scale.z = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              case "x":
              environment.models[m].mesh.position.x = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              case "y":
              environment.models[m].mesh.position.y = Math.cos(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              default: break;
            }
            break;
          case "tan":
            switch(domain.parameters[i]){
              case "red": environment.models[m].mesh.material.color.r = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "green": environment.models[m].mesh.material.color.g = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "blue": environment.models[m].mesh.material.color.b = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i]; break;
              case "scale":
              environment.models[m].mesh.scale.x = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i];
              environment.models[m].mesh.scale.y = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i];
              environment.models[m].mesh.scale.z = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              case "x":
              environment.models[m].mesh.position.x = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              case "y":
              environment.models[m].mesh.position.y = Math.tan(angle * domain.speeds[i]) * domain.multipliers[i];
              break;
              default: break;
            }
            break;
          case "amp":
            analyser.getFloatTimeDomainData(dataArray);
            const amp = (getAverageVolume(dataArray) + 1.0 / 2.0) * domain.multipliers[i];
            switch(domain.parameters[f]){
              case "red": environment.models[m].mesh.material.color.r = amp; break;
              case "green": environment.models[m].mesh.material.color.g = amp; break;
              case "blue": environment.models[m].mesh.material.color.b = amp; break;
              case "scale":
              environment.models[m].mesh.scale.x = amp;
              environment.models[m].mesh.scale.y = amp;
              environment.models[m].mesh.scale.z = amp;
              break;
              case "x":
              environment.models[m].mesh.position.x = amp;
              break;
              case "y":
              environment.models[m].mesh.position.y = amp;
              break;
              default: break;
            }
          }
        });
      }
    }
   }
  }
}
animate();

/* INTERPRETING */
const sendErr = (err) => {
 if (typeof err === 'string'){
  errorP.innerHTML = err;
 }
}

const color = (word) => {
if (typeof word === 'string'){
  switch (word){
    case "red":return new THREE.Color(1.0,0.0,0.0); 
    case "green":return new THREE.Color(0.0,1.0,0.0); 
    case "blue":return new THREE.Color(0.0,0.0,1.0); 
    case "yellow":return new THREE.Color(1.0,1.0,0.0); 
    case "magenta":return new THREE.Color(1.0,0.0,1.0); 
    case "cyan":return new THREE.Color(0.0,1.0,1.0); 
    case "orange":return new THREE.Color(1.0,0.4,0.0); 
    case "pink":return new THREE.Color(1.0,0.2,0.4); 
    case "purple":return new THREE.Color(0.2,0.0,0.5); 
    case "brown":return new THREE.Color(0.3,0.2,0.1); 
    case "beige":return new THREE.Color(0.5,0.4,0.3); 
    case "black":return new THREE.Color(0.0,0.0,0.0); 
    case "white":return new THREE.Color(1.0,1.0,1.0); 
    case "grey":return new THREE.Color(0.3,0.3,0.3); 
    case "gray":return new THREE.Color(0.3,0.3,0.3); 
    default: return null;
     }
} else {
  return null;
}
}

const floats = (word) => {
if (typeof word === 'string'){
  const regex = /[+-]?([0-9]*[.])?[0-9]+/g;
  const result = word.match(regex);
  if (result != null){
    const values = [];
    result.forEach((n)=>{
    if (n.startsWith(".")){
      n = "0" + n;
    }
    values.push(parseFloat(n));
    });
    return values;
  } else {
    return null;
  }
} else {
  return null;
}
}

const medium = (word) =>{
  switch(word){
    case "cube": return {medium: "shape",variant: "cube"};
    case "sphere": return {medium: "shape",variant: "sphere"};
    case "square": return {medium: "shape",variant: "square"};
    case "circle": return {medium: "shape",variant: "circle"};
    case "line": return {medium: "shape", variant: "line"};
    case "cone": return {medium: "shape",variant: "cone"};
    case "cylinder": return {medium: "shape",variant: "cylinder"};
    case "torus": return {medium: "shape",variant: "torus"};
    case "torusKnot": return {medium: "shape",variant: "torusKnot"};
    case "screen": return {medium: "background"};
    case "mul": return {medium: "multiplication"};
    case "camera": return {medium: "camera"};
    case "light": return {medium: "light"};
    default: return null;
  }
}

const model = (c) => {
if (typeof c === 'string'){
  let command = c.split(' ');
  if (medium(command[0]).medium == "shape"){
    let modelObj = {};
    let geometry = undefined;
    let material = new THREE.MeshPhongMaterial();
    modelObj.transform = new THREE.Matrix4();
    modelObj.rotation = {};
    modelObj.domains = [];
    switch (medium(command[0]).variant){
      case "cube": geometry = new THREE.BoxGeometry(1,1,1); break;
      case "sphere": geometry = new THREE.SphereGeometry(1,60,60); break;
      case "square": geometry = new THREE.PlaneGeometry(1,1); break;
      case "circle": geometry = new THREE.CircleGeometry(1,60); break;
      case "cone": geometry = new THREE.ConeGeometry(1,1,60,30); break;
      case "cylinder": geometry = new THREE.CylinderGeometry(1,1,1,60,60); break;
      case "torus": geometry = new THREE.TorusGeometry(1, undefined, 60, 60); break;
      case "torusKnot": geometry = new THREE.TorusKnotGeometry(0.65,0.5,80,80); break;
      default: break;
    }
    if (command.length == 1){
      geometry.scale(1.0,1.0,1.0);
    } else {
      for (let i= 1; i<command.length; i++){
            if (!isNaN(parseFloat(command[i]))){
              let value = parseFloat(command[i]);
              if (value >= 0.0 && value <= 1.0){
                  let matrix = modelObj.transform;
                  matrix.multiply(new THREE.Matrix4().makeScale(parseFloat(command[i]),parseFloat(command[i]),parseFloat(command[i])));
                  modelObj.transform = matrix;
              } else {
                sendErr("Allowed range is 0 - 1");
              }
          } else if (color(command[i]) != null){
            material.color = color(command[i]);
          } else if (command[i].startsWith("rgb(") && command[i].endsWith(")")){
            if (domain(command[i]).functions.length > 0 && domain(command[i]).multipliers.length > 0 &&
              domain(command[i]).speeds.length > 0 && domain(command[i]).parameters.length > 0){
              const paramRegex = /(amp|sin|cos|tan)\*(\.\d+|\d+(\.\d+)?)|(amp|sin|cos|tan)|(\.\d+|\d+(\.\d+)?)|\((\.\d+|\d+(\.\d+)?),(\.\d+|\d+(\.\d+)?)\)/g;
              const result = command[1].match(paramRegex);
              if (result != null){
                let stable = {};
                result.map((r,i)=>{
                  if (!isNaN(parseFloat(r))){
                    if (parseFloat(r) > 1.0 || parseFloat(r) < 0.0){
                      sendErr("Allowed range 0 - 1");
                    } else {
                      switch(i){
                        case i == 0: stable.red = parseFloat(r); break;
                        case i == 1: stable.green = parseFloat(r); break;
                        case i == 2: stable.blue = parseFloat(r); break;
                        default: break;
                      }
                    }
                  }
                });
                material.color = new THREE.Color(0.0,0.0,0.0);
                let domain = domain(command[i])
                domain.stable = stable;
                modelObj.domains.push(domain);
              } else {
                sendErr("Invalid domain");
              }
            } else {
            let values = floats(command[1].slice(command[1].indexOf("(")+1,command[1].indexOf(")")));
            if (values.length == 3){
              if (values[0] >= 0.0 && values[0] <= 1.0 && values[1] >= 0.0 && values[1] <= 1.0  && values[2] >= 0.0 && values[2] <= 1.0){
                material.color = new THREE.Color(values[0],values[1],values[2]);
              } else {
                sendErr("Allowed range 0 - 1");
              }
            } else if (values.length != 3) {
              if (command[1].includes(",") && command[1].split(',').length <= 2){
                sendErr("RGB must have three parameters");
              } 
            }   
            } 
          } else if (command[i].startsWith("rot") && command[i].includes('(') && command[i].endsWith(")")){
            if (command[i][3] == "X" || command[i][3] == "Y" || command[i][3] == "Z"){
              if (floats(command[i]).length == 0){
                sendErr("Provide a speed to rotate");
              }else{
                modelObj.rotation.speed = floats(command[i])[0];
                modelObj.rotation.axis = command[i][3];
              }
            } else if (command[i][5] == "X" || command[i][5] == "Y" || command[i][5] == "Z"){
              if (command[i][3] == "A" && command[i][4] == "r"){
                if (floats(command[i]).length == 0){
                  sendErr("Provide a speed to rotate around");
                }else{
                  modelObj.rotation.orbitSpeed = floats(command[i])[0];
                  modelObj.rotation.around = command[i][5];
                }
              }
            } else {
              if (command[i][3] == "(" || command[i][5] == "("){
                sendErr("Provide an axis to rotate around - either X, Y or Z.");
              } else if (command[i][4] == "(" || command[i][6] == "("){
                sendErr("Such axis does not exist - choose either X, Y or Z.");     
              } 
            }
          } else if (command[i].startsWith('[') && command[i].endsWith(']')){
            const coords = floats(command[i]);
            if (coords.length == 2){
              if (coords[0] >= -1.0 && coords[0] <= 1.0 && coords[1] >= -1.0 && coords[1] <= 1.0){
                let matrix = modelObj.transform;
                if (geometry.boundingBox === null){
                  geometry.computeBoundingBox();
                }
                let meshWidth = geometry.boundingBox.max.x - geometry.boundingBox.min.x;
                let meshHeight = geometry.boundingBox.max.y - geometry.boundingBox.min.y;
                let x  = (coords[0] + 1.0) / 2.0;
                let y  = (coords[1] + 1.0) / 2.0;
                let realX = posFromPixels(width * x,height * y).x / modelObj.transform.elements[0];
                let realY = posFromPixels(width * x,height * y).y * -1.0 / modelObj.transform.elements[0];
                if (realX!=0){
                  //realX -= (meshWidth * modelObj.transform.elements[0]);
                  //realX *= modelObj.transform.elements[0];
                } else if (realY!=0){
                  //realY -= (meshHeight * modelObj.transform.elements[0]);
                  //realY *= modelObj.transform.elements[1];
                }
                matrix.multiply(new THREE.Matrix4().makeTranslation(realX,realY,0));
                modelObj.transform = matrix;
                if(modelObj.rotation == {}){modelObj.rotation = undefined}
              } else {
                sendErr("Allowed range -1 - 1");
              }
            } else {
              if (command[i].includes(",") && command[i].split(',').length <= 1){
                sendErr("Expected two coordinates");
               } else if (command[i].includes(',') && command[i].split(',').length == 2){
                 domain(command[i]) != null ? modelObj.domains.push(domain(command[i])) : sendErr("Invalid domain");
               }
            }
          } else if (command[i].startsWith("mat(") && command[i].endsWith(")")){
            const slice = command[i].slice(4,command[i].length - 1);
            let colors = new Array();
            if (slice.includes(',')){
              const colorNames = slice.split(',');
              for (let c in colorNames){
                color(colorNames[c]) != null ? colors.push(color(colorNames[c])) : sendErr("Invalid name for color - e.g. RGB is not allowed");
              }
              const count = geometry.attributes.position.count;
              geometry.setAttribute('color', new THREE.BufferAttribute(new Float32Array(3 * count), 3));
              let colorAttrib = geometry.attributes.color;
              for (let i=0; i<count; i++){
                let index = randInt(0,colors.length - 1);
                colorAttrib.setXYZ(i, colors[index].r,colors[index].g,colors[index].b);
              }
              material.vertexColors = true;
            } else {
              sendErr("Just a color name would be enough - use at least two colors for this");
            }
          } else if (command[i].startsWith("noise(") && command[i].endsWith(")")){
            const colors = command[i].slice(6,command[i].length - 1).split(',');
            const first = color(colors[0]);
            const second = color(colors[1]);
            const width = 512;
            const height = 512;
            let x = 0;
            let y = 0;
            const compSize = 4;
            const data = new Uint8Array(compSize * width * height);
            for ( let i = 0; i < width * height; i ++ ) {
              x = i;
              if (i % width == 0 && i != 0){y += 1;}
              const noiseValue = noise(x,y);
              const stride = i * compSize;
              data[ stride ] = new THREE.Color().lerpColors(first,second,255).r * noiseValue;
              data[ stride + 1 ] = new THREE.Color().lerpColors(first,second,255).g * noiseValue;
              data[ stride + 2 ] = new THREE.Color().lerpColors(first,second,255).b * noiseValue; 
              data[ stride + 3 ] = 255;
            }
            const texture = new THREE.DataTexture(data, width, height);
            texture.needsUpdate = true;
            texture.generateMipmaps = true;
            material = new THREE.MeshPhongMaterial({ map: texture});
          } else if (command[i].startsWith("tex(") && command[i].endsWith(")")) {
            let url = command[i].slice(5,command[i].length - 2);
            if (isValidUrl(url)){
              const textureLoader = new THREE.TextureLoader();
              const normalMap = textureLoader.load(url);
              material = new THREE.MeshPhongMaterial( { map: normalMap } );
            } else {
              sendErr("Invalid URL");
            }
          } else if (command[i].includes("amp") || command[i].includes("sin") || command[i].includes("cos") || command[i].includes("tan")){
            const paramRegex = /(amp|sin|cos|tan)\*(\.\d+|\d+(\.\d+)?)|(amp|sin|cos|tan)|(\.\d+|\d+(\.\d+)?)|\((\.\d+|\d+(\.\d+)?),(\.\d+|\d+(\.\d+)?)\)/g;
            const result = command[i].match(paramRegex);
            if (result != null){
              result.map((r)=>{
                if (!isNaN(parseFloat(r))){
                  if (parseFloat(r) > 1.0 || parseFloat(r) < 0.0){
                    sendErr("Allowed range 0 - 1");
                  }
                }
              });
              modelObj.domains.push(domain(command[i]));
            } else {
              sendErr("Invalid domain");
            }
          } else {
            sendErr("Unknown parameter. Allowed: radius, color, rotation, texture...");
          }
      }
    }
    if (typeof geometry !== 'undefined'){
      let mesh = new THREE.Mesh(geometry, material);
      mesh.applyMatrix4(modelObj.transform);
      modelObj.mesh = mesh;
      return modelObj;
    } else {
      sendErr("Unknown shape. Allowed: cube, sphere, circle, torus...");
      return null;
    }
  } else {
    return null;
  }
} else {
  return null;
}
}

const screen = (c) => {
  if (typeof c === 'string'){
    let command = c.split(' ');
    if (c == "screen"){
      sendErr("Missing parameters");
      return null;
    } else if (command.length > 2){
      sendErr("Too many parameters");
      return null;
    } else {
          if (!isNaN(parseFloat(command[1]))){
            let scale = parseFloat(command[1]);
            if (scale <= 1.0 && scale >= 0.0){
              return new THREE.Color(scale,scale,scale);
            } else {
              sendErr("Allowed range 0 - 1");
            }
          } else if (command[1].startsWith("rgb(") && command[1].endsWith(")")){
            if (domain(command[1]).functions.length > 0 && domain(command[1]).multipliers.length > 0 &&
              domain(command[1]).speeds.length > 0 && domain(command[1]).parameters.length > 0){
              const paramRegex = /(amp|sin|cos|tan)\*(\.\d+|\d+(\.\d+)?)|(amp|sin|cos|tan)|(\.\d+|\d+(\.\d+)?)|\((\.\d+|\d+(\.\d+)?),(\.\d+|\d+(\.\d+)?)\)/g;
              const result = command[1].match(paramRegex);
              if (result != null){
                let stable = {};
                result.map((r,i)=>{
                  if (!isNaN(parseFloat(r))){
                    if (parseFloat(r) > 1.0 || parseFloat(r) < 0.0){
                      sendErr("Allowed range 0 - 1");
                    } else {
                      switch(i){
                        case i == 0: stable.red = parseFloat(r); break;
                        case i == 1: stable.green = parseFloat(r); break;
                        case i == 2: stable.blue = parseFloat(r); break;
                        default: break; 
                      }
                    }
                  }
                });
                returnObj = domain(command[1]);
                returnObj.stable = stable;
                return returnObj;
              } else {
                sendErr("Invalid domain");
                return null;
              }
            } else {
            let values = floats(command[1].slice(command[1].indexOf("(")+1,command[1].indexOf(")")));
            if (values.length == 3){
              if (values[0] >= 0.0 && values[0] <= 1.0 && values[1] >= 0.0 && values[1] <= 1.0  && values[2] >= 0.0 && values[2] <= 1.0){
                return new THREE.Color(values[0],values[1],values[2]);
              } else {
                sendErr("Allowed range 0 - 1");
                return null;
              }
            } else if (values.length != 3) {
              if (command[1].includes(",") && command[1].split(',').length <= 2){
                sendErr("RGB must have three parameters");
                return null;
              } 
            }   
            } 
          } else if (color(command[1]) != null){
            return color(command[1]);
          } else if (command[1].startsWith("noise(") && command[1].endsWith(")")){
            const colors = command[1].slice(6,command[1].length - 1).split(',');
            const first = color(colors[0]);
            const second = color(colors[1]);
            const w = width;
            const h = height;
            let x = 0;
            let y = 0;
            const compSize = 4;
            const data = new Uint8Array(compSize * width * height);
            for ( let i = 0; i < width * height; i ++ ) {
              x = i;
              if (i % width == 0 && i != 0){y += 1;}
              const noiseValue = noise(x,y) + 1.0 / 2.0;
              const stride = i * 4;
              data[ stride ] = new THREE.Color().lerpColors(first,second,255).r * noiseValue;
              data[ stride + 1 ] = new THREE.Color().lerpColors(first,second,255).g * noiseValue;;
              data[ stride + 2 ] = new THREE.Color().lerpColors(first,second,255).b * noiseValue; 
              data[ stride + 3 ] = 255;
            }
            const texture = new THREE.DataTexture(data, w, h);
            texture.needsUpdate = true;
            texture.generateMipmaps = true;
            return texture;
          } else if (command[1].startsWith("tex(") && command[1].endsWith(")")) {
            let url = command[1].slice(5,command[1].length - 2);
            if (isValidUrl(url)){
              if (url.endsWith('.png') || url.endsWith('.jpg') || url.endsWith('.jpeg') || url.endsWith('.svg')){
                const textureLoader = new THREE.TextureLoader();
                const normalMap = textureLoader.load(url);
                return normalMap;
              }else if (url.endsWith('.gif') || url.endsWith('.mp4') || url.endsWith('.webm') || url.endsWith('.mpg')){
                let redundantVideos = document.querySelectorAll('video');
                if (redundantVideos.length > 0){
                  redundantVideos.forEach((v)=>{
                    v.remove();
                  });
                }
                let video = document.createElement('video');
                video.src = url;
                video.load();
                video.play();
                video.loop = true;
                video.muted = true;
                let videoTexture = new THREE.VideoTexture(video);
                videoTexture.minFilter = THREE.LinearFilter;
                videoTexture.magFilter = THREE.LinearFilter;
                videoTexture.format = THREE.RGBAFormat;
                videoTexture.generateMipmaps = false;
                if (video.readyState === video.HAVE_ENOUGH_DATA){
                  videoTexture.needsUpdate = true;
                }
                return videoTexture;
              }
            } else {
              sendErr("Invalid URL");
            }
          } else {
            sendErr("Unknown parameter. Allowed: grayscale, RGB or texture");
            return null;
          }
    }
  } else {
    return null;
  }
}

const mul = (c) => {
  if (typeof c === 'string'){
   const command = c.split(' ');
   if (command.length == 1){
     sendErr("Give at least one number");
     return null;
   } else if (command.length == 2){
    let value = parseInt(command[1]);
    if (!isNaN(value)){ return {rows: value, columns: value}; }else{
      sendErr("Could not parse the number");
      return null;
    }
   } else if (command.length == 3){
    let rows = parseInt(command[1]);
    let columns = parseInt(command[2]);
    if (!isNaN(rows) && !isNaN(columns)){ return {rows: rows, columns: columns}; }else{
      sendErr("Could not parse neither rows nor columns");
      return null;
    }
   } else {
    sendErr("Too many parameters");
    return null;
   }
  } else {
    return null;
  }
}

const param = (w) => {
if (typeof w === 'string'){

} else {
  return null;
}
}

const domain = (c) => {
  if (typeof c === 'string'){
  let domain = {
    functions: [],
    parameters: [],
    multipliers: [],
    speeds: []
  };
  const paramRegex = /(amp|sin|cos|tan)\*(\.\d+|\d+(\.\d+)?)|(amp|sin|cos|tan)|(\.\d+|\d+(\.\d+)?)|\((\.\d+|\d+(\.\d+)?),(\.\d+|\d+(\.\d+)?)\)/g;
  if (c.indexOf("(") == 3 && c.endsWith(")")){
    let paramIndex = -1;
    const name = c.slice(0,c.indexOf("("));
    const args = c.slice(c.indexOf('(')+1,c.lastIndexOf(')'));
    switch(name){
     case "rgb": 
     const params = args.match(paramRegex);
     params.map((p,i)=>{
      if (p.startsWith('amp') || p.startsWith('sin') || p.startsWith('cos') || p.startsWith('tan')){
        paramIndex++;
        switch(p.slice(0,3)){
          case "sin": domain.functions.push("sin"); 
          switch(paramIndex){
            case 0: domain.parameters.push("red"); break;
            case 1: domain.parameters.push("green"); break;
            case 2: domain.parameters.push("blue"); break;
           } 
          break;
          case "cos": domain.functions.push("cos"); 
          switch(paramIndex){
            case 0: domain.parameters.push("red"); break;
            case 1: domain.parameters.push("green"); break;
            case 2: domain.parameters.push("blue"); break;
           } 
          break;
          case "tan": domain.functions.push("tan"); 
          switch(paramIndex){
            case 0: domain.parameters.push("red"); break;
            case 1: domain.parameters.push("green"); break;
            case 2: domain.parameters.push("blue"); break;
           } 
          break;
          case "amp": domain.functions.push("amp");
          switch(paramIndex){
            case 0: domain.parameters.push("red"); break;
            case 1: domain.parameters.push("green"); break;
            case 2: domain.parameters.push("blue"); break;
           } 
          break;
      }
      if (p.includes('*')){
        const param = p.split('*');
        param.splice(0,1);
        let val = 1.0;
        if (param.length != 0){
          param.map((v)=>{val *= parseFloat(v)});
        }
        domain.multipliers.push(val);
        if (domain.functions[domain.functions.length - 1] == 'amp'){
          domain.speeds.push(null);
        } else {
          domain.speeds.push(1.0);
        }
     } else {
      domain.multipliers.push(1.0);
      if (domain.functions[domain.functions.length - 1] == 'amp'){
        domain.speeds.push(null);
      } else {
        domain.speeds.push(1.0);
      }
     }
     } else if (p.startsWith('(') && p.endsWith(')')){
        const args = p.slice(p.indexOf("(")+1,p.indexOf(")")).split(',');
        if (args.length != 2){
          sendErr("Invalid number of arguments - expected speed and multiplier");
        } else {
          if (domain.functions[domain.functions.length - 1] != 'amp'){
            parseFloat(args[0]) ? domain.speeds[domain.functions.length - 1] = parseFloat(args[0]) : sendErr("Could not parse the speed");
            parseFloat(args[1]) ? domain.multipliers[domain.functions.length - 1] = parseFloat(args[1]) : sendErr("Could not parse the multiplier");
          }else{
            sendErr("Amplitude argument needs only a multiplier");
          }
        }
     } else if (!isNaN(parseFloat(p)) || !isNaN(parseInt(p))){paramIndex++} 
    });
       return domain;
    }
  } else if (c.startsWith("[") && c.endsWith("]")){
    let paramIndex = -1;
    const args = c.slice(c.indexOf('[')+1,c.lastIndexOf(']'));
    const params = args.match(paramRegex);
    params.map((p,i)=>{
      console.log(parseFloat(p));
    if (p.startsWith('amp') || p.startsWith('sin') || p.startsWith('cos') || p.startsWith('tan')){
      paramIndex++;
      switch(p.slice(0,3)){
        case "sin": domain.functions.push("sin"); 
        switch(domain.functions.length - 1){
          case 0: domain.parameters.push("x"); break;
          case 1: domain.parameters.push("y"); break;
         }
        break;
        case "cos": domain.functions.push("cos"); 
        switch(domain.functions.length - 1){
          case 0: domain.parameters.push("x"); break;
          case 1: domain.parameters.push("y"); break;
         }
        break;
        case "tan": domain.functions.push("tan"); 
        switch(domain.functions.length - 1){
          case 0: domain.parameters.push("x"); break;
          case 1: domain.parameters.push("y"); break;
         }
        break;
        case "amp": domain.functions.push("amp"); 
        switch(domain.functions.length - 1){
          case 0: domain.parameters.push("x"); break;
          case 1: domain.parameters.push("y"); break;
         }
        break;
      }
      if (p.includes('*')){
        const param = p.split('*');
        param.splice(0,1);
        let val = 1.0;
        if (param.length != 0){
          param.map((v)=>{val *= parseFloat(v)});
        }
        domain.multipliers.push(val);
        if (domain.functions[domain.functions.length - 1] == 'amp'){
          domain.speeds.push(null);
        } else {
          domain.speeds.push(1.0);
        }
      } else {
        domain.multipliers.push(1.0);
        if (domain.functions[domain.functions.length - 1] == 'amp'){
          domain.speeds.push(null);
        } else {
          domain.speeds.push(1.0);
        }
      }
    } else if (p.startsWith('(') && p.endsWith(')')){
      const args = p.slice(p.indexOf("(")+1,p.indexOf(")")).split(',');
      if (args.length != 2){
        sendErr("Invalid number of arguments - expected speed and multiplier");
      } else {
        if (domain.functions[domain.functions.length - 1] != 'amp'){
          parseFloat(args[0]) ? domain.speeds[domain.functions.length - 1] = parseFloat(args[0]) : sendErr("Could not parse the speed");
          parseFloat(args[1]) ? domain.multipliers[domain.functions.length - 1] = parseFloat(args[1]) : sendErr("Could not parse the multiplier");
        }else{
          sendErr("Amplitude argument needs only a multiplier");
        }
      }
    } else if (!isNaN(parseFloat(p)) || !isNaN(parseInt(p))){paramIndex++} 
   }
   );
   return domain;
  } else if (c.startsWith('amp') || c.startsWith('sin') || c.startsWith('cos') || c.startsWith('tan')){
    domain.functions.push(c.slice(0,3));
    domain.parameters.push('scale');
    c.startsWith('amp') ? domain.speeds.push(null) : domain.speeds.push(1.0);
    if (c.includes('*')){
      const param = c.split('*');
      param.splice(0,1);
      let val = 1.0;
      if (param.length != 0){
        param.map((v)=>{val *= parseFloat(v)});
      }
      domain.multipliers.push(val);
    } else if (c.includes('(') && c.includes(')')){
      const args = c.slice(c.indexOf("(")+1,c.indexOf(")")).split(',');
      if (args.length != 2){
        sendErr("Invalid number of arguments - expected speed and multiplier");
      } else {
        if (domain.functions[domain.functions.length - 1] != 'amp'){
          parseFloat(args[0]) ? domain.speeds[domain.functions.length - 1] = parseFloat(args[0]) : sendErr("Could not parse the speed");
          parseFloat(args[1]) ? domain.multipliers[domain.functions.length - 1] = parseFloat(args[1]) : sendErr("Could not parse the multiplier");
        }else{
          sendErr("Amplitude argument needs only a multiplier");
        }
      }
    } else {
      domain.multipliers.push(1.0);
    };
    return domain;
  } 
  } else {
    return null;
  }
}

const interpret = () => {
  if (typeof audioCtx === 'undefined' && typeof analyser === 'undefined' && typeof source === 'undefined'){
    audioCtx = new AudioContext();
    source = navigator.mediaDevices.getUserMedia({audio: true, video: false});
    analyser = audioCtx.createAnalyser();
    analyser.fftSize = 256;
    bufferLength = analyser.frequencyBinCount;
    dataArray = new Float32Array(bufferLength);
    source.then((stream)=>{
    const msSource = audioCtx.createMediaStreamSource(stream);
    msSource.connect(analyser);
    })
    .catch((err)=>{
    alert(err);
    });
  }
  scene.clear();
  const light = new THREE.DirectionalLight(new THREE.Color(1,1,1), 1.0);
  light.position.set(0,0,2);
  scene.add(light);
  environment.models = new Array();
  const code = String(input.value).trim();
  errorP.innerHTML = '';
  if (code != ""){
    let lines = code.split('\n');
    for (let l in lines){
      lines[l].trim();
      if (lines[l].includes(';')){
      let commands = lines[l].split(';');
      if (code[code.length - 1] == ";" || lines[l][lines[l].length - 1] == ";"){
        commands.pop();
      }
      for (let c in commands){
        let words = commands[c].trim().split(' ');
        if (medium(words[0]) != null){
          switch (medium(words[0]).medium){
            case "shape": {
              let shape = model(commands[c].trimStart());
              if (shape != null){environment.models.push(shape)}
            }; break;
            case "background": {
              let back = screen(commands[c].trimStart());
              if (back != null){environment.background = back}              
            }; break;
            case "multiplication": {
              let m = mul(commands[c].trimStart());
              if (m != null){environment.globalMul = m}
            }; break;
            case "camera": {}; break;
            case "light": {}; break;
            default: sendErr("What to set?"); break;
           }
         } else {
          sendErr("Unknown thing to render or play with");
         }
      }
     } else {
      let words = lines[l].trim().split(' ');
      if (words[0][0] === undefined){words.shift()}      
      if (medium(words[0]) != null){
        switch (medium(words[0]).medium){
          case "shape": {
            let shape = model(lines[l].trimStart());
            if (shape != null){environment.models.push(shape)}
            }; break;
            case "background": {
            let back = screen(lines[l].trimStart());
            if (back != null){environment.background = back}
            }; break;
          case "multiplication": {
            let m = mul(lines[l].trimStart());
            if (m != null){environment.globalMul = m}
          }; break;
          case "camera": {}; break;
          case "light": {}; break;
          default: sendErr("What to set?"); break;
         }
       } else {
        sendErr("Unknown thing to render or play with");
       }
     }
    }
  } else {
    environment.background = new THREE.Color(0,0,0);
  }
  if (!environment.background.functions && !environment.background.parameters && !environment.background.multipliers && !environment.background.speeds){
    scene.background = environment.background;
  } else {
    const stableColors = environment.background.stable;
    for (entry in Object.entries(stableColors)){
      switch(entry[0]){
        case "red": scene.background.r = entry[1]; break;
        case "green": scene.background.g = entry[1]; break;
        case "blue": scene.background.b = entry[1]; break;
        default: break;
      }
    }
  }
  if (environment.models.length != 0){
    for (let m in environment.models){
      scene.add(environment.models[m].mesh);
    }
  }
}
/* ALL JS THINGS */
      window.onload = () => {
      welcomeDialog.showModal();
      }
      window.onresize = (e) =>{
      renderer.setSize(window.innerWidth, window.innerHeight );
      camera.aspect = window.innerWidth / window.innerHeight;
      //camera.updateProjectionMatrix();
      renderer.setViewport(0,0,window.innerWidth,window.innerHeight);
      }
      closeCanvasRecD.onclick = (e) => {canvasRecDialog.close();}
      closeRecSaveD.onclick = (e) => {recSaveDialog.close();}
      gotIt.onclick = (e) => {welcomeDialog.close();}
      approveRecord.onclick = (e) => {
        canvasRecDialog.close();
      }
      /*saveFile.onclick = (e) => {
        let name = fileNameInput.value;
        if (name != "" && name != null){
        }
        else {
        }
        recSaveDialog.close();
      }*/
      input.addEventListener('keydown', (e)=>{
       if (e.ctrlKey && e.key == 'Enter'){
        interpret();
       } else if (e.ctrlKey && e.key == 'd'){
        window.open('https://github.com/adambaranec/pulchra/blob/main/docs.md', '_blank', 'noopener,noreferrer');
       } else if (e.ctrlKey && e.key == 'r'){
        let sessions = parseInt(sessionStorage.getItem('sessions'));
        sessions += 1;
        sessionStorage.setItem('sessions', sessions.toString());
        mediaRecorder.start();
        alert('To stop recording press CTRL + S');
       } else if (e.ctrlKey && e.key == 'p'){
        requestAnimationFrame(() => {
          let photoNum = parseInt(sessionStorage.getItem('photos'));
          photoNum += 1;
          sessionStorage.setItem('photos', photoNum.toString());
          let image = renderer.domElement.toDataURL('image/jpeg',0.95);
          let a = document.createElement('a');
          a.href = image;
          a.download = `PulchraPhoto${sessionStorage.getItem('photos')}.jpg`;
          a.click();
          a.remove();
        });
       } else if (e.ctrlKey && e.key == 's'){
        mediaRecorder.stop();
       } 
      });