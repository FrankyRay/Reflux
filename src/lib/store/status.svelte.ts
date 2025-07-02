import { registerShortcut } from "$lib/utils/shortcut";

// Command Output ~ Terminal
type CommandOutput = {
  status: "info" | "warm" | "success" | "error";
  message: string;
};

export const commandOutput: CommandOutput = $state({
  status: "info",
  message: "Reflux v0.1.0-beta.1",
});

export function setCommandOutput(value: CommandOutput) {
  commandOutput.status = value.status;
  commandOutput.message = value.message;
}

export function defaultCommandOutput(timeout: number = 5000) {
  setTimeout(
    () => setCommandOutput({ status: "info", message: "Reflux v0.1.0-beta.1" }),
    timeout
  );
}

// Progress Status ~ Progress Bar
type ProgressStatus = {
  status: "none" | "download" | "success" | "error";
  value: number;
};

export const progressStatus = $state({
  status: "none",
  value: 0,
});

export function setProgressStatus(value: ProgressStatus) {
  progressStatus.status = value.status;
  progressStatus.value = value.value;
}

export function defaultProgressStatus(timeout: number = 5000) {
  setTimeout(() => setProgressStatus({ status: "none", value: 100 }), timeout);
}

// YT-DLP Version
export const ytdlpVersion = $state({
  version: "?",
});

// Show search page
type PageTabs = {
  downloadTab: string;
  showSearch: boolean;
};

export const pageTabs = $state<PageTabs>({
  downloadTab: "output",
  showSearch: true,
});

registerShortcut("ctrl+d", () => (pageTabs.showSearch = !pageTabs.showSearch));
