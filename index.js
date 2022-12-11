import init, {start, draw, set} from './pkg/pulchra.js';
      let canvas = document.getElementById('canvas');
      let input = document.getElementById('input');
      let audio;
      let gl;
      window.onload = (e) =>{
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        init().then(()=>{start(canvas.getContext('webgl2'))});
      }
      window.onresize = (e) =>{
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      canvas.getContext('webgl2').viewport(0,0,canvas.width,canvas.height);
      }
      function record(){
        let canvas_stream = canvas.captureStream(60);
        let recorder = new MediaRecorder(canvas_stream);
      }
      input.addEventListener('keydown', (e)=>{
        if (e.metaKey && e.key == 'Enter'){
          console.clear();
          if (typeof audio === 'undefined'){
            audio = new AudioContext();
           } else {
            audio.close();
            audio = new AudioContext();
           }
          if (typeof gl === 'undefined'){
            gl = canvas.getContext('webgl2');
          }
          if (input.value || input.value != ''){
            let string = String(input.value);
            set(string,gl,audio);
          } else {
            document.getElementById('error').innerHTML = '';
            start(gl);
          }
          } 
          else if (e.ctrlKey && e.key == 'r'){
          //shows a dialog and starts recording
          }
          else if (e.ctrlKey && e.key == 's'){
          //stops and saves all recordings
          }
      });