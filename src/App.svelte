<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    writeTextFile,
    readTextFile,
    BaseDirectory,
    createDir,
    exists,
  } from "@tauri-apps/api/fs";

  let disabled = false;
  let active = false;
  let recording = false;

  let key_list = [];
  let key_list_active = [];

  document.addEventListener(
    "keypress",
    (event) => {
      if (!recording) return;

      invoke("create_event", {
        key: event.key.charAt(0),
        interval: 1000,
      });

      key_list = [
        ...key_list,
        {
          key: event.key.charAt(0),
          interval: 1000,
        },
      ];

      key_list_active = [...key_list_active, false];

      invoke("toggle_bot", { toggle: 1 });
      setTimeout(() => {
        invoke("toggle_bot", { toggle: 0 });
      }, 100);

      disabled = false;
      recording = false;
    },
    false
  );

  function recordKey() {
    disabled = true;
    recording = true;
  }

  function deleteKey(i) {
    invoke("delete_event", {
      key: key_list[i].key,
      interval: key_list[i].interval,
    });

    key_list = key_list.filter((m, index) => index !== i);
  }

  function toggleBot() {
    active = !active;
    disabled = active;

    invoke("toggle_bot", { toggle: active ? 1 : 0 });
  }

  function toggleKey(i) {
    key_list_active[i] = !key_list_active[i];
    console.log(key_list_active);
  }

  async function loadConfig() {
    const contents = await readTextFile("config.conf", {
      dir: BaseDirectory.AppConfig,
    });

    invoke("clear_events");

    let temp = JSON.parse(contents);
    for (let i = 0; i < temp.length; i++) {
      key_list = [...key_list, temp[i]];
      key_list_active = [...key_list_active, false];

      invoke("create_event", {
        key: temp[i].key,
        interval: parseInt(temp[i].interval),
      });
    }

    invoke("toggle_bot", { toggle: 1 });
    setTimeout(() => {
      invoke("toggle_bot", { toggle: 0 });
    }, 100);
  }

  async function exportConfig() {
    await createDir("config", {
      dir: BaseDirectory.AppConfig,
      recursive: true,
    });
    await writeTextFile("config.conf", JSON.stringify(key_list), {
      dir: BaseDirectory.AppConfig,
    });
  }
</script>

