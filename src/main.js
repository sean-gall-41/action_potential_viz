const { invoke } = window.__TAURI__.tauri;

const plotArea = document.getElementById('plotting-area');
const plotAreaWrapper = document.getElementById('canvas');
const ctx = plotArea.getContext('2d');

const N = 2000;
const randElem = Array.from({length: N}, (_, id) => [id, Math.random()]);

plotArea.width = plotAreaWrapper.getBoundingClientRect().width;
ctx.strokeStyle = 'blue';
ctx.lineWidth = 0.5;
ctx.beginPath();

randElem.forEach(([id, elem]) => {
  let x = (id / N) * plotArea.width;
  let y = elem * plotArea.height;
  ctx.lineTo(x, y);
});

ctx.stroke();

//let greetInputEl;
//let greetMsgEl;

//async function greet() {
//  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
//}

window.addEventListener("DOMContentLoaded", () => {
  //greetInputEl = document.querySelector("#greet-input");
  //greetMsgEl = document.querySelector("#greet-msg");
  //document
  //  .querySelector("#greet-button")
  //  .addEventListener("click", () => greet());
});
