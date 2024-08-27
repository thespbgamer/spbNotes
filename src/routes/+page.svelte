<script lang="ts">
	import Notification from "./Notification.svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";
	import { desktopDir } from "@tauri-apps/api/path";
	import { isRegistered, register } from "@tauri-apps/api/globalShortcut";
	import { appWindow } from "@tauri-apps/api/window";

	let new_note_text = "";
	let folderLocation = "";
	let outputMessage = "";
	let timeout: any;

	registerShortcuts();
	async function registerShortcuts() {
		if ((await isRegistered("CommandOrControl+Super+Alt+A")) == false) {
			await register("CommandOrControl+Super+Alt+A", () => {
				appWindow.show();
				appWindow.center();
				appWindow.setFocus();
				document.getElementById("greet-input")?.focus();
			});
		}
	}

	async function make_new_note(event: any) {
		event.preventDefault();
		new_note_text = new_note_text.trim();
		if (timeout) {
			clearTimeout(timeout);
		}

		timeout = setTimeout(() => {
			outputMessage = "";
		}, 3000);

		if (!folderLocation || folderLocation.length <= 0) {
			folderLocation = await desktopDir();
		}

		if (!new_note_text || folderLocation.length <= 0) {
			outputMessage = "You need to insert text first!";
			return;
		}

		outputMessage = await invoke("make_new_note", {
			fileData: new_note_text,
			path: folderLocation,
			fileName: printDateTime() + ".txt"
		});

		new_note_text = "";
	}

	async function pickFolder() {
		try {
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

	function handleKeydown(event: any) {
		// Check if CTRL + Enter was pressed
		if (event.ctrlKey && event.key === "Enter") {
			make_new_note(event);
		}
	}
</script>

<div class="container grid place-content-center">
	<h1 class="text-3xl font-bold text-cyan-500 mb-4 underline">Insert a new note</h1>

	<form on:submit|preventDefault={make_new_note}>
		<textarea
			bind:value={new_note_text}
			id="greet-input"
			rows="10"
			class="my-1 block p-2.5 w-full text-sm text-white bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-zinc-800 dark:border-zinc-600 dark:placeholder-zinc-500 dark:text-white dark:focus:ring-orange-500 dark:focus:border-orange-500"
			placeholder="Write your note here..."
			on:keydown={handleKeydown}
		></textarea>

		<button
			type="button"
			on:click|preventDefault={pickFolder}
			class="text-xs text-white bg-gray-800 hover:bg-gray-900 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-full text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700"
			>Select Folder</button
		>
		<br />
		<button
			type="submit"
			class="text-white bg-yellow-600 hover:bg-yellow-500 focus:outline-none focus:ring-4 focus:ring-yellow-300 font-medium rounded-full text-sm px-5 py-2.5 text-center me-2 mb-2 dark:focus:ring-yellow-900"
			>Submit</button
		>
	</form>

	{#if outputMessage}
		<Notification>{outputMessage}</Notification>
	{/if}
</div>

<style>
</style>
