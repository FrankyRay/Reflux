import { once } from "@tauri-apps/api/event";
import { downloadDir } from "@tauri-apps/api/path";
import { LazyStore } from "@tauri-apps/plugin-store";

class Setting {
  private storeFile = new LazyStore("settings.json", { autoSave: false });

  private _output: OutputSetting = $state({
    directory: "",
    output: "",
  });

  async initialize() {
    const output = await this.storeFile.get<OutputSetting>("output");
    if (output) this.output = output;
    else await this.defaultOutput();
  }

  get output() {
    return this._output;
  }

  set output(value: Partial<OutputSetting>) {
    const keys = Object.keys(value) as Array<keyof OutputSetting>;
    keys.forEach((key) => {
      const val = value[key];
      if (val !== undefined) {
        this._output[key] = val;
      }
    });

    this.storeFile.set("output", this._output);
  }

  async defaultOutput() {
    this.output = {
      directory: await downloadDir(),
      output: "%(title).200B [%(id)s].%(ext)s",
    };
  }
}

export const setting = new Setting();
setting.initialize();
