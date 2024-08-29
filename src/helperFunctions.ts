import { isRegistered, register } from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";

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
