use strict;
import { randFloat, randInt } from 'three/src/math/MathUtils';

const errorP = document.getElementById('error');
const sendErr = (errorParagraph, err) => {
    if (typeof err === 'string') {
        errorParagraph.innerHTML = err;
    }
}

const color = (word) => {
    if (typeof word === 'string') {
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
        } else {
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
                case "grey": return new THREE.Color('gray');
                case "gray": return new THREE.Color('gray');
                default: sendErr(errorP, "Invalid color name");  return null;
            }
        }
    } else {
        sendErr(errorP, "Invalid color");
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
        case "camera": return { medium: "camera" };
        case "light": return { medium: "light" };
        default: sendErr(errorP, "Unknown object to add to the scene. Choose: cube, screen..."); return null;
    }
}

const model = (command, scene) => {
    if (medium(command[0]).medium == "shape") {
        let modelObj = {};
        let geometry = undefined;
        let material = new THREE.MeshPhongMaterial();
        modelObj.transform = new THREE.Matrix4();
        modelObj.rotation = {};
        switch (medium(command[0]).variant) {
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
        if (command.length == 1) {
            geometry.scale(1.0, 1.0, 1.0);
            material.color = new THREE.Color(1.0, 1.0, 1.0);
        } else if (command.length > 1) {
            for (let i = 1; i < command.length; i++) {
                if (!isNaN(parseFloat(command[i]))) {
                    let value = parseFloat(command[i]);
                    if (value >= 0.0 && value <= 1.0) {
                        let matrix = modelObj.transform;
                        matrix.multiply(new THREE.Matrix4().makeScale(parseFloat(command[i]), parseFloat(command[i]), parseFloat(command[i])));
                        modelObj.transform = matrix;
                    } else {
                        sendErr(errorP, "Allowed scale range is 0 - 1");
                    }
                } else if (color(command[i]) != null) {
                    material.color = color(command[i]);
                } else if (command[i].startsWith("mat(") && command[i].endsWith(")")) {
                    const slice = command[i].slice(4, command[i].length - 1);
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
                } else if (command[i].startsWith("rot") && command[i].includes('(') && command[i].endsWith(")")) {
                    if (command[i][3] == "X" || command[i][3] == "Y" || command[i][3] == "Z") {
                        if (floats(command[i]).length == 0) {
                            sendErr(errorP, "Provide a speed to rotate");
                        } else {
                            modelObj.rotation.speed = floats(command[i])[0];
                            modelObj.rotation.axis = command[i][3];
                        }
                    }
                } 
            }
        }
    }
}

const interpret = (scene, input) => {
    scene.clear();
    const light = new THREE.DirectionalLight(new THREE.Color(1, 1, 1), 1.0);
    light.position.set(0, 0, 2);
    scene.add(light);

    const result = {};
    for (command in input.split(';')) {

    }
}