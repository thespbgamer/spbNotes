import { isRegistered, register } from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/dialog";
import { desktopDir } from "@tauri-apps/api/path";

export async function registerShortcuts() {
	const registered = await isRegistered("CommandOrControl+Super+Alt+A");

	if (!registered) {
		await register("CommandOrControl+Super+Alt+A", async () => {
			console.log("Start show app shortcut");
			if (await appWindow.isVisible()) {
				await appWindow.setAlwaysOnTop(false);
				await appWindow.hide();
			} else {
				await appWindow.setAlwaysOnTop(true);
				await appWindow.show();

				while (!appWindow.isVisible()) {}

				await appWindow.center();

				await appWindow.setFocus();
				document.getElementById("greet-input")?.focus();
				await appWindow.setAlwaysOnTop(false);
			}
		});
	}
}

export async function pickFolder(folderLocation: string) {
	try {
		const selectedFolder = await open({
			directory: true,
			multiple: false, // set to true if you want to allow multiple folder selection
			title: "Select a folder",
			defaultPath: folderLocation // you can specify a default path here
		});

		if (selectedFolder) {
			folderLocation = selectedFolder.toString() + (selectedFolder[selectedFolder.length - 1] != "/" ? "/" : "");
			localStorage.setItem("location", folderLocation);
		} else {
			console.log("No folder selected");
		}
	} catch (error) {
		console.error("Error picking folder:", error);
	} finally {
		return folderLocation;
	}
}
export async function resetFolderLocation() {
	localStorage.removeItem("location");
	return await desktopDir();
}
export function printDateTime() {
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
