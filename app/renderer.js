const { exec } = require('child_process');
const path = require('path');
const { shell } = require('electron');

const selectMunchFile = document.querySelector('#select-munch-file');
const selectUnmunchFile = document.querySelector('#select-unmunch-file');
const inputFile = document.querySelector('#munch-input-file');
const munchFilename = document.querySelector('#munch-filename');
const viewFile = document.querySelector('#view-file');

let munchFolder = '';
let munchAction = '';

selectMunchFile.addEventListener('click', (e) => {
  if (munchAction) {
    return;
  }

  munchAction = 'munch';
  inputFile.click();
});

selectUnmunchFile.addEventListener('click', (e) => {
  if (munchAction) {
    return;
  }

  munchAction = 'unmunch';
  inputFile.click();
});

inputFile.addEventListener('change', () => {
  const filePath = inputFile.files[0].path;
  console.log(filePath);
  munchFolder = path.dirname(filePath);
  munchFilename.textContent = `${munchAction}ing ${path.basename(filePath)}...`;

  document.querySelectorAll('.action').forEach((el) => {
    el.textContent = munchAction;
  });
  changeState('loading');

  exec(
    `../target/release/constore ${
      munchAction === 'munch' ? '-e' : '-d'
    } "${filePath.replace(/"/g, '\\"')}"`,
    (err, stdout, stderr) => {
      munchAction = '';

      if (err) {
        console.log('unable to execute command', err);
        changeState('failure');
        return;
      }

      changeState('success');
      console.log(stdout);
      console.log(stderr);
    }
  );
});

viewFile.addEventListener('click', () => {
  shell.openItem(munchFolder);
});

function changeState(state) {
  document.querySelectorAll('.munch-state').forEach((el) => {
    el.classList.remove('munch-state-visible');
  });

  if (state) {
    document
      .querySelector(`.munch-state-${state}`)
      .classList.add('munch-state-visible');
  }
}
