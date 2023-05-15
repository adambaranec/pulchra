import * as THREE from 'three';
import {createNoise2D} from "simplex-noise";
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

/* RENDERER */
/* MUST-HAVE CODE*/
/* THE ENVIRONMENT WHICH CONTROLS THE SCENE*/
let environment = {
  background: new THREE.Color(0,0,0),
  models: new Array(),
  oscillators: new Array()
};
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera( 25, window.innerWidth / window.innerHeight, 0.1, 1000 );
camera.position.x = 0;
camera.position.y = 0;
camera.position.z = 4;
const renderer = new THREE.WebGLRenderer();
renderer.setSize( window.innerWidth, window.innerHeight );
document.body.appendChild( renderer.domElement );
function animate() {
	requestAnimationFrame( animate );
	renderer.render( scene, camera );
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
    case  "red":return new THREE.Color(1.0,0.0,0.0); 
      case "green":return new THREE.Color(0.0,1.0,0.0); 
     case"blue":return new THREE.Color(0.0,0.0,1.0); 
       case  "yellow":return new THREE.Color(1.0,1.0,0.0); 
        case "magenta":return new THREE.Color(1.0,0.0,1.0); 
     case    "cyan":return new THREE.Color(0.0,1.0,1.0); 
       case  "orange":return new THREE.Color(1.0,0.45,0.0); 
     case    "pink":return new THREE.Color(1.0,0.6,0.8); 
       case  "purple":return new THREE.Color(0.2,0.0,0.5); 
      case   "brown":return new THREE.Color(0.3,0.2,0.1); 
      case   "beige":return new THREE.Color(0.5,0.4,0.3); 
      case   "black":return new THREE.Color(0.0,0.0,0.0); 
      case   "white":return new THREE.Color(1.0,1.0,1.0); 
     case    "grey":return new THREE.Color(0.3,0.3,0.3); 
     case    "gray":return new THREE.Color(0.3,0.3,0.3); 
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
    let material = new THREE.MeshBasicMaterial();
    switch (medium(command[0]).variant){
      case "cube": geometry = new THREE.BoxGeometry(1,1,1); break;
      case "sphere": geometry = new THREE.SphereGeometry(1,60,60); break;
      case "square": geometry = new THREE.PlaneGeometry(1,1); break;
      case "circle": geometry = new THREE.CircleGeometry(1,60); break;
      case "cone": geometry = new THREE.ConeGeometry(1,1,60,30); break;
      case "cylinder": geometry = new THREE.CylinderGeometry(1,1,1,60,60); break;
      case "torus": geometry = new THREE.TorusGeometry(1); break;
      case "torusKnot": geometry = new THREE.TorusKnotGeometry(1,1); break;
      default: break;
    }
    if (command.length == 1){
      geometry.scale = 1.0;
    } else {
      for (let i= 1; i<command.length; i++){
        if (!isNaN(parseFloat(command[i]))){
          geometry.scale = parseFloat(command[i]);
        } else if (color(command[i]) != null){
          material.color = color(command[i]);
        } else if (command[i].startsWith("rgb(") && command[i].endsWith(")")){
          let values = floats(command[i].slice(command[i].indexOf("(")+1,command[i].indexOf(")")));
          if (values[0] >= 0.0 && values[0] <= 1.0 && values[1] >= 0.0 && values[1] <= 1.0  && values[2] >= 0.0 && values[2] <= 1.0){
            material.color = new THREE.MeshBasicMaterial({color: new THREE.Color(values[0],values[1],values[2])});
          } else {
            sendErr("Allowed range is 0 - 1");
          }
        } else if (command[i].startsWith("rot") && command[i].includes('(') && command[i].endsWith(")")){
          if (command[i][3] == "X" || command[i][3] == "Y" || command[i][3] == "Z"){
            modelObj.rotation = {speed: floats(command[i])[0], axis: command[i][3]};
          } else {
            if (command[i][3] == "("){
              sendErr("Provide an axis to rotate around - either X, Y or Z.");
            } else if (command[i][4] == "("){
              sendErr("Such axis does not exist - choose from X, Y or Z.");     
            }    
          }
        } else if (command[i].startsWith('[') && command[i].endsWith(']')){
          const coords = floats(command[i]);
          if (coords.length == 2){
            geometry.translate(coords[0],coords[1],0.0);
          } else {
            sendErr("Expected only two coordinates to translate");
          }
        } else if (command[i] == "mat"){
          material.vertexColors = true;
        } else if (command[i].startsWith("noise(") && command[i].endsWith(")")){
          const colors = command[i].slice(6,command[i].length - 1).split(',');
          const first = color(colors[0]);
          const second = color(colors[1]);
          const width = 128;
          const height = 128;
          const data = new Uint8Array(4 * width * height);
          for ( let i = 0; i < width * height; i ++ ) {
            let x = i;
            let y = 0;
            if (width % x == 0 && x != 0){y += 1;}
            const noiseValue = noise(x,y) + 1.0 / 2.0;
            const stride = i * 4;
            data[ stride ] = parseInt(first.lerp(second, noiseValue).r * 255.0);
            data[ stride + 1 ] = parseInt(first.lerp(second, noiseValue).g * 255.0);
            data[ stride + 2 ] = parseInt(first.lerp(second, noiseValue).b * 255.0);
            data[ stride + 3 ] = 255;
          }
          const texture = new THREE.DataTexture(data, width, height);
          texture.generateMipmaps = true;
          material = new THREE.MeshBasicMaterial({envMap: texture});
        } else {
          sendErr("Unknown parameter. Allowed: radius, color, rotation, texture...");
        }
      }
    }
    if (typeof geometry !== 'undefined'){
      modelObj.mesh = new THREE.Mesh(geometry, material);
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
        let values = floats(command[1].slice(command[1].indexOf("(")+1,command[1].indexOf(")")));
        if (values.length == 3){
          if (values[0] >= 0.0 && values[0] <= 1.0 && values[1] >= 0.0 && values[1] <= 1.0  && values[2] >= 0.0 && values[2] <= 1.0){
            return new THREE.Color(values[0],values[1],values[2]);
          } else {
            sendErr("Allowed range 0 - 1");
            return null;
          }
        } else {
          sendErr("RGB must have three parameters");
          return null;
        }   
      } else if (color(command[1]) != null){
        return color(command[1]);
      } else if (command[1].startsWith("noise(") && command[1].endsWith(")")){
        const colors = command[1].slice(6,command[1].length - 1).split(',');
        const first = color(colors[0]);
        const second = color(colors[1]);
        const width = window.innerWidth;
        const height = window.innerHeight;
        const data = new Uint8Array(4 * width * height);
        for ( let i = 0; i < width * height; i ++ ) {
          let x = i;
          let y = 0;
          if (width % x == 0 && x != 0){y += 1;}
          const noiseValue = noise(x,y) + 1.0 / 2.0;
          const stride = i * 4;
          data[ stride ] = parseInt(first.lerp(second, noiseValue).r * 255.0);
          data[ stride + 1 ] = parseInt(first.lerp(second, noiseValue).g * 255.0);
          data[ stride + 2 ] = parseInt(first.lerp(second, noiseValue).b * 255.0);
          data[ stride + 3 ] = 255;
        }
        const texture = new THREE.DataTexture(data, width, height);
        texture.generateMipmaps = true;
        return texture;
      } else {
        sendErr("Unknown parameter. Allowed: grayscale, RGB or texture");
        return null;
      }
    }
  } else {
    return null;
  }
}

