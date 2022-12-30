import init, {start, set} from './pkg/pulchra.js';
      let canvas = document.getElementById('canvas');
      let input = document.getElementById('input');
      let audio;
      let gl;
      let dialog = document.querySelector('dialog');
      let form = document.getElementById('form');
      let close = document.getElementById('close-dialog');
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
      close.onclick = () =>{
        dialog.close();
      }
      form.addEventListener('submit', (e)=>{
      e.preventDefault();
      dialog.close();
      const data = new FormData(form);
      const audioData = [];
      const videoData = [];
      if (data['audio'] != ""){
      let stream = audio.createMediaStreamDestination().stream;
      let recorder = new MediaRecorder(stream);
      recorder.ondataavailable = (e)=>{
        audioData.push(e.data);
      };
      recorder.start();
      }
      if (data['video'] != ""){
        let stream = canvas.captureStream(60);
        let recorder = new MediaRecorder(stream);
        recorder.ondataavailable = (e)=>{
          videoData.push(e.data);
        };
        recorder.start();
      }
      window.addEventListener('keydown', (e)=>{
        if (e.ctrlKey && e.key == 's'){
          recorder.stop();
          recorder.onstop = (e)=>{
            if (audioData.length != 0){
              var audio = new Blob(audioData, {'type': 'audio/wav'});
              var audioURL = URL.createObjectURL(audio);
            }
            if (videoData.length != 0){
              var video = new Blob(videoData, { 'type' : 'video/mp4' });
              var videoURL = URL.createObjectURL(video);
            }
          };
        }
      });
      });
      let previousCode = new String();
      let currentCode = new String();
      let oldNodesArray = [];
      let newNodesArray = [];
      input.addEventListener('keydown', (e)=>{
        e.preventDefault();
        previousCode = currentCode;
        let string = String(input.value);
        currentCode = string;
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
            set(string,audio);            
          } else {
            document.getElementById('error').innerHTML = '';
            start(gl);
          }
          } 
      });