import App from "./App.js";

const data = [
  {
    text: 'JS 공부하기',
    isCompleted: true
  },
  {
    text: 'JS 복습하기',
    isCompleted: true
  }
];

const $target = document.querySelector('#App');
new App($target, data);