const interpret = () => {
  environment.models = new Array();
  const code = String(input.value);
  errorP.innerHTML = '';
  if (code != ""){
    let lines = code.split('\n');
    for (let l in lines){
     if (lines[l].includes(';')){
      let commands = lines[l].split(';');
      if (code[code.length - 1] == ";"){
        commands.pop();
      }
      for (let c in commands){
        let words = commands[c].trimStart().split(' ');
        if (medium(words[0]) != null){
          switch (medium(words[0]).medium){
            case "shape": {
              let shape = model(commands[c]);
              if (shape != null){environment.models.push(shape)}
            }; break;
            case "background": {
              let back = screen(commands[c]);
              if (back != null){environment.background = back}              
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
      let words = lines[l].split(' ');
      if (words[0][0] === undefined){words.shift()}      
      if (medium(words[0]) != null){
        switch (medium(words[0]).medium){
          case "shape": {
            let shape = model(lines[l]);
            if (shape != null){environment.models.push(shape)}
            }; break;
            case "background": {
            let back = screen(lines[l]);
            if (back != null){environment.background = back}
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
  scene.background = environment.background;
}
/* ALL JS THINGS */
      window.onresize = (e) =>{
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      canvas.getContext('webgl2').viewport(0,0,canvas.width,canvas.height);
      }
      closeCanvasRecD.onclick = (e) => {canvasRecDialog.close();}
      closeRecSaveD.onclick = (e) => {recSaveDialog.close();}
      gotIt.onclick = (e) => {welcomeDialog.close();}
      approveRecord.onclick = (e) => {
        canvasRecDialog.close();
        let sessions = parseInt(sessionStorage.getItem('sessions'));
        sessions += 1;
        sessionStorage.setItem('sessions', sessions.toString());
        let canvasCaptureStream = canvas.captureStream(60);
        let recorder = new MediaRecorder(canvasCaptureStream, {mimeType: 'video/webm'});
        recorder.start();
        recorder.requestData();
        let file = [];
        recorder.ondataavailable = (e)=>{
          file.push(e.data);
        }
        input.addEventListener('keydown', (e) =>{
         if (e.ctrlKey && e.key == 's'){
           recorder.stop();
           recorder.onstop = (e)=>{
            let video = new Blob(file, {'type': 'video/webm'}); 
            let videoURL = URL.createObjectURL(video);  
            recSaveDialog.showModal();
            saveFile.onclick = (c)=>{
             let downloadElem = document.getElementById('file');
             downloadElem.href = videoURL;
             let name = fileNameInput.value;
               name = '.webm' ? downloadElem.download = `pulchra-${sessionStorage.getItem('sessions')}.webm` :
               downloadElem.download = `${name}.webm`; 
             downloadElem.onclick = (e) => {
               recSaveDialog.close();
               console.log(name);
             }
            }
           }
          }
        });
      }


      input.addEventListener('keydown', (e)=>{
       if (e.ctrlKey && e.key == 'Enter'){
        interpret();
       } else if (e.ctrlKey && e.key == 'h'){
        window.open('https://github.com/adambaranec/pulchra/blob/main/docs.md', '_blank', 'noopener');
       } else if (e.ctrlKey && e.key == 'r'){
        canvasRecDialog.showModal();
       }
       else if (e.ctrlKey && e.key == 'p'){
         recSaveDialog.showModal();
         saveFile.onclick = (c)=>{
         canvas.toBlob((img)=>{
          let sessions = parseInt(sessionStorage.getItem('sessions'));
          sessions += 1;
          sessionStorage.setItem('sessions', sessions.toString());
            let downloadElem = document.getElementById('file');
            downloadElem.href = URL.createObjectURL(img);
            let name = fileNameInput.value;
            name = '.jpg' ? downloadElem.download = `pulchra-${sessionStorage.getItem('sessions')}.jpg` :
            downloadElem.download = `${name}.jpg`;
          }, 'image/jpg');
         }
         downloadElem.onclick = (e) => {
          recSaveDialog.close();
        }
       }
      });