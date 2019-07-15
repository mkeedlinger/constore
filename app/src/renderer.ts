const { exec } = require('child_process');
const path = require('path');
const { shell } = require('electron');

const selectMunchFile = document.querySelector('#select-munch-file');
const selectUnmunchFile = document.querySelector('#select-unmunch-file');
const inputFile: HTMLInputElement = document.querySelector('#munch-input-file');
const munchFilename = document.querySelector('#munch-filename');
const viewFile = document.querySelector('#view-file');

let munchFolder = '';
let preMunchAction = '';
let munchAction = '';

selectMunchFile.addEventListener('click', (e) => {
  if (munchAction) {
    return;
  }

  preMunchAction = 'munch';
  inputFile.click();
});

selectUnmunchFile.addEventListener('click', (e) => {
  if (munchAction) {
    return;
  }

  preMunchAction = 'unmunch';
  inputFile.click();
});

inputFile.addEventListener('change', () => {
  const filePath = inputFile.files[0].path;
  console.log(filePath);
  munchFolder = path.dirname(filePath);
  munchFilename.textContent = path.basename(filePath);
  inputFile.value = '';
  munchAction = preMunchAction;

  document.querySelectorAll('.action').forEach((el) => {
    el.textContent = munchAction;
  });
  changeState('loading');

  exec(
    `../target/release/constore ${
      munchAction === 'munch' ? '-e' : '-d'
    } "${filePath.replace(/"/g, '\\"')}"`,
    (err: Error, stdout: string, stderr: string) => {
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

function changeState(state: string) {
  document.querySelectorAll('.munch-state').forEach((el) => {
    el.classList.remove('munch-state-visible');
  });

  if (state) {
    document
      .querySelector(`.munch-state-${state}`)
      .classList.add('munch-state-visible');
  }
}
