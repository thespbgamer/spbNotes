import { isRegistered, register } from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";

export async function registerShortcuts() {
	const registered = await isRegistered("CommandOrControl+Super+Alt+A");

	if (!registered) {
		await register("CommandOrControl+Super+Alt+A", async () => {
			//await appWindow.setAlwaysOnTop(true);
			await appWindow.show();

			await appWindow.setFocus();
			//await appWindow.setAlwaysOnTop(false);
			document.getElementById("greet-input")?.focus();
			console.log("End");
		});
	}
}
