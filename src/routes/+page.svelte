<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";
	import { desktopDir } from "@tauri-apps/api/path";

	let new_note_text = "";
	let folderLocation = "";
	let outputMessage = "";

	async function make_new_note() {
		setTimeout(() => {
			outputMessage = "";
		}, 3000);
		if (!folderLocation || folderLocation.length <= 0) {
			folderLocation = await desktopDir();
		}

		if (!new_note_text || folderLocation.length <= 0) {
			outputMessage = "Missing text message";
			return;
		}

		outputMessage = await invoke("make_new_note", {
			fileData: new_note_text,
			path: folderLocation,
			fileName: printDateTime() + ".txt"
		});

		new_note_text = "";
	}

	async function pickFolder(event: any) {
		try {
			event.preventDefault();
			const selectedFolder = await open({
				directory: true,
				multiple: false, // set to true if you want to allow multiple folder selection
				title: "Select a folder",
				defaultPath: folderLocation // you can specify a default path here
			});

			if (selectedFolder) {
				folderLocation = selectedFolder.toString() + (selectedFolder[selectedFolder.length - 1] != "/" ? "/" : "");
			} else {
				console.log("No folder selected");
			}
		} catch (error) {
			console.error("Error picking folder:", error);
		}
	}
	function printDateTime() {
		const now = new Date();

		// Get hours, minutes, and seconds, and pad with leading zeros if necessary
		const hours = String(now.getHours()).padStart(2, "0");
		const minutes = String(now.getMinutes()).padStart(2, "0");
		const seconds = String(now.getSeconds()).padStart(2, "0");
		const milis = String(now.getMilliseconds()).padStart(2, "0");

		// Get day, month, and year, and pad day and month with leading zeros
		const day = String(now.getDate()).padStart(2, "0");
		const month = String(now.getMonth() + 1).padStart(2, "0"); // Month is zero-indexed, so add 1
		const year = now.getFullYear();

		// Format the date and time as hhmmss-ddmmyyyy
		const formattedDateTime = `${hours}${minutes}${seconds}-${day}${month}${year}_${milis}`;

		return formattedDateTime;
	}
</script>

<div class="container grid place-content-center">
	<h1 class="text-3xl font-bold text-cyan-700 mb-4 underline">Insert a new note</h1>

	<form on:submit|preventDefault={make_new_note}>
		<textarea
			bind:value={new_note_text}
			id="greet-input"
			rows="10"
			class="my-1 block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
			placeholder="Write your note here..."
		></textarea>

		<button
			type="button"
			on:click={pickFolder}
			class="text-xs text-white bg-gray-800 hover:bg-gray-900 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-full text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700"
			>Select Folder</button
		>
		<br />
		<button
			type="submit"
			class="text-white bg-yellow-400 hover:bg-yellow-500 focus:outline-none focus:ring-4 focus:ring-yellow-300 font-medium rounded-full text-sm px-5 py-2.5 text-center me-2 mb-2 dark:focus:ring-yellow-900"
			>Submit</button
		>
	</form>

	<div>{outputMessage}</div>
</div>

<style>
</style>
