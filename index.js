      import init, {interpret} from './pkg/pulchra.js';
      let canvas = document.getElementById('canvas');
      let input = document.getElementById('input');
      let canvasRecDialog = document.getElementById('canvas-rec');
      let recSaveDialog = document.getElementById('record-save');
      let welcomeDialog = document.getElementById('welcome');
      let closeCanvasRecD = document.getElementById('close-canvas-rec');
      let closeRecSaveD = document.getElementById('close-record-save');
      let approveRecord = document.getElementById('approve');
      let saveFile = document.getElementById('save-file');
      let gotIt = document.getElementById('got-it');
      let fileNameInput = document.getElementById('file-name');
  
      window.onload = (e) =>{
        welcomeDialog.showModal();
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        sessionStorage.setItem('sessions', '-1');
        init();
      }
      window.onresize = (e) =>{
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      canvas.getContext('webgl2').viewport(0,0,canvas.width,canvas.height);
      }
      closeCanvasRecD.onclick = (e) => {canvasRecDialog.close();}
      closeRecSaveD.onclick = (e) => {recSaveDialog.close();}
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
           saveFile.onclick = (c)=>{
            let downloadElem = document.getElementById('file');
            downloadElem.href = videoURL;
            let name = fileNameInput.value;
            name = '' ? downloadElem.download = `pulchra-${sessionStorage.getItem('sessions')}.mp4` :
            downloadElem.download = `${name}.mp4`;
           }
          }
        });
      }

      input.addEventListener('keydown', (e)=>{
       if (e.ctrlKey && e.key == 'Enter'){
       console.clear();
       interpret();
       } else if (e.ctrlKey && e.key == 'H'){
        window.open('https://github.com/adambaranec/pulchra/blob/main/docs.md', '_blank', 'noopener');
       } else if (e.ctrlKey && e.key == 'R'){
            canvasRecDialog.showModal();
       }
       else if (e.ctrlKey && e.key == 'P'){
         let image = canvas.toDataURL('image/jpeg');
         recSaveDialog.showModal();
         saveFile.onclick = (c)=>{
          let downloadElem = document.getElementById('file');
          downloadElem.href = image;
          let name = fileNameInput.value;
          name = '' ? downloadElem.download = `pulchra-${sessionStorage.getItem('sessions')}.mp4` :
          downloadElem.download = `${name}.mp4`;
         }
       }
      });