import { randInt } from 'three/src/math/MathUtils';
import * as THREE from "three";
const errorP = document.getElementById('error');

const sendErr = (errorParagraph, err) => {
    if (typeof err === 'string') {
        errorParagraph.innerHTML = err;
    }
}

const isValidUrl = (url) => {
  try {
      new URL(url);
      return true;
  } catch (e) {
    sendErr(errorP, "invalid url");
      return false;
  }
}

const color = (word) => {
        if (word.startsWith("rgb(") && word.endsWith(")")) {
            let values = floats(word.slice(word.indexOf("(") + 1, word.indexOf(")")));
            if (values.length == 3) {
                if (values[0] >= 0.0 && values[0] <= 1.0 && values[1] >= 0.0 && values[1] <= 1.0 && values[2] >= 0.0 && values[2] <= 1.0) {
                    return new THREE.Color(values[0], values[1], values[2]);
                } else {
                    sendErr(errorP, "Allowed range 0 - 1");
                    return null;
                }
            } else if (values.length != 3) {
                if (command[1].includes(",") && command[1].split(',').length <= 2) {
                    sendErr(errorP, "RGB must have three parameters");
                    return null;
                }
            }
        } else if (!word.includes("(") && !word.includes(")") && !word.includes("[") && !word.includes("]")) {
            switch (word) {
                case "red": return new THREE.Color('red');
                case "green": return new THREE.Color('green');
                case "blue": return new THREE.Color('blue');
                case "yellow": return new THREE.Color('yellow');
                case "magenta": return new THREE.Color('magenta');
                case "cyan": return new THREE.Color('cyan');
                case "orange": return new THREE.Color('orange');
                case "pink": return new THREE.Color('pink');
                case "purple": return new THREE.Color('purple');
                case "brown": return new THREE.Color('brown');
                case "beige": return new THREE.Color('beige');
                case "black": return new THREE.Color('black');
                case "white": return new THREE.Color('white');
                case "grey": return new THREE.Color('grey');
                case "gray": return new THREE.Color('gray');
                //case "azure": return new THREE.Color(0xF0FFFF);
                case "ivory": return new THREE.Color('ivory');
                case "khaki": return new THREE.Color('khaki');
                case "olive": return new THREE.Color('olive');
                default: sendErr(errorP, "Invalid color name");  return null;
            }
        } else {
          return null;
        }
}

