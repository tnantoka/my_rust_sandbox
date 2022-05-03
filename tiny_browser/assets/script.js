import init, { render } from '../pkg/tiny_browser.js';

document.addEventListener('DOMContentLoaded', () => {
  document.querySelector('.js-html').value = `<div class="a">
  <div class="b">
    <div class="c">
      <div class="d">
        <div class="e">
          <div class="f">
            <div class="g">
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<div class="container">
  <div class="outer">
    <p class="inner"></p>
    <p class="inner" id="bye"></p>
  </div>
</div>`;
  document.querySelector('.js-css').value = `* { display: block; padding: 12px; }
.a { background: #ff0000; }
.b { background: #ffa500; }
.c { background: #ffff00; }
.d { background: #008000; }
.e { background: #0000ff; }
.f { background: #4b0082; }
.g { background: #800080; }

.container {
  width: 600px;
  padding: 10px;
  border-width: 1px;
  margin: auto;
}

.outer {
  background: #00ccff;
  border-color: #666666;
  border-width: 2px;
  margin: 50px;
  margin-top: 10px;
  padding: 30px;
}

.inner {
  border-color: #cc0000;
  border-width: 4px;
  height: 100px;
  margin-bottom: 20px;
  width: 500px;
}

.inner#bye {
  background: #ffff00;
}
`;

  document.querySelector('.js-form').addEventListener('submit', (e) => {
    e.preventDefault();
    output();
  });

  init().then(() => {
    output();
  });
});


const output = () => {
  const html = document.querySelector('.js-html').value;
  const css = document.querySelector('.js-css').value;
  const output = render(html, css);

  const canvas = document.querySelector('.js-canvas');
  const context = canvas.getContext('2d');

  const imageData = context.createImageData(canvas.width, canvas.height);
  const data = imageData.data;

  output.pixels.forEach(({ r, g, b, a }, i) => {
    [r, g, b, a].forEach((color, j) => {
      data[i * 4 + j] = color;
    });
  });
  context.putImageData(imageData, 0, 0);
};