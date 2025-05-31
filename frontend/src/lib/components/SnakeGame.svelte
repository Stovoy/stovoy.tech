<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { browser } from '$app/environment';

  const CANVAS_WIDTH = 800;
  const CANVAS_HEIGHT = 600;
  const INITIAL_SPEED = 2;
  const SEGMENT_SIZE = 12;
  const FOOD_SIZE = 8;

  type Vector2 = { x: number; y: number };
  type PowerUpType = 'speed' | 'grow' | 'magnet';
  
  interface SnakeSegment {
    x: number;
    y: number;
    targetX: number;
    targetY: number;
    size: number;
  }

  interface Food {
    x: number;
    y: number;
    type: 'normal' | 'golden';
    pulse: number;
  }

  interface PowerUp {
    x: number;
    y: number;
    type: PowerUpType;
    timer: number;
    collected: boolean;
  }

  interface Particle {
    x: number;
    y: number;
    vx: number;
    vy: number;
    life: number;
    maxLife: number;
    color: string;
    size: number;
  }

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  // Game state
  let snake: SnakeSegment[] = [];
  let directionAngle = 0; // radians, 0 = right
  const SNAKE_ROTATE_SPEED = 0.25; // radians per frame (fast)
  const MOVE_SPEED = INITIAL_SPEED;
  let foods: Food[] = [];
  let powerUps: PowerUp[] = [];
  let particles: Particle[] = [];
  
  // Game stats
  let score = 0;
  let lives = 3;
  let gameRunning = true;
  let gameStarted = false;
  
  // Effects and timers
  let speedBoostTimer = 0;
  let magnetTimer = 0;
  let invulnerableTimer = 0;
  let cameraShake = 0;
  
  // Animation
  let gameTime = 0;
  let lastTime = 0;
  
  // Sound simulation (visual feedback)
  let soundPulse = 0;
  
  // Controls
  let keys: Record<string, boolean> = {};
  let touchStart: Vector2 | null = null;
  let targetDirection: Vector2 = { x: 1, y: 0 };

  function initGame() {
    snake = [
      { x: CANVAS_WIDTH / 2, y: CANVAS_HEIGHT / 2, targetX: CANVAS_WIDTH / 2, targetY: CANVAS_HEIGHT / 2, size: SEGMENT_SIZE }
    ];
    directionAngle = 0;
    foods = [];
    powerUps = [];
    particles = [];
    score = 0;
    lives = 3;
    gameRunning = true;
    gameStarted = true;
    
    // Spawn initial food
    spawnFood();
    spawnFood(); // Start with 2 foods
  }

  function spawnFood() {
    const margin = 50;
    const food: Food = {
      x: margin + Math.random() * (CANVAS_WIDTH - margin * 2),
      y: margin + Math.random() * (CANVAS_HEIGHT - margin * 2),
      type: Math.random() < 0.8 ? 'normal' : 'golden',
      pulse: 0
    };
    foods.push(food);
  }

  function spawnPowerUp() {
    if (powerUps.length >= 2) return; // Max 2 powerups at once
    
    const types: PowerUpType[] = ['speed', 'grow', 'magnet'];
    const margin = 50;
    const powerUp: PowerUp = {
      x: margin + Math.random() * (CANVAS_WIDTH - margin * 2),
      y: margin + Math.random() * (CANVAS_HEIGHT - margin * 2),
      type: types[Math.floor(Math.random() * types.length)],
      timer: 0,
      collected: false
    };
    powerUps.push(powerUp);
  }

  function createParticles(x: number, y: number, count: number, color: string) {
    for (let i = 0; i < count; i++) {
      const angle = (Math.PI * 2 * i) / count + Math.random() * 0.5;
      const velocity = 2 + Math.random() * 4;
      particles.push({
        x,
        y,
        vx: Math.cos(angle) * velocity,
        vy: Math.sin(angle) * velocity,
        life: 0,
        maxLife: 30 + Math.random() * 30,
        color,
        size: 2 + Math.random() * 4
      });
    }
  }

  function updateSnake(deltaTime: number) {
    if (!gameRunning) return;

    // Compute target direction from keys
    let tx = 0, ty = 0;
    if (keys['KeyW'] || keys['ArrowUp']) ty -= 1;
    if (keys['KeyS'] || keys['ArrowDown']) ty += 1;
    if (keys['KeyA'] || keys['ArrowLeft']) tx -= 1;
    if (keys['KeyD'] || keys['ArrowRight']) tx += 1;
    if (tx !== 0 || ty !== 0) {
      const len = Math.hypot(tx, ty);
      targetDirection = { x: tx / len, y: ty / len };
    }

    // Rotate directionAngle toward targetDirection
    if (tx !== 0 || ty !== 0) {
      const targetAngle = Math.atan2(targetDirection.y, targetDirection.x);
      let diff = targetAngle - directionAngle;
      // Normalize diff to [-PI, PI]
      while (diff > Math.PI) diff -= 2 * Math.PI;
      while (diff < -Math.PI) diff += 2 * Math.PI;
      const maxStep = SNAKE_ROTATE_SPEED;
      if (Math.abs(diff) < maxStep) {
        directionAngle = targetAngle;
      } else {
        directionAngle += Math.sign(diff) * maxStep;
      }
    }

    // Move head at constant speed
    const currentSpeed = MOVE_SPEED + (speedBoostTimer > 0 ? 3 : 0);
    const head = snake[0];
    head.targetX += Math.cos(directionAngle) * currentSpeed;
    head.targetY += Math.sin(directionAngle) * currentSpeed;

    // Smooth interpolation
    const lerpFactor = 0.3;
    head.x += (head.targetX - head.x) * lerpFactor;
    head.y += (head.targetY - head.y) * lerpFactor;

    // Follow segments
    for (let i = 1; i < snake.length; i++) {
      const current = snake[i];
      const target = snake[i - 1];
      const distance = Math.sqrt((target.x - current.x) ** 2 + (target.y - current.y) ** 2);
      if (distance > SEGMENT_SIZE * 1.5) {
        const factor = 0.1;
        current.x += (target.x - current.x) * factor;
        current.y += (target.y - current.y) * factor;
      }
    }

    // Wall collision
    if (invulnerableTimer <= 0) {
      if (head.x < 0 || head.x > CANVAS_WIDTH || head.y < 0 || head.y > CANVAS_HEIGHT) {
        hitWall();
      }
      // Self collision
      for (let i = 3; i < snake.length; i++) {
        const segment = snake[i];
        const distance = Math.sqrt((head.x - segment.x) ** 2 + (head.y - segment.y) ** 2);
        if (distance < SEGMENT_SIZE) {
          hitSelf();
          break;
        }
      }
    }
    // Food collision
    foods.forEach((food, index) => {
      const distance = Math.sqrt((head.x - food.x) ** 2 + (head.y - food.y) ** 2);
      if (distance < SEGMENT_SIZE + FOOD_SIZE) {
        eatFood(food, index);
      }
    });
    // PowerUp collision
    powerUps.forEach((powerUp, index) => {
      if (powerUp.collected) return;
      const distance = Math.sqrt((head.x - powerUp.x) ** 2 + (head.y - powerUp.y) ** 2);
      if (distance < SEGMENT_SIZE + 15) {
        collectPowerUp(powerUp, index);
      }
    });
  }

  function eatFood(food: Food, index: number) {
    const points = food.type === 'golden' ? 50 : 10;
    score += points;
    
    // Add segments
    const segmentsToAdd = food.type === 'golden' ? 3 : 1;
    for (let i = 0; i < segmentsToAdd; i++) {
      const tail = snake[snake.length - 1];
      snake.push({
        x: tail.x,
        y: tail.y,
        targetX: tail.x,
        targetY: tail.y,
        size: SEGMENT_SIZE
      });
    }

    // Effects
    const color = food.type === 'golden' ? '#ffd700' : '#00ff00';
    createParticles(food.x, food.y, food.type === 'golden' ? 15 : 8, color);
    cameraShake = 5;
    soundPulse = 10;

    // Remove food and spawn new one
    foods.splice(index, 1);
    spawnFood();


    // Random powerup spawn
    if (Math.random() < 0.3) {
      spawnPowerUp();
    }
  }

  function collectPowerUp(powerUp: PowerUp, index: number) {
    powerUp.collected = true;
    
    switch (powerUp.type) {
      case 'speed':
        speedBoostTimer = 300;
        break;
      case 'grow':
        for (let i = 0; i < 5; i++) {
          const tail = snake[snake.length - 1];
          snake.push({
            x: tail.x,
            y: tail.y,
            targetX: tail.x,
            targetY: tail.y,
            size: SEGMENT_SIZE
          });
        }
        break;
      case 'magnet':
        magnetTimer = 300;
        break;
    }

    createParticles(powerUp.x, powerUp.y, 12, '#ff00ff');
    cameraShake = 8;
    soundPulse = 15;
    
    setTimeout(() => {
      powerUps.splice(index, 1);
    }, 100);
  }

  function hitWall() {
    lives--;
    if (lives <= 0) {
      gameOver();
    } else {
      respawn();
    }
  }

  function hitSelf() {
    lives--;
    if (lives <= 0) {
      gameOver();
    } else {
      respawn();
    }
  }

  function respawn() {
    // Reset position to center
    snake[0].x = CANVAS_WIDTH / 2;
    snake[0].y = CANVAS_HEIGHT / 2;
    snake[0].targetX = CANVAS_WIDTH / 2;
    snake[0].targetY = CANVAS_HEIGHT / 2;
    
    invulnerableTimer = 120; // 2 seconds of invulnerability
    cameraShake = 15;
    createParticles(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2, 20, '#ff0000');
  }



  function gameOver() {
    gameRunning = false;
    createParticles(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2, 50, '#ff0000');
    cameraShake = 30;
  }

  function update(currentTime: number) {
    if (!gameStarted) return;
    
    const deltaTime = currentTime - lastTime;
    lastTime = currentTime;
    gameTime += deltaTime;

    // Update timers
    if (speedBoostTimer > 0) speedBoostTimer--;
    if (magnetTimer > 0) magnetTimer--;
    if (invulnerableTimer > 0) invulnerableTimer--;
    if (cameraShake > 0) cameraShake *= 0.9;
    if (soundPulse > 0) soundPulse--;

    updateSnake(deltaTime);

    // Update particles
    particles = particles.filter(particle => {
      particle.x += particle.vx;
      particle.y += particle.vy;
      particle.vx *= 0.98;
      particle.vy *= 0.98;
      particle.life++;
      return particle.life < particle.maxLife;
    });

    // Update foods
    foods.forEach(food => {
      food.pulse += 0.1;
      if (magnetTimer > 0) {
        const head = snake[0];
        const dx = head.x - food.x;
        const dy = head.y - food.y;
        const dist = Math.hypot(dx, dy);
        if (dist > 0) {
          const pull = Math.min(5, dist) * 0.1;
          food.x += (dx / dist) * pull;
          food.y += (dy / dist) * pull;
        }
      }
    });

    // Update powerups
    powerUps.forEach(powerUp => {
      powerUp.timer += 0.1;
    });

    draw();
  }

  function draw() {
    if (!ctx) return;

    // Draw background grid
    drawGrid();

    // Camera shake
    const shakeX = (Math.random() - 0.5) * cameraShake;
    const shakeY = (Math.random() - 0.5) * cameraShake;
    ctx.save();
    ctx.translate(shakeX, shakeY);

    // Clear canvas with gradient background
    const gradient = ctx.createLinearGradient(0, 0, 0, CANVAS_HEIGHT);
    gradient.addColorStop(0, '#0f0f23');
    gradient.addColorStop(1, '#1a1a2e');
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

    // Draw particles
    particles.forEach(particle => {
      const alpha = 1 - (particle.life / particle.maxLife);
      ctx.save();
      ctx.globalAlpha = alpha;
      ctx.fillStyle = particle.color;
      ctx.beginPath();
      ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
      ctx.fill();
      ctx.restore();
    });

    // Draw foods
    foods.forEach(food => {
      const pulseSize = Math.sin(food.pulse) * 2;
      const size = FOOD_SIZE + pulseSize;
      
      ctx.save();
      ctx.shadowBlur = 10;
      ctx.shadowColor = food.type === 'golden' ? '#ffd700' : '#00ff00';
      ctx.fillStyle = food.type === 'golden' ? '#ffd700' : '#00ff00';
      ctx.beginPath();
      ctx.arc(food.x, food.y, size, 0, Math.PI * 2);
      ctx.fill();
      ctx.restore();
    });

    // Draw powerups
    powerUps.forEach(powerUp => {
      if (powerUp.collected) return;
      
      const colors = {
        speed: '#ff6b6b',
        grow: '#45b7d1',
        magnet: '#e056fd'
      };
      
      const bounce = Math.sin(powerUp.timer) * 3;
      
      ctx.save();
      ctx.shadowBlur = 15;
      ctx.shadowColor = colors[powerUp.type];
      ctx.fillStyle = colors[powerUp.type];
      ctx.beginPath();
      ctx.arc(powerUp.x, powerUp.y + bounce, 12, 0, Math.PI * 2);
      ctx.fill();
      
      // Draw symbol
      ctx.fillStyle = '#ffffff';
      ctx.font = '12px monospace';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      const symbols = { speed: '⚡', grow: '+', magnet: '☄' };
      ctx.fillText(symbols[powerUp.type], powerUp.x, powerUp.y + bounce);
      ctx.restore();
    });

    // Draw snake
    snake.forEach((segment, index) => {
      const isHead = index === 0;
      const t = gameTime * 0.01;
      
      let color;
      if (invulnerableTimer > 0 && Math.floor(gameTime * 0.1) % 2) {
        color = '#ffffff';
      } else {
        const baseHue = 120 + Math.sin(t + index * 0.5) * 30;
        color = `hsl(${baseHue}, 70%, ${isHead ? 80 : 60 - index * 2}%)`;
      }
      
      ctx.save();
      ctx.shadowBlur = isHead ? 15 : 8;
      ctx.shadowColor = color;
      ctx.fillStyle = color;
      ctx.beginPath();
      ctx.arc(segment.x, segment.y, isHead ? segment.size + 2 : segment.size, 0, Math.PI * 2);
      ctx.fill();
      
      // Draw eyes on head
      if (isHead) {
        ctx.fillStyle = '#ffffff';
        const eyeOffset = 4;
        ctx.beginPath();
        ctx.arc(segment.x - eyeOffset, segment.y - eyeOffset, 2, 0, Math.PI * 2);
        ctx.arc(segment.x + eyeOffset, segment.y - eyeOffset, 2, 0, Math.PI * 2);
        ctx.fill();
        
        ctx.fillStyle = '#000000';
        ctx.beginPath();
        ctx.arc(segment.x - eyeOffset, segment.y - eyeOffset, 1, 0, Math.PI * 2);
        ctx.arc(segment.x + eyeOffset, segment.y - eyeOffset, 1, 0, Math.PI * 2);
        ctx.fill();
      }
      ctx.restore();
    });

    ctx.restore();

    // UI
    drawUI();
  }

  function drawGrid() {
    if (!ctx) return;
    ctx.save();
    ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    // Subtle grid lines
    ctx.globalAlpha = 0.18;
    for (let x = 0; x <= CANVAS_WIDTH; x += 40) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, CANVAS_HEIGHT);
      ctx.strokeStyle = 'rgba(0,255,255,0.18)';
      ctx.lineWidth = 1;
      ctx.shadowBlur = 8;
      ctx.shadowColor = '#00f2fe';
      ctx.stroke();
    }
    for (let y = 0; y <= CANVAS_HEIGHT; y += 40) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(CANVAS_WIDTH, y);
      ctx.strokeStyle = 'rgba(0,255,255,0.18)';
      ctx.lineWidth = 1;
      ctx.shadowBlur = 8;
      ctx.shadowColor = '#00f2fe';
      ctx.stroke();
    }
    // Bolder center lines
    ctx.globalAlpha = 0.35;
    ctx.beginPath();
    ctx.moveTo(CANVAS_WIDTH/2, 0);
    ctx.lineTo(CANVAS_WIDTH/2, CANVAS_HEIGHT);
    ctx.strokeStyle = '#00f2fe';
    ctx.lineWidth = 2;
    ctx.shadowBlur = 16;
    ctx.shadowColor = '#00f2fe';
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(0, CANVAS_HEIGHT/2);
    ctx.lineTo(CANVAS_WIDTH, CANVAS_HEIGHT/2);
    ctx.strokeStyle = '#00f2fe';
    ctx.lineWidth = 2;
    ctx.shadowBlur = 16;
    ctx.shadowColor = '#00f2fe';
    ctx.stroke();
    ctx.restore();
  }

  function drawUI() {
    // Sound pulse indicator
    if (soundPulse > 0) {
      ctx.save();
      ctx.globalAlpha = soundPulse / 20;
      ctx.strokeStyle = '#ffffff';
      ctx.lineWidth = 3;
      ctx.beginPath();
      ctx.arc(50, 50, 20 + soundPulse * 2, 0, Math.PI * 2);
      ctx.stroke();
      ctx.restore();
    }

    // Score and stats
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 24px monospace';
    ctx.textAlign = 'left';
    ctx.fillText(`Score: ${score}`, 20, 30);
    ctx.fillText(`Lives: ${lives}`, 20, 60);

    // Speed indicator
    const speedText = speedBoostTimer > 0 ? 'SPEED BOOST!' : '';
    if (speedText) {
      ctx.fillStyle = speedBoostTimer > 0 ? '#ff6b6b' : '#4ecdc4';
      ctx.font = 'bold 16px monospace';
      ctx.textAlign = 'center';
      ctx.fillText(speedText, CANVAS_WIDTH / 2, 50);
    }



    // Game over screen
    if (!gameRunning) {
      ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
      ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
      
      ctx.fillStyle = '#ffffff';
      ctx.font = 'bold 48px monospace';
      ctx.textAlign = 'center';
      ctx.fillText('GAME OVER', CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 - 50);
      
      ctx.font = 'bold 24px monospace';
      ctx.fillText(`Final Score: ${score}`, CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2);
      
      
      ctx.font = '18px monospace';
      ctx.fillText('Press SPACE to restart', CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 + 80);
    }

    // Start screen
    if (!gameStarted) {
      ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
      ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
      
      ctx.fillStyle = '#ffffff';
      ctx.font = 'bold 64px monospace';
      ctx.textAlign = 'center';
      ctx.fillText('SUPER SNAKE', CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 - 100);
      
      ctx.font = '24px monospace';
      ctx.fillText('Use WASD or Arrow Keys', CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 - 20);
      ctx.fillText('Collect power-ups for special effects!', CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 + 10);
      ctx.fillText('Golden food = 3x growth + bonus points', CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 + 40);
      
      ctx.font = 'bold 20px monospace';
      ctx.fillText('Press SPACE to start!', CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 + 100);
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    keys[event.code] = true;

    // Start or restart game with space
    if (event.code === 'Space') {
      if (!gameStarted) {
        initGame();
      } else if (!gameRunning) {
        initGame();
      }
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    keys[event.code] = false;
  }

  function handleTouchStart(event: TouchEvent) {
    event.preventDefault();
    const touch = event.touches[0];
    touchStart = { x: touch.clientX, y: touch.clientY };
  }

  function handleTouchEnd(event: TouchEvent) {
    event.preventDefault();
    if (!touchStart || !gameRunning) return;
    const touch = event.changedTouches[0];
    const dx = touch.clientX - touchStart.x;
    const dy = touch.clientY - touchStart.y;
    const minSwipe = 30;
    if (Math.abs(dx) < minSwipe && Math.abs(dy) < minSwipe) {
      return;
    }
    // Set target direction based on swipe
    let tx = 0, ty = 0;
    if (Math.abs(dx) > Math.abs(dy)) {
      if (dx > 0) tx = 1;
      else tx = -1;
    } else {
      if (dy < 0) ty = -1;
      else ty = 1;
    }
    if (tx !== 0 || ty !== 0) {
      const len = Math.hypot(tx, ty);
      targetDirection = { x: tx / len, y: ty / len };
    }
    touchStart = null;
  }

  onMount(() => {
    if (!browser) return;
    
    ctx = canvas.getContext('2d')!;
    
    // Event listeners
    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);
    canvas.addEventListener('touchstart', handleTouchStart);
    canvas.addEventListener('touchend', handleTouchEnd);
    
    // Game loop
    function gameLoop(currentTime: number) {
      update(currentTime);
      requestAnimationFrame(gameLoop);
    }
    requestAnimationFrame(gameLoop);
  });

  onDestroy(() => {
    if (!browser) return;
    
    window.removeEventListener('keydown', handleKeyDown);
    window.removeEventListener('keyup', handleKeyUp);
    canvas?.removeEventListener('touchstart', handleTouchStart);
    canvas?.removeEventListener('touchend', handleTouchEnd);
  });
</script>

<div class="game-container">
  <canvas bind:this={canvas} width={CANVAS_WIDTH} height={CANVAS_HEIGHT} class="game-canvas"></canvas>
  {#if !gameStarted}
    <button class="play-btn-center" on:click={initGame}>Play!</button>
  {/if}
</div>
