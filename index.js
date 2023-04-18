      import {init, interpret, draw, play} from '/pkg/pulchra.js';
      let canvas = document.getElementById('canvas');
      let input = document.getElementById('input');
      let audio = undefined;
      let canvasRecDialog = document.getElementById('canvas-rec');
      let recSaveDialog = document.getElementById('record-save');
      let welcomeDialog = document.getElementById('welcome');
      let close = document.getElementById('close1');
      let close2 = document.getElementById('close2');
      let approveRecord = document.getElementById('approve');
      let saveFile = document.getElementById('save-file');
      let gotIt = document.getElementById('got-it');
      let fileNameInput = document.getElementById('file-name');

      let previousResults = Object.create();
      let currentResults = Object.create();
      let oscillators = new Array();
      let models = new Array();
  
      window.onload = (e) =>{
        welcomeDialog.showModal();
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        sessionStorage.setItem('sessions', '-1');
      }
      window.onresize = (e) =>{
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      canvas.getContext('webgl2').viewport(0,0,canvas.width,canvas.height);
      }
      close.onclick = (e) => {canvasRecDialog.close();}
      close2.onclick = (e) => {recSaveDialog.close();}
      gotIt.onclick = (e) => {welcomeDialog.close();}
      approveRecord.onclick = (e) => {
        let sessions = parseInt(sessionStorage.getItem('sessions'));
        sessions += 1;
        sessionStorage.setItem('sessions', sessions.toString());
        let recorder = new MediaRecorder(canvas.captureStream(60));
        let file = [];
        recorder.start();
        recorder.ondataavailable = (e) =>{
          file.push(e.data);
        }
        input.addEventListener('keydown', (e) =>{
          if (e.metaKey && e.key == 'S'){
           recorder.stop();
           let video = new Blob(file, { 'type' : 'video/mp4' });
           let videoURL = URL.createObjectURL(video);  
           recSaveDialog.showModal();
           saveFile.onclick = (click)=>{
            let downloadElem = document.getElementById('file');
            downloadElem.href = videoURL;
            let name = fileNameInput.value;
            name = '' ? downloadVideo.download = `pulchra-${sessionStorage.getItem('sessions')}.mp4` :
            downloadVideo.download = `${name}.mp4`;
           }
          }
        });
      }

      const start = () => {
       let results = interpret();
       let oscs = results.oscillators;
       let objects = results.models;
       if (Object.keys(currentResults).length() > 0){
        previousResults = Object.assign(previousResults, currentResults);
        currentResults.models = objects;
        currentResults.oscillators = oscs;
        if (oscs.length() != 0){
        //comparing oscillators from previous and current sessions
        if (currentResults.oscillators.length() > previousResults.oscillators.length()){

        } else if (currentResults.oscillators.length() < previousResults.oscillators.length()){

        }
        }
       } else {
        if (oscs.length() != 0){
          if (typeof audio === undefined){audio = new AudioContext();}
          for (osc in oscs){
  
          }
         }
         if (objects.length() != 0){
          draw(objects);
         }
         currentResults.models = objects;
         currentResults.oscillators = oscs;
       }
      };
     

      input.addEventListener('keydown', (e)=>{
       if (e.ctrlKey && e.key == 'Enter'){
       console.clear();
       start();
       } else if (e.ctrlKey && e.key == 'H'){
        window.open('https://github.com/adambaranec/pulchra/blob/main/docs.md', '_blank', 'noopener');
       } else if (e.ctrlKey && e.key == 'R'){
            canvasRecDialog.showModal();
       }
      });