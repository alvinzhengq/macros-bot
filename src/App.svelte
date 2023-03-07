<script lang="ts">
  let recording = false;

  function recordKey() {
    recording = true;

    document.addEventListener(
      "keypress",
      (event) => {
        var name = event.key;
        var code = event.code;
        
        recording = false;
      },
      false
    );
  }
</script>

<main class="flex flex-col items-center align-middle p-6">
  <div class="bg-black/[0.04] w-[90vw] h-[70vh] rounded-lg" />

  <div class="mt-10 w-[50vw] flex flex-row justify-between">
    <button
      disabled={recording}
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
      on:click={recordKey}
      disabled={recording}
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
      disabled={recording}
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