<main class="flex flex-col items-center align-middle p-6">
  <div
    id="main"
    class="bg-black/[0.04] w-[90vw] h-[70vh] rounded-lg flex flex-row flex-wrap justify-around items-center p-12"
    class:selected={active}
  >
    {#each key_list as { key, interval }, i}
      <div
        class="w-20 h-20 border-2 rounded-lg border-gray-400 bg-gray-400 text-center text-white relative"
      >
        <h1 class="text-3xl font-bold">{key}</h1>
        <h1
          class="text-xl font-bold cursor-pointer"
          on:click={() => {
            toggleKey(i);
          }}
        >
          {interval / 1000}s
        </h1>
        <button
          on:click={() => {
            deleteKey(i);
          }}
          class="absolute z-100 inline-flex items-center justify-center w-6 h-6 text-xs font-bold text-white bg-gray-500 rounded-full -top-2 -right-2"
          >X</button
        >
      </div>

      <div
        id="modal"
        class="hidden fixed top-0 left-0 right-0 z-50 w-full p-2 overflow-x-hidden overflow-y-auto md:inset-0 h-[calc(100%-1rem)] md:h-full"
        class:selected={key_list_active[i]}
      >
        <div
          class="w-full h-full max-w-2xl md:h-auto absolute top-1/3 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
        >
          <!-- Modal content -->
          <div
            class="bg-gray-300 rounded-lg p-6 flex flex-col justify-center items-center"
          >
            <input
              type="text"
              id="first_name"
              class="text-black text-sm rounded-lgblock w-full p-2.5 rounded-xl "
              placeholder="1000"
              bind:value={key_list[i].interval}
              required
            />
            <button
              on:click={() => {
                invoke("create_event", {
                  key: key_list[i].key,
                  interval: parseInt(key_list[i].interval),
                });

                toggleKey(i);

                invoke("toggle_bot", { toggle: 1 });
                setTimeout(() => {
                  invoke("toggle_bot", { toggle: 0 });
                }, 100);
              }}
              class="bg-blue-400 font-medium rounded-lg text-sm px-8 py-2.5 text-center inline-flex items-center mt-4 text-white"
              >Submit</button
            >
          </div>
        </div>
      </div>
    {/each}
  </div>

  <div class="mt-10 w-[50vw] flex flex-row justify-between">
    <button
      on:click={loadConfig}
      {disabled}
      type="button"
      class="duration-200 disabled:bg-gray-300 disabled:border-gray-300 disabled:text-white disabled:hover: hover:text-white hover:bg-gray-800 text-gray-800 border-[2px] border-gray-800 font-medium rounded-lg text-sm px-8 py-2.5 text-center inline-flex items-center mr-2"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="28"
        height="28"
        fill="currentColor"
        class="bi bi-folder2-open"
        viewBox="0 0 16 16"
      >
        <path
          d="M1 3.5A1.5 1.5 0 0 1 2.5 2h2.764c.958 0 1.76.56 2.311 1.184C7.985 3.648 8.48 4 9 4h4.5A1.5 1.5 0 0 1 15 5.5v.64c.57.265.94.876.856 1.546l-.64 5.124A2.5 2.5 0 0 1 12.733 15H3.266a2.5 2.5 0 0 1-2.481-2.19l-.64-5.124A1.5 1.5 0 0 1 1 6.14V3.5zM2 6h12v-.5a.5.5 0 0 0-.5-.5H9c-.964 0-1.71-.629-2.174-1.154C6.374 3.334 5.82 3 5.264 3H2.5a.5.5 0 0 0-.5.5V6zm-.367 1a.5.5 0 0 0-.496.562l.64 5.124A1.5 1.5 0 0 0 3.266 14h9.468a1.5 1.5 0 0 0 1.489-1.314l.64-5.124A.5.5 0 0 0 14.367 7H1.633z"
        />
      </svg>
    </button>

    <button
      on:click={toggleBot}
      disabled={!active && disabled}
      type="button"
      class="duration-200 disabled:bg-gray-300 disabled:border-gray-300 disabled:text-white disabled:hover: hover:text-white hover:bg-green-300 text-green-300 border-[2px] border-green-300 font-medium rounded-lg text-sm px-8 py-2.5 text-center inline-flex items-center mr-2"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="32"
        height="32"
        fill="currentColor"
        class="bi bi-play-fill"
        viewBox="0 0 16 16"
      >
        <path
          d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393z"
        />
      </svg>
    </button>

    <button
      on:click={recordKey}
      {disabled}
      type="button"
      class="duration-200 disabled:bg-gray-300 disabled:border-gray-300 disabled:text-white disabled:hover:  hover:text-white hover:bg-blue-400 text-blue-400 border-[2px] border-blue-400 font-medium rounded-lg text-sm px-8 py-2.5 text-center inline-flex items-center mr-2"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="28"
        height="28"
        fill="currentColor"
        class="bi bi-plus-circle"
        viewBox="0 0 16 16"
      >
        <path
          d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"
        />
        <path
          d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"
        />
      </svg>
    </button>

    <button
      on:click={exportConfig}
      {disabled}
      type="button"
      class="duration-200 disabled:bg-gray-300 disabled:border-gray-300 disabled:text-white disabled:hover:  hover:text-white hover:bg-gray-800 text-gray-800 border-[2px] border-gray-800 font-medium rounded-lg text-sm px-8 py-2.5 text-center inline-flex items-center mr-2"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="28"
        height="28"
        fill="currentColor"
        class="bi bi-save"
        viewBox="0 0 16 16"
      >
        <path
          d="M2 1a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H9.5a1 1 0 0 0-1 1v7.293l2.646-2.647a.5.5 0 0 1 .708.708l-3.5 3.5a.5.5 0 0 1-.708 0l-3.5-3.5a.5.5 0 1 1 .708-.708L7.5 9.293V2a2 2 0 0 1 2-2H14a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2h2.5a.5.5 0 0 1 0 1H2z"
        />
      </svg>
    </button>
  </div>
</main>

<style>
  #main.selected {
    pointer-events: none;
  }

  #modal.selected {
    display: block !important;
  }
</style>
