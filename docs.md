# Pulchra documentation

## Screen
Background color to set, only with the help of grayscale or RGB.
```
screen 0.5
screen rgb(1,1,0)
```

## 3D shapes
Default shapes are cube and sphere.
Their radius or color can be set.

Examples:
```
cube
sphere 0.5
cube 0.5 rgb(0.1,0.4,0.5)
sphere rgb(1,0.2,0.2)
```
## Audio oscillators
Types available: sine, saw, square, triangle.
Default parameters are frequency and gain.

Examples:
```
sin 330
sqr 120*0.5
saw 10*0.9
tri 1000
```

## Effects
`mul` multiplies the current scene across the screen in rows and columns

This example multiplies to 2 rows and 3 columns:
```
mul 2 3
```
