import init, { tokenize } from '/pkg/hello_lindera_wasm.js';

document.addEventListener('DOMContentLoaded', () => {
  document.querySelector('.js-form').addEventListener('submit', (e) => {
    e.preventDefault();
    printTokens();
  });
});

init().then(() => {
  printTokens();
});

const printTokens = () => {
  const text = document.querySelector('.js-text').value;
  const tokens = tokenize(text);
  const output = tokens
    .map((token) => `${token.text}: ${token.detail.join(', ')}`)
    .join('\n');
  document.querySelector('.js-output').innerHTML = output;
}