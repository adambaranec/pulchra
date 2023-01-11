import init, {clear, set} from './pkg/pulchra.js';
      let canvas = document.getElementById('canvas');
      let input = document.getElementById('input');
      let audio;
      let gl;
      let dialog = document.getElementById('dialog');
      let dialog2 = document.getElementById('dialog2');
      let form = document.getElementById('form');
      let form2 = document.getElementById('form2');
      let close = document.getElementById('close-dialog');
      let close2 = document.getElementById('close-dialog-2');
      let fileName = document.getElementById('filename');
      window.onload = (e) =>{
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        init().then(()=>{start(canvas.getContext('webgl2'))});
        sessionStorage.setItem('sessions', '-1');
      }
      window.onresize = (e) =>{
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      canvas.getContext('webgl2').viewport(0,0,canvas.width,canvas.height);
      }
      close.onclick = () =>{
        dialog.close();
      }
      close2.onclick = () =>{
        dialog2.close();
      }
      form.addEventListener('submit', async (e)=>{
      e.preventDefault();
      let sessions = parseInt(sessionStorage.getItem('sessions')) += 1;
      sessionStorage.setItem('sessions', sessions.toString());
      let stream;
      let recorder;
      dialog.close();
      const data = new FormData(form);
      const audioData = [];
      const videoData = [];
      if (data['audio'] != ""){
      stream = await audio.createMediaStreamDestination().stream;
      recorder = await new MediaRecorder(stream);
      recorder.ondataavailable = (e)=>{
        audioData.push(e.data);
      };
      await recorder.start();
      }
      if (data['video'] != ""){
        stream = await canvas.captureStream(60);
        recorder = await new MediaRecorder(stream);
        recorder.ondataavailable = (e)=>{
          videoData.push(e.data);
        };
        await recorder.start();
      }
      input.addEventListener('keydown', (e)=>{
        if (e.ctrlKey && e.key == 's'){
          recorder.stop();
          recorder.onstop = (e)=>{
            if (audioData.length != 0){
              var audio = new Blob(audioData, {'type': 'audio/wav'});
              var audioURL = URL.createObjectURL(audio);
              var downloadAudio = document.createElement('a');
              downloadAudio.href = audioURL;
              dialog2.appendChild(downloadAudio);
              dialog2.showModal();
              downloadAudio.onclick = (e) =>{
                let name = fileName.value;
                name = '' ? downloadAudio.download = `pulchra-${sessionStorage.getItem('sessions')}.wav` :
                downloadAudio.download = `${name}.wav`;
              };
            }
            if (videoData.length != 0){
              var video = new Blob(videoData, { 'type' : 'video/mp4' });
              var videoURL = URL.createObjectURL(video);             
              var downloadVideo = document.createElement('a');
              downloadVideo.href = videoURL;
              dialog2.appendChild(downloadVideo);
              let name = fileName.value;
              downloadVideo.onclick = (e) =>{
                let name = fileName.value;
                name = '' ? downloadVideo.download = `pulchra-${sessionStorage.getItem('sessions')}.mp4` :
                downloadVideo.download = `${name}.mp4`;
              };
            }
          };
        }
      });
      });

      input.addEventListener('keydown', (e)=>{
        let string = String(input.value);
        if (e.metaKey && e.key == 'Enter'){
          console.clear();
          if (typeof audio === 'undefined'){
            audio = new AudioContext();
           }
          if (typeof gl === 'undefined'){
            gl = canvas.getContext('webgl2');
          }
          if (string != ''){
            set(string,audio);
          } else {
            document.getElementById('error').innerHTML = '';
            clear(gl);
          }
          } 
      });