const canvas = document.getElementById('board');

const wait = ms => new Promise(resolve => setTimeout(() => resolve(), ms));

// TODO: this causes some problems when the canvas size changes
const height = canvas.clientHeight;
const width = canvas.clientWidth;

// takes 2-d array of integers and edit the canvas
// TODO: rewrite this as this is inefficient and ugly
const draw = (b) => {
  // clear all objects
  {
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, width, height);
  }

  // draw vertial lines
  for (let i = 0; i < 7; i += 1) {
    const ctx = canvas.getContext('2d');
    ctx.lineWidth = 0.3;
    ctx.fillStyle = 'black';
    ctx.beginPath();
    ctx.moveTo(width * (i + 1) / 8, 0);
    ctx.lineTo(width * (i + 1) / 8, height);
    ctx.stroke();
  }

  // draw horizontal lines
  for (let i = 0; i < 7; i += 1) {
    const ctx = canvas.getContext('2d');
    ctx.lineWidth = 0.3;
    ctx.fillStyle = 'black';
    ctx.beginPath();
    ctx.moveTo(0, height * (i + 1) / 8);
    ctx.lineTo(width, height * (i + 1) / 8);
    ctx.stroke();
  }

  // draw stones
  for (let i = 0; i < 8; i += 1) {
    for (let j = 0; j < 8; j += 1) {
      if (b[i][j] === 1) {
        const ctx = canvas.getContext('2d');
        ctx.fillStyle = 'black';
        ctx.beginPath();
        ctx.arc(width * (i + 0.5) / 8, height * (j + 0.5) / 8, height / 20, Math.PI * 2, false);
        ctx.fill();
      } else if (b[i][j] === 2) {
        const ctx = canvas.getContext('2d');
        ctx.fillStyle = 'white';
        ctx.beginPath();
        ctx.arc(width * (i + 0.5) / 8, height * (j + 0.5) / 8, height / 20, Math.PI * 2, false);
        ctx.fill();
      }
    }
  }
};

import('reversi-wasm-core').then(async ({ Reversi }) => {
  document.getElementById('start').addEventListener('click', async () => {
    const strategy_cpu = [...document.getElementById('strategy_cpu').options].filter(i => i.selected)[0].value;
    const player_user = [...document.getElementById('player_user').options].filter(i => i.selected)[0].value;

    // TODO: allow users to restart in midst of the game
    document.getElementById('start').disabled = true;
    document.getElementById('player_user').disabled = true;
    document.getElementById('strategy_cpu').disabled = true;

    const reversi = Reversi.new(player_user, strategy_cpu);

    const draw_board = () => draw(JSON.parse(reversi.board()));

    draw_board();

    if (player_user === 'white') {
      reversi.wait_for_cpu();
      await wait(1000);
      draw_board();
    }

    canvas.addEventListener('click', async (e) => {
      const rect = e.target.getBoundingClientRect();
      const x = Math.floor((e.clientX - rect.left) * 8 / width);
      const y = Math.floor((e.clientY - rect.top) * 8 / height);

      const is_valid = reversi.make_move(x, y);
      if (!is_valid) return;

      draw_board();

      reversi.wait_for_cpu();

      await wait(1000);

      draw_board();

      while (!reversi.has_valid_move()) {
        // if the game has ended, print the result
        let result = reversi.result();
        if (result) {
          result = JSON.parse(result);
          alert(`result: ${result[0]} - ${result[1]}`);
          return;
        }

        // if the game has not yet ended, we have to pass
        reversi.pass();
        reversi.wait_for_cpu();
        await wait(1000);

        draw_board();
      }
    }, false);
  });
});
