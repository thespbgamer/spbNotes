<script lang="ts">
	import {invoke} from "@tauri-apps/api/tauri";
  import { open } from '@tauri-apps/api/dialog';
  import { desktopDir } from '@tauri-apps/api/path';


  let new_note_text = "";
  let folderLocation = "";
  let outputMessage = "";

  async function make_new_note() {
      if(!folderLocation || folderLocation.length <= 0){
        folderLocation = await desktopDir();
      }

      if(!new_note_text || folderLocation.length <= 0){
        outputMessage = 'Missing text message'
        return;
      }

      outputMessage = await invoke("make_new_note", { fileData: new_note_text, path: folderLocation, fileName: (printDateTime() + '.txt') });
    }



async function pickFolder() {
  try {
    const selectedFolder = await open({
      directory: true,
      multiple: false, // set to true if you want to allow multiple folder selection
      title: 'Select a folder',
      defaultPath: folderLocation // you can specify a default path here
    });

    if (selectedFolder) {
      if(selectedFolder[selectedFolder.length -1] != '/'){
        
      }
      outputMessage = selectedFolder.toString() + (selectedFolder[selectedFolder.length -1] != '/' ? '/' : '');
      folderLocation = outputMessage;
    } else {
      console.log('No folder selected');
    }
  } catch (error) {
    console.error('Error picking folder:', error);
  }
}
function printDateTime() {
  const now = new Date();

  // Get hours, minutes, and seconds, and pad with leading zeros if necessary
  const hours = String(now.getHours()).padStart(2, '0');
  const minutes = String(now.getMinutes()).padStart(2, '0');
  const seconds = String(now.getSeconds()).padStart(2, '0');
  const milis = String(now.getMilliseconds()).padStart(2, '0');

  // Get day, month, and year, and pad day and month with leading zeros
  const day = String(now.getDate()).padStart(2, '0');
  const month = String(now.getMonth() + 1).padStart(2, '0'); // Month is zero-indexed, so add 1
  const year = now.getFullYear();

  // Format the date and time as hhmmss-ddmmyyyy
  const formattedDateTime = `${hours}${minutes}${seconds}-${day}${month}${year}_${milis}`;

  return formattedDateTime;
}


</script>

<div class="container grid place-content-center">




  <h1 class="text-3xl font-bold text-cyan-700 mb-4 underline">
    Insert a new note
  </h1>

<button on:click={pickFolder}>Select Folder</button>

  <form class="row" on:submit|preventDefault={make_new_note}>

<textarea bind:value={new_note_text} id="greet-input" rows="10" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Write your thoughts here..."></textarea>


<button type="submit" class="py-2.5 pt-2 px-5 me-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">Sent to file</button>

  </form>

  <div style="white-space: pre-wrap;">{outputMessage}</div>
</div>

<style >
  
</style>
