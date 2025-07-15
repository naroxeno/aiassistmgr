<script lang="ts">
  // 导入需要的组件

  // 目前处于早期开发阶段，因此始终为true
  const isFirstRun = true;
</script>

<main class="container">
  <!-- 内容区域，根据是否首次运行显示不同组件 -->
  <div class="content">
    {#if isFirstRun}

    {:else}

    {/if}
  </div>
</main>

<style>

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


  /* 保留原有媒体查询 */
  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
