<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  let systemInfo = $state({
    system_name: "",
    kernel_version: "",
    host_name: "",
    os_version: "",
    cpu_arch: "",
  });

  onMount(() => {
    invoke("get_system_info").then((info) => {
      let payload = info as Record<string, unknown>;
      systemInfo = {
        system_name: payload.system_name as string,
        kernel_version: payload.kernel_version as string,
        host_name: payload.host_name as string,
        os_version: payload.os_version as string,
        cpu_arch: payload.cpu_arch as string,
      };
    });
  });
</script>

<div class="container">
  <h2>System Information</h2>
  <ul>
    <li><strong>System Name:</strong> {systemInfo.system_name}</li>
    <li><strong>Kernel Version:</strong> {systemInfo.kernel_version}</li>
    <li><strong>Host Name:</strong> {systemInfo.host_name}</li>
    <li><strong>OS Version:</strong> {systemInfo.os_version}</li>
    <li><strong>CPU Architecture:</strong> {systemInfo.cpu_arch}</li>
  </ul>
</div>
