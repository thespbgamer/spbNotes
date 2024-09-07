<script lang="ts">
	import Notification from "./Notification.svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { registerShortcuts, pickFolder, printDateTime, resetFolderLocation } from "../helperFunctions";

	let new_note_text: string = "";
	let folderLocation: string = localStorage.getItem("location") ?? "";
	let currentNotification: string = "";
	let timeout: ReturnType<typeof setTimeout>;
	startup();
	registerShortcuts();

	async function make_new_note() {
		new_note_text = new_note_text.trim();
		if (timeout) {
			clearTimeout(timeout);
		}

		timeout = setTimeout(() => {
			currentNotification = "";
		}, 3000);

		if (!new_note_text || new_note_text.length <= 0) {
			currentNotification = "You need to insert text first!";
			return;
		}

		currentNotification = await invoke("make_new_note", {
			fileData: new_note_text,
			path: folderLocation,
			fileName: printDateTime() + ".txt"
		});

		new_note_text = "";
	}

	function handleKeydown(event: KeyboardEvent) {
		// Check if CTRL + Enter was pressed
		if (event.ctrlKey && event.key === "Enter") {
			event.preventDefault();
			make_new_note();
		}
	}
	async function startup() {
		if (!folderLocation || folderLocation.length <= 0) {
			folderLocation = await resetFolderLocation();
		}
	}
</script>

<div class="container grid place-content-center w-full">
	<h1 class="my-2 text-4xl font-extrabold text-gray-900 md:text-5xl lg:text-6xl text-orange-500">Add Your Notes</h1>
	<form on:submit|preventDefault={make_new_note}>
		<h6 class="text-lg font-bold dark:text-white">Message:</h6>

		<textarea
			bind:value={new_note_text}
			id="greet-input"
			rows="10"
			class="my-1 block p-2.5 w-full text-sm text-white rounded-lg border bg-zinc-800 border-zinc-600 placeholder-zinc-500 focus:ring-orange-500 focus:border-orange-500"
			placeholder="Write your note here..."
			on:keydown={handleKeydown}
		></textarea>

		<h6 class="text-lg font-bold text-white">Location:</h6>
		<button
			type="button"
			class="text-xs mb-5 font-semibold text-zinc-300 cursor-pointer"
			on:click={() => {
				navigator.clipboard.writeText(folderLocation);
				currentNotification = "Location path was copied.";

				timeout = setTimeout(() => {
					currentNotification = "";
				}, 1000);
			}}
		>
			{folderLocation}
		</button>

		<br />
		<button
			type="submit"
			class="text-white w-full bg-orange-500 hover:bg-orange-400 focus:outline-none focus:ring-4 font-medium rounded-full text-sm px-5 py-2.5 text-center me-2 mb-2 focus:ring-orange-900"
			>Submit</button
		>
	</form>
	<button
		type="button"
		on:click|preventDefault={async () => {
			folderLocation = await pickFolder(folderLocation);
		}}
		class="text-xs absolute right-5 bottom-5 text-white focus:outline-none focus:ring-4 rounded-full px-5 py-2.5 bg-zinc-900 hover:bg-zinc-600 focus:ring-zinc-500 border-zinc-600"
		>Change Folder</button
	><button
		type="button"
		on:click|preventDefault={async () => {
			folderLocation = await resetFolderLocation();
		}}
		class="text-xs absolute left-5 bottom-5 text-white focus:outline-none focus:ring-4 rounded-full px-5 py-2.5 bg-zinc-900 hover:bg-zinc-600 focus:ring-zinc-500 border-zinc-600"
		>Reset folder</button
	>
	{#if currentNotification}
		<Notification>{currentNotification}</Notification>
	{/if}
</div>

<style>
</style>
