<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  const GRID = 20;
  const CELL = 20;
  const CANVAS = GRID * CELL;

  type Direction = 'Up' | 'Down' | 'Left' | 'Right';

  let canvas: HTMLCanvasElement;

  let snake: Array<{ x: number; y: number }> = [];
  let direction: Direction = 'Right';
  let nextDirection: Direction = 'Right';
  let food = { x: 5, y: 5 };
  let intervalId: ReturnType<typeof setInterval>;

  function reset() {
    snake = [{ x: GRID / 2, y: GRID / 2 }];
    direction = 'Right';
    nextDirection = 'Right';
    food = randomEmptyCell();
    // nothing else to do here
  }

  function randomEmptyCell() {
    while (true) {
      const x = Math.floor(Math.random() * GRID);
      const y = Math.floor(Math.random() * GRID);
      if (!snake.some((p) => p.x === x && p.y === y)) {
        return { x, y };
      }
    }
  }

  function step() {

    direction = nextDirection;

    const head = { ...snake[0] };
    if (direction === 'Up') head.y -= 1;
    else if (direction === 'Down') head.y += 1;
    else if (direction === 'Left') head.x -= 1;
    else if (direction === 'Right') head.x += 1;

    if (
      head.x < 0 ||
      head.x >= GRID ||
      head.y < 0 ||
      head.y >= GRID ||
      snake.some((p) => p.x === head.x && p.y === head.y)
    ) {
      reset();
      draw();
      return;
    }

    snake.unshift(head);

    if (head.x === food.x && head.y === food.y) {
      food = randomEmptyCell();
    } else {
      snake.pop();
    }

    draw();
  }

  function draw() {
    const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
    ctx.fillStyle = '#111827';
    ctx.fillRect(0, 0, CANVAS, CANVAS);


    const t = Date.now() / 100;
    const r = Math.sin(t) * 127 + 128;
    const g = Math.cos(t) * 127 + 128;
    const b = 200;
    ctx.fillStyle = `rgb(${r.toFixed(0)},${g.toFixed(0)},${b})`;
    snake.forEach((p) => {
      ctx.fillRect(p.x * CELL, p.y * CELL, CELL, CELL);
    });

    ctx.fillStyle = '#ef4444';
    ctx.fillRect(food.x * CELL, food.y * CELL, CELL, CELL);
  }

  function handleKey(event: KeyboardEvent) {
    const key = event.key;
    let newDir: Direction | null = null;
    if (key === 'ArrowUp' || key === 'w' || key === 'W') newDir = 'Up';
    else if (key === 'ArrowDown' || key === 's' || key === 'S') newDir = 'Down';
    else if (key === 'ArrowLeft' || key === 'a' || key === 'A') newDir = 'Left';
    else if (key === 'ArrowRight' || key === 'd' || key === 'D') newDir = 'Right';
    // spacebar no longer used
    if (!newDir) return;
    const opposite: Record<Direction, Direction> = {
      Up: 'Down',
      Down: 'Up',
      Left: 'Right',
      Right: 'Left'
    };
    if (newDir !== opposite[direction]) {
      nextDirection = newDir;
    }
  }

  onMount(() => {
    reset();
    draw();
    intervalId = setInterval(step, 100);
    window.addEventListener('keydown', handleKey);

    let touchStart: { x: number; y: number } | null = null;
    function handleTouchStart(e: TouchEvent) {
      const t = e.touches.item(0);
      if (t) touchStart = { x: t.clientX, y: t.clientY };
    }
    function handleTouchMove(e: TouchEvent) {
      if (!touchStart) return;
      const t = e.touches.item(0);
      if (!t) return;
      const dx = t.clientX - touchStart.x;
      const dy = t.clientY - touchStart.y;
      const adx = Math.abs(dx);
      const ady = Math.abs(dy);
      if (adx < 30 && ady < 30) return;
      let dir: Direction;
      if (adx > ady) {
        dir = dx > 0 ? 'Right' : 'Left';
      } else {
        dir = dy > 0 ? 'Down' : 'Up';
      }
      const opposite: Record<Direction, Direction> = {
        Up: 'Down',
        Down: 'Up',
        Left: 'Right',
        Right: 'Left'
      };
      if (dir !== opposite[direction]) {
        nextDirection = dir;
      }
      touchStart = null;
    }
    window.addEventListener('touchstart', handleTouchStart);
    window.addEventListener('touchmove', handleTouchMove);

    onDestroy(() => {
      clearInterval(intervalId);
      window.removeEventListener('keydown', handleKey);
      window.removeEventListener('touchstart', handleTouchStart);
      window.removeEventListener('touchmove', handleTouchMove);
    });
  });
</script>

<div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
  <canvas bind:this={canvas} width={CANVAS} height={CANVAS} style="border:4px solid var(--green);border-radius:4px;background:#f3f4f6;box-shadow:0 0 8px rgba(0,0,0,0.4);"></canvas>
  <p style="font-size:0.875rem;max-width:16rem;text-align:center;opacity:0.7;">Use WASD or arrow keys. Collide with yourself or a wall to restart. Colors gently shift over time for some extra pizzazz.</p>
</div>
