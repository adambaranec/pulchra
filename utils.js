/*SOME BASIC MATHS */
const posFromPixels = (x, y, camera) => {
    let vector = new THREE.Vector3();
    vector.set(
        (x / width) * 2 - 1,
        - (y / height) * 2 + 1,
        0.5);
    vector.unproject(camera);
    let dir = vector.sub(camera.position).normalize();
    let distance = - camera.position.z / dir.z;
    let pos = camera.position.clone().add(dir.multiplyScalar(distance));
    return pos;
}
const pixelsFromPos = (pos, camera) => {
    let vector = new THREE.Vector3();
    vector.set(pos.x, pos.y, pos.z);
    vector.project(camera);
    vector.x = Math.round((vector.x + 1) * width / 2);
    vector.y = Math.round((-vector.y + 1) * height / 2);
    return vector;
}
const posFromUV = (u, v, camera) => {
    let vector = new THREE.Vector3();
    vector.set(
        (u) * 2 - 1,
        - (v) * 2 + 1,
        0.5);
    vector.unproject(camera);
    let dir = vector.sub(camera.position).normalize();
    let distance = - camera.position.z / dir.z;
    let pos = camera.position.clone().add(dir.multiplyScalar(distance));
    return pos;
}
const uvFromPixels = (x, y, camera) => {
    let vector = new THREE.Vector3();
    vector.set(
        (x / width) * 2 - 1,
        - (y / height) * 2 + 1,
        0.5);
    vector.unproject(camera);
    let dir = vector.sub(camera.position).normalize();
    let distance = - camera.position.z / dir.z;
    let pos = camera.position.clone().add(dir.multiplyScalar(distance));
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
