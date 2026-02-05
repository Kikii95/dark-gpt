<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';

  interface Prerequisites {
    os: { name: string; version: string; arch: string };
    docker: { installed: boolean; running: boolean; version?: string; download_url?: string };
    ollama: { installed: boolean; running: boolean; version?: string; download_url?: string };
    model_dolphin: boolean;
    installed_models: string[];
    https_configured: boolean;
  }

  interface HealthReport {
    docker: { name: string; status: string; message?: string };
    ollama: { name: string; status: string; message?: string };
    webui: { name: string; status: string; message?: string };
    caddy: { name: string; status: string; message?: string };
  }

  interface DownloadProgress {
    status: string;
    completed: number;
    total: number;
    percent: number;
  }

  interface InstallProgress {
    dependency: string;
    phase: string;
    downloaded: number;
    total: number;
    percent: number;
    message: string;
  }

  interface AvailableModel {
    name: string;
    size: string;
    ram_required: string;
    recommended: boolean;
  }

  let loading = $state(true);
  let loadingMessage = $state('Checking system...');
  let error = $state<string | null>(null);
  let prerequisites = $state<Prerequisites | null>(null);
  let health = $state<HealthReport | null>(null);
  let webuiUrl = $state('https://dark-gpt.local');
  let showSetup = $state(true);
  let servicesRunning = $state(false);

  // Model selection state
  let availableModels = $state<AvailableModel[]>([]);
  let selectedModel = $state('dolphin-llama3:8b');

  // Model download state
  let downloadingModel = $state(false);
  let downloadProgress = $state<DownloadProgress | null>(null);

  // Dependency install state
  let installingDependency = $state<string | null>(null);
  let installProgress = $state<InstallProgress | null>(null);

  let unlistenProgress: UnlistenFn | null = null;
  let unlistenInstall: UnlistenFn | null = null;

  onMount(async () => {
    try {
      // Set up event listener for model download progress
      unlistenProgress = await listen<DownloadProgress>('model-download-progress', (event) => {
        downloadProgress = event.payload;
      });

      // Set up event listener for dependency install progress
      unlistenInstall = await listen<InstallProgress>('install-progress', (event) => {
        installProgress = event.payload;
      });

      // Load available models
      availableModels = await invoke<AvailableModel[]>('get_available_models');

      // Check prerequisites
      prerequisites = await invoke<Prerequisites>('detect_prerequisites');

      // If all good, check services health
      if (prerequisites.docker.running && prerequisites.ollama.running) {
        loadingMessage = 'Checking services...';
        health = await invoke<HealthReport>('check_all_services');
        webuiUrl = await invoke<string>('get_webui_url');

        if (health.webui.status === 'Healthy') {
          showSetup = false;
          servicesRunning = true;
        }
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  onDestroy(() => {
    if (unlistenProgress) unlistenProgress();
    if (unlistenInstall) unlistenInstall();
  });

  async function refreshPrerequisites() {
    loading = true;
    loadingMessage = 'Refreshing...';
    error = null;
    try {
      prerequisites = await invoke<Prerequisites>('detect_prerequisites');
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function downloadModel(modelName?: string) {
    const model = modelName || selectedModel;

    if (!prerequisites?.ollama.running) {
      error = 'Ollama must be running to download models';
      return;
    }

    downloadingModel = true;
    downloadProgress = { status: `Starting ${model}...`, completed: 0, total: 0, percent: 0 };
    error = null;

    try {
      await invoke('pull_model', { modelName: model });
      // Refresh prerequisites to update model status
      prerequisites = await invoke<Prerequisites>('detect_prerequisites');
    } catch (e) {
      error = String(e);
    } finally {
      downloadingModel = false;
      downloadProgress = null;
    }
  }

  async function startServices() {
    loading = true;
    loadingMessage = 'Starting services...';
    error = null;
    try {
      await invoke('start_services');
      // Wait a bit for services to start
      loadingMessage = 'Waiting for services to be ready...';
      await new Promise(r => setTimeout(r, 5000));
      health = await invoke<HealthReport>('check_all_services');
      webuiUrl = await invoke<string>('get_webui_url');
      showSetup = false;
      servicesRunning = true;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function stopServices() {
    loading = true;
    loadingMessage = 'Stopping services...';
    try {
      await invoke('stop_services');
      servicesRunning = false;
      showSetup = true;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function openDownload(url: string) {
    window.open(url, '_blank');
  }

  async function installDependency(dep: 'ollama' | 'docker') {
    installingDependency = dep;
    installProgress = null;
    error = null;
    try {
      await invoke(dep === 'ollama' ? 'install_ollama' : 'install_docker');
      await refreshPrerequisites();
    } catch (e) {
      error = String(e);
    } finally {
      installingDependency = null;
      installProgress = null;
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }
</script>

<main class="min-h-screen bg-dark-900 text-white">
  {#if loading}
    <div class="flex items-center justify-center min-h-screen">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-accent-500 mx-auto mb-4"></div>
        <p class="text-gray-400">{loadingMessage}</p>
      </div>
    </div>
  {:else if error}
    <div class="flex items-center justify-center min-h-screen p-4">
      <div class="bg-red-900/20 border border-red-500 rounded-lg p-6 max-w-md w-full">
        <h2 class="text-xl font-bold text-red-400 mb-2">Error</h2>
        <p class="text-gray-300 mb-4">{error}</p>
        <div class="flex gap-2">
          <button
            class="flex-1 px-4 py-2 bg-dark-700 hover:bg-dark-600 rounded-lg"
            onclick={() => { error = null; }}
          >
            Dismiss
          </button>
          <button
            class="flex-1 px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg"
            onclick={() => window.location.reload()}
          >
            Retry
          </button>
        </div>
      </div>
    </div>
  {:else if showSetup}
    <!-- Setup Wizard -->
    <div class="flex items-center justify-center min-h-screen p-4">
      <div class="bg-dark-800 rounded-2xl shadow-xl p-8 max-w-lg w-full">
        <h1 class="text-3xl font-bold mb-2 text-center">🔐 Dark-GPT</h1>
        <p class="text-gray-400 text-center mb-8">Setup Wizard</p>

        {#if prerequisites}
          <div class="space-y-4 mb-8">
            <!-- OS Info -->
            <div class="flex items-center justify-between p-3 bg-dark-700 rounded-lg">
              <span>Operating System</span>
              <span class="text-green-400">
                ✓ {prerequisites.os.name} ({prerequisites.os.arch})
              </span>
            </div>

            <!-- Docker -->
            <div class="p-3 bg-dark-700 rounded-lg">
              <div class="flex items-center justify-between">
                <span>Docker</span>
                {#if prerequisites.docker.running}
                  <span class="text-green-400">✓ v{prerequisites.docker.version}</span>
                {:else if prerequisites.docker.installed}
                  <span class="text-yellow-400">⚠ Not running</span>
                {:else if installingDependency === 'docker'}
                  <span class="text-blue-400 text-sm">Installing...</span>
                {:else}
                  <button
                    class="px-3 py-1 bg-accent-600 hover:bg-accent-700 rounded text-sm disabled:opacity-50"
                    onclick={() => installDependency('docker')}
                    disabled={!!installingDependency || downloadingModel}
                  >
                    Download & Install
                  </button>
                {/if}
              </div>
              {#if installingDependency === 'docker' && installProgress}
                <div class="mt-2">
                  <div class="flex justify-between text-sm text-gray-400 mb-1">
                    <span class="truncate mr-2">{installProgress.message}</span>
                    {#if installProgress.phase === 'downloading' && installProgress.total > 0}
                      <span class="shrink-0">{installProgress.percent.toFixed(1)}%</span>
                    {/if}
                  </div>
                  <div class="w-full bg-dark-600 rounded-full h-2">
                    {#if installProgress.phase === 'downloading' && installProgress.total > 0}
                      <div
                        class="bg-accent-500 h-2 rounded-full transition-all duration-300"
                        style="width: {installProgress.percent}%"
                      ></div>
                    {:else}
                      <div class="bg-accent-500 h-2 rounded-full animate-pulse w-full"></div>
                    {/if}
                  </div>
                  {#if installProgress.phase === 'downloading' && installProgress.total > 0}
                    <div class="text-xs text-gray-500 mt-1">
                      {formatBytes(installProgress.downloaded)} / {formatBytes(installProgress.total)}
                    </div>
                  {/if}
                </div>
              {/if}
            </div>

            <!-- Ollama -->
            <div class="p-3 bg-dark-700 rounded-lg">
              <div class="flex items-center justify-between">
                <span>Ollama</span>
                {#if prerequisites.ollama.running}
                  <span class="text-green-400">✓ v{prerequisites.ollama.version}</span>
                {:else if prerequisites.ollama.installed}
                  <span class="text-yellow-400">⚠ Not running</span>
                {:else if installingDependency === 'ollama'}
                  <span class="text-blue-400 text-sm">Installing...</span>
                {:else}
                  <button
                    class="px-3 py-1 bg-accent-600 hover:bg-accent-700 rounded text-sm disabled:opacity-50"
                    onclick={() => installDependency('ollama')}
                    disabled={!!installingDependency || downloadingModel}
                  >
                    Download & Install
                  </button>
                {/if}
              </div>
              {#if installingDependency === 'ollama' && installProgress}
                <div class="mt-2">
                  <div class="flex justify-between text-sm text-gray-400 mb-1">
                    <span class="truncate mr-2">{installProgress.message}</span>
                    {#if installProgress.phase === 'downloading' && installProgress.total > 0}
                      <span class="shrink-0">{installProgress.percent.toFixed(1)}%</span>
                    {/if}
                  </div>
                  <div class="w-full bg-dark-600 rounded-full h-2">
                    {#if installProgress.phase === 'downloading' && installProgress.total > 0}
                      <div
                        class="bg-accent-500 h-2 rounded-full transition-all duration-300"
                        style="width: {installProgress.percent}%"
                      ></div>
                    {:else}
                      <div class="bg-accent-500 h-2 rounded-full animate-pulse w-full"></div>
                    {/if}
                  </div>
                  {#if installProgress.phase === 'downloading' && installProgress.total > 0}
                    <div class="text-xs text-gray-500 mt-1">
                      {formatBytes(installProgress.downloaded)} / {formatBytes(installProgress.total)}
                    </div>
                  {/if}
                </div>
              {/if}
            </div>

            <!-- Model Selection -->
            <div class="p-3 bg-dark-700 rounded-lg">
              <label for="model-select" class="block text-sm text-gray-400 mb-2">Select AI Model:</label>
              <select
                id="model-select"
                bind:value={selectedModel}
                class="w-full bg-dark-600 border border-dark-500 rounded p-2 mb-3 text-white"
                disabled={downloadingModel || !!installingDependency}
              >
                {#each availableModels as model}
                  <option value={model.name}>
                    {model.name} (~{model.size}) - {model.ram_required} RAM {model.recommended ? '⭐' : ''}
                  </option>
                {/each}
              </select>

              <div class="flex items-center justify-between">
                <span class="text-sm">{selectedModel}</span>
                {#if prerequisites.installed_models?.some(m => m.startsWith(selectedModel))}
                  <span class="text-green-400">✓ Installed</span>
                {:else if downloadingModel}
                  <span class="text-blue-400">Downloading...</span>
                {:else}
                  <button
                    class="px-3 py-1 bg-accent-600 hover:bg-accent-700 rounded text-sm disabled:opacity-50"
                    onclick={() => downloadModel()}
                    disabled={!prerequisites.ollama.running}
                  >
                    Download
                  </button>
                {/if}
              </div>

              {#if downloadingModel && downloadProgress}
                <div class="mt-3">
                  <div class="flex justify-between text-sm text-gray-400 mb-1">
                    <span>{downloadProgress.status}</span>
                    <span>{downloadProgress.percent.toFixed(1)}%</span>
                  </div>
                  <div class="w-full bg-dark-600 rounded-full h-2">
                    <div
                      class="bg-accent-500 h-2 rounded-full transition-all duration-300"
                      style="width: {downloadProgress.percent}%"
                    ></div>
                  </div>
                  <div class="text-xs text-gray-500 mt-1">
                    {formatBytes(downloadProgress.completed)} / {formatBytes(downloadProgress.total)}
                  </div>
                </div>
              {/if}
            </div>
          </div>

          <!-- Refresh button -->
          <button
            class="w-full py-2 mb-4 bg-dark-700 hover:bg-dark-600 rounded-lg text-gray-400 text-sm"
            onclick={refreshPrerequisites}
          >
            🔄 Refresh Status
          </button>

          {#if prerequisites.docker.running && prerequisites.ollama.running}
            <button
              class="w-full py-3 bg-accent-600 hover:bg-accent-700 rounded-lg font-semibold transition-colors disabled:opacity-50"
              onclick={startServices}
              disabled={downloadingModel || !!installingDependency}
            >
              🚀 Start Dark-GPT
            </button>
            {#if !prerequisites.model_dolphin}
              <p class="text-center text-yellow-400 text-xs mt-2">
                Note: Model will be downloaded automatically when needed
              </p>
            {/if}
          {:else}
            <p class="text-center text-yellow-400 text-sm">
              Please install and start Docker and Ollama to continue.
            </p>
          {/if}
        {/if}
      </div>
    </div>
  {:else}
    <!-- Main Interface -->
    <div class="flex flex-col h-screen">
      <!-- Header -->
      <header class="flex items-center justify-between px-4 py-2 bg-dark-800 border-b border-dark-600">
        <div class="flex items-center gap-3">
          <span class="text-xl font-bold">🔐 Dark-GPT</span>
          {#if servicesRunning}
            <span class="px-2 py-1 bg-green-600/20 text-green-400 text-xs rounded-full flex items-center gap-1">
              <span class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></span>
              Running
            </span>
          {/if}
        </div>
        <div class="flex items-center gap-2">
          <a
            href={webuiUrl}
            target="_blank"
            rel="noopener noreferrer"
            class="px-3 py-1 bg-dark-700 hover:bg-dark-600 rounded text-sm text-gray-300"
          >
            Open in Browser ↗
          </a>
          <button
            class="px-3 py-1 bg-red-600/20 hover:bg-red-600/40 text-red-400 rounded text-sm"
            onclick={stopServices}
          >
            Stop Services
          </button>
        </div>
      </header>

      <!-- WebUI iframe -->
      <iframe
        src={webuiUrl}
        class="flex-1 w-full border-0"
        title="Open-WebUI"
        allow="clipboard-write; microphone"
      ></iframe>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }
</style>
