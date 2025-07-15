<script lang="ts">
  // 导入需要的组件和函数
  import { invoke } from "@tauri-apps/api/core";
  import Welcome from "./welcome.svelte"
  import BasicSettings from "./basicSettings.svelte"; // 导入Hello组件

  // 视图状态管理
  let currentView = $state(0); // 当前显示的视图索引
  const totalViews = 2; // 总视图数量（图片视图 + Hello组件视图）

  // 视图切换函数
  function navigate(direction: number) {
    // 根据方向更新当前视图索引，确保在有效范围内
    currentView = Math.max(0, Math.min(totalViews - 1, currentView + direction));
  }
</script>

<main class="container">
  <h1>设置向导</h1>

  <!-- 轮播容器 -->
  <div class="carousel">
    <!-- 左侧箭头按钮（仅在非首页时显示） -->
    {#if currentView > 0}
      <button class="arrow left" on:click={() => navigate(-1)} aria-label="返回上一页">
        ←
      </button>
    {/if}

    <!-- 内容区域 -->
    <div class="content">
      {#if currentView === 0}
          <Welcome />
      {:else if currentView === 1}
          <BasicSettings />
      {/if}
    </div>

    <!-- 右侧箭头按钮（在非最后一页时显示） -->
    {#if currentView < totalViews - 1}
      <button class="arrow right" on:click={() => navigate(1)} aria-label="前往下一页">
        →
      </button>
    {/if}
  </div>

  <!-- 导航指示点 -->
  <div class="dots">
    {#each Array(totalViews) as _, index}
      <button
        class="dot {index === currentView ? 'active' : ''}"
        on:click={() => currentView = index}
        aria-label={`切换到视图 ${index + 1}`}
      />
    {/each}
  </div>
</main>

<style>
  /* 保留原有样式 */
  .logo.ai-learning-assistant-launcher:hover {
    filter: drop-shadow(0 0 2em #747bee);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #000;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  /* 新增轮播组件样式 */
  .carousel {
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    width: 100%;
    margin: 2rem 0;
  }

  .content {
    width: 80%; /* 内容区域宽度 */
    min-height: 200px; /* 最小高度确保布局稳定 */
    display: flex;
    justify-content: center;
    align-items: center;
  }

  /* 箭头按钮样式 */
  .arrow {
    width: 50px;
    height: 50px;
    border-radius: 50%; /* 圆形按钮 */
    background-color: #646cff;
    color: white;
    border: none;
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
  }

  .arrow:hover {
    background-color: #535bf2;
    transform: scale(1.1);
  }

  .arrow:active {
    transform: scale(0.95);
  }

  /* 指示点样式 */
  .dots {
    display: flex;
    justify-content: center;
    margin-top: 1rem;
    gap: 0.5rem;
  }

  .dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background-color: #ccc;
    border: none;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .dot.active {
    background-color: #646cff;
  }

  .dot:hover {
    background-color: #535bf2;
  }

  /* 保留原有媒体查询 */
  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
