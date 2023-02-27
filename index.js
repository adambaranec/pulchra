      let canvas = document.getElementById('canvas');
      let input = document.getElementById('input');
      let audio;
      let canvasRecDialog = document.getElementById('canvas-rec');
      let recSaveDialog = document.getElementById('record-save');
      let welcomeDialog = document.getElementById('welcome');
      let close = document.getElementById('close1');
      let close2 = document.getElementById('close2');
      let approveRecord = document.getElementById('approve');
      let saveFile = document.getElementById('save-file');
      let gotIt = document.getElementById('got-it');
      let fileNameInput = document.getElementById('file-name');
  
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
     

      input.addEventListener('keydown', (e)=>{
        if (e.metaKey && e.key == 'Enter'){
          console.clear();
          if (typeof audio === 'undefined'){
            audio = new AudioContext();
           }
          } else if (e.metaKey && e.key == 'H'){
            // directing to docs in pulchra repo
          } else if (e.metaKey && e.key == 'R'){
            canvasRecDialog.showModal();
          }
      });