export class ParticlesNumberLimit {
    constructor() {
        this.mode = "delete";
        this.value = 0;
    }
    load(data) {
        if (!data) {
            return;
        }
        if (data.mode !== undefined) {
            this.mode = data.mode;
        }
        if (data.value !== undefined) {
            this.value = data.value;
        }
    }
}
