# Pulchra Docs
![screenshot](screenshot.png)
# How to use
Shortcuts (executed only when the text input field is on focus):
- Ctrl + Enter: Render 
(Unable to write? Try clicking Tab.) 
# Language
Code is written into the text field input and individual functions are always separated by semicolon.
Let's go!
## Screen
Background color or texture.
### Functions

- screen **scale** - grayscale (range from 0 - 1)
- screen rgb(**red**,**green**,**blue**) - RGB (all numbers range from 0 - 1)
- screen **name of color** - color from English name of the color 
- screen noise(**name of color**,**name of color**) - procedural texture made with Simplex noise used as the background texture
- screen tex("*insert URL here*") - whatever image/video from the Internet as the background texture

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
- octahedron
- dodecahedron
- icosahedron

Possible parameters (optional, in whatever order):
- radius (floating-point number from 0 - 1)
- color: 
  - rgb(**red**,**green**,**blue**)  (all numbers range from 0 - 1)
  - English color name
  - mat(*at least two color names separated by comma*) - randomly puts a color from the array into the mesh, e.g. mat(yellow,black,magenta)
- coordinates
- rotation:
  - one axis: either rotX(**speed**),rotY("**speed**),rotZ(**speed**), e.g. rotX(1),rotY(-.2),rotZ(.3)
  - multiple axes: rot(**x**,**y**,**z**), e.g. rot(.4,1,0)
- tex("*insert URL here*") - whatever image or video from the Internet as a texture

Examples:
```
cube

sphere 0.5

cone 0.5 magenta

torus rgb(1,0.2,0.2) [0,1]

circle 0.3 magenta [-0.5,1] rotZ(1) 

torusKnot rot(.3,1,0)

sphere mat(red,green,yellow)

circle mat(green,purple,khaki,olive) 1 rotZ(1); sphere 0.7 [1.4,0] tex("https://upload.wikimedia.org/wikipedia/commons/3/39/%D0%9C%D0%B0%D1%8F%D0%BA_%D0%91%D0%B5%D0%BB%D1%8B%D0%B9.webm") rotY(1); sphere 0.7 [-1.4,0] rotY(-1) tex("https://upload.wikimedia.org/wikipedia/commons/3/39/%D0%9C%D0%B0%D1%8F%D0%BA_%D0%91%D0%B5%D0%BB%D1%8B%D0%B9.webm")

octahedron rotY(1) mat(yellow,orange,red,olive) .81; cube [2,0] .7 rotY(2) rgb(.7,.4,1); sphere [-2,0] .7 mat(purple,blue) rot(.5,1,0)

screen tex("https://upload.wikimedia.org/wikipedia/commons/7/7c/Beech_forest_male_karpaty_01.jpg"); dodecahedron rgb(.5,.8,.6) rotY(1) .79 [1,0]; sphere [-2,0] .7 mat(green,blue,brown) rotY(-1)
```
# Using texture sources from the Internet
- Perfect source of photos/videos is [Wikimedia Commons](https://commons.m.wikimedia.org/wiki/Main_Page)
- Copy link of the image or video you want to use (it should start with *https://upload.wikimedia.org*)
- That's it! You can use the URL in the `tex()` function.