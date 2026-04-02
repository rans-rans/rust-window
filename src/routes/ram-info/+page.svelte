<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  interface RamData {
    ram_capacity: number;
    ram_used: number;
  }

  let ramData: RamData | null = $state(null);
  let id = "0.0";

  onMount(() => {
    const setupListener = async () => {
      const unlisten = await listen("system-info-update", (event) => {
        const sysData = event.payload as Record<string, unknown>;
        ramData = {
          ram_capacity: sysData.ram_capacity as number,
          ram_used: sysData.ram_used as number,
        };
      });

      return unlisten;
    };

    const unsubscribe = setupListener();

    return () => {
      unsubscribe.then((f) => f());
    };
  });

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }
</script>

<div class="container">
  <h1>RAM Information</h1>

  {#if ramData}
    <div class="info-section">
      <div class="info-item">
        <label for={id}>Total Capacity:</label>
        <value>{formatBytes(ramData.ram_capacity)}</value>
      </div>

      <div class="info-item">
        <label for={id}>Used:</label>
        <value>{formatBytes(ramData.ram_used)}</value>
      </div>

      <div class="info-item">
        <label for={id}>Available:</label>
        <value>{formatBytes(ramData.ram_capacity - ramData.ram_used)}</value>
      </div>

      <div class="info-item">
        <label for={id}>Usage Percentage:</label>
        <value
          >{((ramData.ram_used / ramData.ram_capacity) * 100).toFixed(
            2,
          )}%</value
        >
      </div>
    </div>

    <div class="progress-bar">
      <div
        class="progress-fill"
        style="width: {(ramData.ram_used / ramData.ram_capacity) * 100}%"
      ></div>
    </div>
  {:else}
    <p>Loading RAM information...</p>
  {/if}
</div>

<style>
  .container {
    padding: 20px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
  }

  h1 {
    margin-top: 0;
    color: #333;
  }

  .info-section {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
    margin: 20px 0;
  }

  .info-item {
    background: #f5f5f5;
    padding: 15px;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  label {
    font-weight: 600;
    color: #666;
    font-size: 0.9rem;
  }

  value {
    font-size: 1.3rem;
    font-weight: 500;
    color: #333;
  }

  .progress-bar {
    width: 100%;
    height: 30px;
    background: #e0e0e0;
    border-radius: 15px;
    overflow: hidden;
    margin-top: 20px;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #4caf50, #8bc34a);
    transition: width 0.3s ease;
  }
</style>
