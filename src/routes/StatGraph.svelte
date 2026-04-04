<script lang="ts">
  export let title;
  export let currValue: number = 0.0;
  export let chartData: number[] = [];

  let yLabels = [0, 25, 50, 75,100];

  // Internal dimensions for the coordinate system
  const viewWidth = 1000;
  const viewHeight = 200;
  const margin = 5;

  // This calculates the SVG path string every time performanceData changes
  $: pathData = chartData
    .map((val, i) => {
      const pointCount = chartData.length;

      // Calculate X: spread points evenly across the width
      const x = pointCount > 1 ? (i / (pointCount - 1)) * viewWidth : 0;

      // Calculate Y: Invert scale (SVG 0 is top) and clamp to viewHeight
      // Using (1 - val/100) ensures 100% is at the top
      const y = (1 - val / 100) * (viewHeight - margin * 2) + margin;

      return `${x},${y+29}`;
    })
    .join(" L "); // 'L' creates line segments in a path string
</script>

<div class="container">
  <h2>{title}({currValue.toFixed(2)}%)</h2>
  <div class="chart-wrapper">
    <svg
      viewBox={`0 0 ${viewWidth} ${viewHeight}`}
      preserveAspectRatio="none"
      class="chart-svg"
    >
      {#each yLabels as label}
        {@const yPos = (1.1 - label / 100) * (viewHeight - margin * 2) + margin}
        <!-- dash array lines -->
        <line
          x1="0"
          y1={yPos+10}
          x2={viewWidth}
          y2={yPos+10}
          stroke="#ffe"
          stroke-dasharray="5"
        />
        <!-- percentage labels -->
        <text
          x="2"
          y={yPos+10}
          fill="#fff"
          font-size="16"
          font-family="sans-serif"
        >
          {label}%
        </text>
      {/each}

      <path
        d="M {pathData}"
        fill="none"
        stroke="#00ff88"
        stroke-width="1"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .chart-wrapper {
    width: 100%;
    height: 250px;
    background: #111;
    border: 1px solid #333;
    border-radius: 12px;
    padding: 8px;
    overflow: hidden; /* Ensures no spillover */
  }

  .chart-svg {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>