const floats = (word) => {
    if (typeof word === 'string') {
        const regex = /[+-]?([0-9]*[.])?[0-9]+/g;
        const result = word.match(regex);
        if (result != null) {
            const values = [];
            result.forEach((n) => {
                if (n.startsWith(".")) {
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

const medium = (word) => {
    switch (word) {
        case "cube": return { medium: "shape", variant: "cube" }; 
        case "sphere": return { medium: "shape", variant: "sphere" };
        case "square": return { medium: "shape", variant: "square" };
        case "circle": return { medium: "shape", variant: "circle" };
        case "line": return { medium: "shape", variant: "line" };
        case "cone": return { medium: "shape", variant: "cone" };
        case "cylinder": return { medium: "shape", variant: "cylinder" };
        case "torus": return { medium: "shape", variant: "torus" };
        case "torusKnot": return { medium: "shape", variant: "torusKnot" };
        case "screen": return { medium: "background" };
        //case "camera": return { medium: "camera" };
        //case "light": return { medium: "light" };
        default: return null;
    }
}

const model = (command) => {
  let subjectToRender;
  let geometry = new THREE.BufferGeometry();
  let material = new THREE.MeshPhongMaterial();
    let modelObj = {
    rotation: {
     speed: 0,
     axis: ""
    },
    mesh: new THREE.Mesh(geometry, material)
  };
  if (command.length > 1){
    subjectToRender = medium(command[0]);
  } else if (command.length == 1) {
    subjectToRender = medium(command[0]);
  }
  if (subjectToRender.medium == "shape") {
            switch (subjectToRender.variant) {
            case "cube": geometry = new THREE.BoxGeometry(1, 1, 1); break;
            case "sphere": geometry = new THREE.SphereGeometry(1, 60, 60); break;
            case "square": geometry = new THREE.PlaneGeometry(1, 1); break;
            case "circle": geometry = new THREE.CircleGeometry(1, 60); break;
            case "cone": geometry = new THREE.ConeGeometry(1, 1, 60, 30); break;
            case "cylinder": geometry = new THREE.CylinderGeometry(1, 1, 1, 60, 60); break;
            case "torus": geometry = new THREE.TorusGeometry(1, undefined, 60, 60); break;
            case "torusKnot": geometry = new THREE.TorusKnotGeometry(0.65, 0.5, 80, 80); break;
            default: break;
        }
  }
  console.log(geometry);
  if (command.length == 1){
      geometry.scale(1.0, 1.0, 1.0);
      material.color = new THREE.Color(1.0, 1.0, 1.0);
  } else {
    let words = command.slice(1);
    if (words.length >= 1){
      words.forEach((word,i)=>{
       if (!isNaN(parseFloat(word))) {
         let value = parseFloat(word);
         if (value >= 0.0 && value <= 1.0) {
             geometry.scale(value, value, value);
          } else {
            sendErr(errorP, "Allowed scale range is 0 - 1");
           }
         } else if (color(word) != null) {
            material.color = color(word);
         } else if (word.startsWith("mat(") && word.endsWith(")")) {
                    const slice = word.slice(4, word.length - 1);
                    let colors = new Array();
                    if (slice.includes(',')) {
                        const colorNames = slice.split(',');
                        for (let c in colorNames) {
                            color(colorNames[c]) != null ? colors.push(color(colorNames[c])) : sendErr(errorP, "Invalid colors for the effect");
                        }
                        const count = geometry.attributes.position.count;
                        geometry.setAttribute('color', new THREE.BufferAttribute(new Float32Array(3 * count), 3));
                        let colorAttrib = geometry.attributes.color;
                        for (let i = 0; i < count; i++) {
                            let index = randInt(0, colors.length - 1);
                            colorAttrib.setXYZ(i, colors[index].r, colors[index].g, colors[index].b);
                        }
                        material.vertexColors = true;
                    } else {
                        sendErr(errorP, "Just a color name would be enough - use at least two colors for this");
                    }
         }  else if (word.startsWith("tex(") && word.endsWith(")")) {
              let url = word.slice(5,word.length - 2);
                            if (isValidUrl(url)){
                              if (url.endsWith('.png') || url.endsWith('.jpg') || url.endsWith('.jpeg') || url.endsWith('.svg')){
                                const textureLoader = new THREE.TextureLoader();
                                const normalMap = textureLoader.load(url);
                                material.map = normalMap;
                              }else if (url.endsWith('.gif') || url.endsWith('.mp4') || url.endsWith('.webm') || url.endsWith('.mpg')){
                                let redundantVideos = document.querySelectorAll('video');
                                if (redundantVideos.length > 0){
                                  redundantVideos.forEach((v)=>{
                                    v.remove();
                                  });
                                }
                                let video = document.createElement('video');
                                video.src = url;
                                video.crossOrigin = "anonymous";
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
                                material.map = videoTexture;
                              }
                            } else {
                              sendErr(errorP, "Invalid URL");
                            }
         } else if (word.startsWith("rot") && word.includes('(') && word.endsWith(")")) {
                    if (word[3] == "X" || word[3] == "Y" || word[3] == "Z") {
                        if (floats(word).length == 0) {
                            sendErr(errorP, "Provide a speed to rotate");
                        } else {
                            modelObj.rotation.speed = floats(word)[0];
                            modelObj.rotation.axis = word[3];
                        }
                    } /*else if (word.index('(') == 3){
                      
                    } */          
         } else if (word.startsWith("[") && word.endsWith("]")){
           const axes = word.slice(1, -1);
           if (!axes.includes(",")){
            sendErr(errorP,"Not a recognized translation. Describe the amount of X and Y to translate");
           } else {
            const values = axes.split(',');
            if (values.length > 2){
              sendErr(errorP,"Too many axes - only X and Y");
            } else if (values.length == 2){
              geometry.translate(parseFloat(values[0]),parseFloat(values[1]),0);
            }
           }
         } else {
            sendErr(errorP, "Unknown parameter. Allowed: radius, color, rotation, texture...");
          } 
      });
    }
  }
  modelObj.mesh = new THREE.Mesh(geometry, material);
  return modelObj;
}

const screen = (c) => {
    if (c.length == 1){
      sendErr(errorP,"Missing parameters");
      return null;
    } else if (c.length > 2){
      sendErr(errorP,"Too many parameters");
      return null;
    } else if (c.length == 2){
      if (!isNaN(parseFloat(c[1]))){
            let scale = parseFloat(c[1]);
            if (scale <= 1.0 && scale >= 0.0){
              return new THREE.Color(scale,scale,scale);
            } else {
              sendErr("Allowed range 0 - 1");
            }
      } else if (color(c[1]) != null){
          return color(c[1]);
      } else if (c[1].startsWith("tex(") && c[1].endsWith(")")) {
            let url = c[1].slice(5,c[1].length - 2);
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
                video.crossOrigin = "anonymous";
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
          }
    }
}

export const interpret = (scene, meshInfosArray, input) => {
  errorP.innerHTML = "";
  scene.clear();
    const light = new THREE.DirectionalLight(new THREE.Color(1, 1, 1), 1.0);
    light.position.set(0, 0, 2);
    scene.add(light);

    if (input != ""){
      let lines = input.split('\n');
      for (let l in lines){
          lines[l].trim();
          if (lines[l].includes(';')){
          let commands = lines[l].split(';');
          if (input[input.length - 1] == ";" || lines[l][lines[l].length - 1] == ";"){
            commands.pop();
          }
          for (let c in commands){
            let words = commands[c].trim().split(' ');
            let subjectToRender = medium(words[0]);
            if (subjectToRender.medium == "shape"){
              let meshInfo = model(words)
              if (meshInfo != null || meshInfo != undefined){
                scene.add(meshInfo.mesh)
                meshInfosArray.push(meshInfo)
              }
            } else if (subjectToRender.medium == "background"){
              let screenColor = screen(words)
              if (screenColor != null || screenColor != undefined){
                scene.background = screenColor;
              }
            }
          }
        } else {
          let words = input.split(' ');
          let subjectToRender = medium(words[0]);
          if (subjectToRender.medium == "shape"){
              let meshInfo = model(words)
              if (meshInfo != null || meshInfo != undefined){
                scene.add(meshInfo.mesh)
                meshInfosArray.push(meshInfo)
              }
            } else if (subjectToRender.medium == "background"){
              let screenColor = screen(words)
              if (screenColor != null || screenColor != undefined){
                scene.background = screenColor;
              }
            }
        }
      }
    }
}