<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import StatGraph from "../StatGraph.svelte";
  import { onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let performanceData: number[] = $state([]);
  let ramusage = $state(0.0);

  function openRamWindow() {
    invoke("open_ram_window");
  }

  onMount(() => {
    const setupListener = async () => {
      const unlisten = await listen("system-info-update", (event) => {
        const sysData = event.payload as Record<string, unknown>;
        const payload = {
          ram_capacity: sysData.ram_capacity as number,
          ram_used: sysData.ram_used as number,
        };
        const usedPercentage = (payload.ram_used / payload.ram_capacity) * 100;
        ramusage = usedPercentage;
        performanceData = [...performanceData, usedPercentage].slice(-25); // Keep last 25 data points
      });

      // Cleanup function when component is destroyed
      return unlisten;
    };

    const unsubscribe = setupListener();

    return () => {
      unsubscribe.then((f) => f());
    };
  });
</script>

<div style="padding: 1rem;">
  <StatGraph
    title="RAM Usage"
    chartData={performanceData}
    currValue={ramusage}
  />
  <button onclick={openRamWindow}>More Info</button>
</div>
