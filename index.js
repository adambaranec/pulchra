      import init, {render} from './pkg/pulchra.js';
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
        init().then(()=>render());
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
        canvasRecDialog.close();
        let sessions = parseInt(sessionStorage.getItem('sessions'));
        sessions += 1;
        sessionStorage.setItem('sessions', sessions.toString());
        let canvasCaptureStream = canvas.captureStream(60);
        let recorder = new MediaRecorder(canvasCaptureStream, {mimeType: 'video/webm'});
        recorder.start();
        let file = [];
        recorder.ondataavailable = (e)=>{
          file.push(e.data);
        }
        input.addEventListener('keydown', (e) =>{
          if (e.ctrlkey && e.key == 'Enter'){
            render();
          }
          else if (e.ctrlKey && e.key == 's'){
           recorder.stop();
           recorder.onstop = (e)=>{
            let video = new Blob(file, {'type': 'video/webm'}); 
            let videoURL = URL.createObjectURL(video);  
            recSaveDialog.showModal();
            saveFile.onclick = (c)=>{
             let downloadElem = document.getElementById('file');
             downloadElem.href = videoURL;
             let name = fileNameInput.value;
               name = '.webm' ? downloadElem.download = `pulchra-${sessionStorage.getItem('sessions')}.webm` :
               downloadElem.download = `${name}.webm`; 
             downloadElem.onclick = (e) => {
               recSaveDialog.close();
               console.log(name);
             }
            }
           }
          }
        });
      }


      input.addEventListener('keydown', (e)=>{
       if (e.ctrlKey && e.key == 'Enter'){
       console.clear();
       } else if (e.ctrlKey && e.key == 'h'){
        window.open('https://github.com/adambaranec/pulchra/blob/main/docs.md', '_blank', 'noopener');
       } else if (e.ctrlKey && e.key == 'r'){
        canvasRecDialog.showModal();
       }
       else if (e.ctrlKey && e.key == 'p'){
         recSaveDialog.showModal();
         saveFile.onclick = (c)=>{
         canvas.toBlob((img)=>{
          let sessions = parseInt(sessionStorage.getItem('sessions'));
          sessions += 1;
          sessionStorage.setItem('sessions', sessions.toString());
            let downloadElem = document.getElementById('file');
            downloadElem.href = URL.createObjectURL(img);
            let name = fileNameInput.value;
            name = '.jpg' ? downloadElem.download = `pulchra-${sessionStorage.getItem('sessions')}.jpg` :
            downloadElem.download = `${name}.jpg`;
          }, 'image/jpg');
         }
         downloadElem.onclick = (e) => {
          recSaveDialog.close();
        }
       }
      });