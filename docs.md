# Pulchra documentation
# How to use
Shortcuts (executed only when the textarea is on focus):
- Ctrl + Enter: Render 
- Ctrl + H: Open this doc in a new tab
- Ctrl + R: Record a video of the canvas (in progress)
- Ctrl + P: Take a photo of the canvas (in progress)
# Language
Individual functions are always separated by semicolon.
Let's go!
## Screen
Background color or texture.
### Functions

screen **scale** - grayscale (0 - 1)
screen rgb(**red**,**green**,**blue**) - RGB (all numbers 0 - 1)
screen **name** - color from English color name
screen noise(**name**,**name**) - procedural texture made with Simplex noise

Examples:
```
screen 0.5
screen rgb(1,0.5,0)
screen yellow
screen noise(black,white)
```
## 3D shapes
Available shapes:
- circle
- cone
- cube
- cylinder
- sphere
- square
- torus
- torusKnot

Possible parameters (optional, in whatever order):
- radius (floating-point number from 0 - 1)
- color: 
  - rgb(**red**,**green**,**blue**)  (all numbers 0 - 1)
  - English color name
  - mat(*at least two color names separated by comma*) - randomly puts a color from the array into the mesh
  - noise(**first**,**second**) - procedural texture made with Simplex noise
- coordinates ([**x**,**y**] from -1 to 1)
- rotation (word parts: rot + X/Y/Z + (**speed** - can also be a negative number))

Examples:
```
cube
sphere 0.5
cone 0.5 magenta
torus rgb(1,0.2,0.2) [0,1]
circle 0.3 magenta [-0.5,1] rotZ(1) 
```
## Effects
`mul` divides the current scene in rows and columns
### Functions
*All parameters must not be floating-point numbers.*
mul **integer** - number of rows and columns is the same
mul **rows** **columns**

