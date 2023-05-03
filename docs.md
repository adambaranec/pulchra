# Pulchra documentation

## Screen
Background color to set, only with the help of grayscale, RGB or color name.

Examples:
```
screen 0.5
screen rgb(1,0.5,0)
screen yellow
```

## 3D shapes
Default shapes are cube and sphere.
Parameters: radius, color, UV coordinates, rotation.

Examples:
```
cube
sphere 0.5
cube 0.5 pink
sphere rgb(1,0.2,0.2) [0,1]
sphere 0.3 magenta [-0.5,1] rotate(1) 
```

## Effects
`mul` divides the current scene in rows and columns

This example multiplies to 2 rows and 3 columns:
```
mul 2 3
```
