import { deepExtend } from "../../Utils/Utils.js";
const defaultPosition = 50;
export class ManualParticle {
    load(data) {
        if (!data) {
            return;
        }
        if (data.position) {
            this.position = {
                x: data.position.x ?? defaultPosition,
                y: data.position.y ?? defaultPosition,
                mode: data.position.mode ?? "percent",
            };
        }
        if (data.options) {
            this.options = deepExtend({}, data.options);
        }
    }
}
