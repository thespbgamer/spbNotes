<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";

  async function make_new_note() {
      greetMsg = await invoke("make_new_note", { name });
  }
</script>

<div class="container grid place-content-center">
  <h1 class="text-3xl font-bold text-cyan-700 mb-4 underline">
    Insert a new note
  </h1>


  <form class="row" on:submit|preventDefault={make_new_note}>

<textarea bind:value={name} id="greet-input" rows="10" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Write your thoughts here..."></textarea>

<button type="submit" class="py-2.5 pt-2 px-5 me-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">Sent to file</button>

  </form>

  <p>{greetMsg}</p>
</div>

<style >
  
</style>
