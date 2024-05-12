import { errorPrefix, generatedAttribute } from "./Utils/Constants.js";
import { executeOnSingleOrMultiple, getLogger, itemFromSingleOrMultiple } from "../Utils/Utils.js";
import { EventDispatcher } from "../Utils/EventDispatcher.js";
import { getRandom } from "../Utils/NumberUtils.js";
async function getItemsFromInitializer(container, map, initializers, force = false) {
    let res = map.get(container);
    if (!res || force) {
        res = await Promise.all([...initializers.values()].map((t) => t(container)));
        map.set(container, res);
    }
    return res;
}
async function getDataFromUrl(data) {
    const url = itemFromSingleOrMultiple(data.url, data.index);
    if (!url) {
        return data.fallback;
    }
    const response = await fetch(url);
    if (response.ok) {
        return (await response.json());
    }
    getLogger().error(`${errorPrefix} ${response.status} while retrieving config file`);
    return data.fallback;
}
export class Engine {
    constructor() {
        this._configs = new Map();
        this._domArray = [];
        this._eventDispatcher = new EventDispatcher();
        this._initialized = false;
        this.plugins = [];
        this._initializers = {
            interactors: new Map(),
            movers: new Map(),
            updaters: new Map(),
        };
        this.interactors = new Map();
        this.movers = new Map();
        this.updaters = new Map();
        this.presets = new Map();
        this.effectDrawers = new Map();
        this.shapeDrawers = new Map();
        this.pathGenerators = new Map();
    }
    get configs() {
        const res = {};
        for (const [name, config] of this._configs) {
            res[name] = config;
        }
        return res;
    }
    get version() {
        return "3.3.0";
    }
    addConfig(config) {
        const key = config.key ?? config.name ?? "default";
        this._configs.set(key, config);
        this._eventDispatcher.dispatchEvent("configAdded", { data: { name: key, config } });
    }
    async addEffect(effect, drawer, refresh = true) {
        executeOnSingleOrMultiple(effect, (type) => {
            if (!this.getEffectDrawer(type)) {
                this.effectDrawers.set(type, drawer);
            }
        });
        await this.refresh(refresh);
    }
    addEventListener(type, listener) {
        this._eventDispatcher.addEventListener(type, listener);
    }
    async addInteractor(name, interactorInitializer, refresh = true) {
        this._initializers.interactors.set(name, interactorInitializer);
        await this.refresh(refresh);
    }
    async addMover(name, moverInitializer, refresh = true) {
        this._initializers.movers.set(name, moverInitializer);
        await this.refresh(refresh);
    }
    async addParticleUpdater(name, updaterInitializer, refresh = true) {
        this._initializers.updaters.set(name, updaterInitializer);
        await this.refresh(refresh);
    }
    async addPathGenerator(name, generator, refresh = true) {
        if (!this.getPathGenerator(name)) {
            this.pathGenerators.set(name, generator);
        }
        await this.refresh(refresh);
    }
    async addPlugin(plugin, refresh = true) {
        if (!this.getPlugin(plugin.id)) {
            this.plugins.push(plugin);
        }
        await this.refresh(refresh);
    }
    async addPreset(preset, options, override = false, refresh = true) {
        if (override || !this.getPreset(preset)) {
            this.presets.set(preset, options);
        }
        await this.refresh(refresh);
    }
    async addShape(shape, drawer, refresh = true) {
        executeOnSingleOrMultiple(shape, (type) => {
            if (!this.getShapeDrawer(type)) {
                this.shapeDrawers.set(type, drawer);
            }
        });
        await this.refresh(refresh);
    }
    clearPlugins(container) {
        this.updaters.delete(container);
        this.movers.delete(container);
        this.interactors.delete(container);
    }
    dispatchEvent(type, args) {
        this._eventDispatcher.dispatchEvent(type, args);
    }
    dom() {
        return this._domArray;
    }
    domItem(index) {
        const dom = this.dom(), item = dom[index];
        if (!item || item.destroyed) {
            const deleteCount = 1;
            dom.splice(index, deleteCount);
            return;
        }
        return item;
    }
    async getAvailablePlugins(container) {
        const res = new Map();
        for (const plugin of this.plugins) {
            if (plugin.needsPlugin(container.actualOptions)) {
                res.set(plugin.id, await plugin.getPlugin(container));
            }
        }
        return res;
    }
    getEffectDrawer(type) {
        return this.effectDrawers.get(type);
    }
    async getInteractors(container, force = false) {
        return await getItemsFromInitializer(container, this.interactors, this._initializers.interactors, force);
    }
    async getMovers(container, force = false) {
        return await getItemsFromInitializer(container, this.movers, this._initializers.movers, force);
    }
    getPathGenerator(type) {
        return this.pathGenerators.get(type);
    }
    getPlugin(plugin) {
        return this.plugins.find((t) => t.id === plugin);
    }
    getPreset(preset) {
        return this.presets.get(preset);
    }
    getShapeDrawer(type) {
        return this.shapeDrawers.get(type);
    }
    getSupportedEffects() {
        return this.effectDrawers.keys();
    }
    getSupportedShapes() {
        return this.shapeDrawers.keys();
    }
    async getUpdaters(container, force = false) {
        return await getItemsFromInitializer(container, this.updaters, this._initializers.updaters, force);
    }
    init() {
        if (this._initialized) {
            return;
        }
        this._initialized = true;
    }
    async load(params) {
        const randomFactor = 10000, id = params.id ?? params.element?.id ?? `tsparticles${Math.floor(getRandom() * randomFactor)}`, { index, url } = params, options = url ? await getDataFromUrl({ fallback: params.options, url, index }) : params.options;
        let domContainer = params.element ?? document.getElementById(id);
        if (!domContainer) {
            domContainer = document.createElement("div");
            domContainer.id = id;
            document.body.append(domContainer);
        }
        const currentOptions = itemFromSingleOrMultiple(options, index), dom = this.dom(), oldIndex = dom.findIndex((v) => v.id.description === id), minIndex = 0;
        if (oldIndex >= minIndex) {
            const old = this.domItem(oldIndex);
            if (old && !old.destroyed) {
                old.destroy();
                const deleteCount = 1;
                dom.splice(oldIndex, deleteCount);
            }
        }
        let canvasEl;
        if (domContainer.tagName.toLowerCase() === "canvas") {
            canvasEl = domContainer;
            canvasEl.dataset[generatedAttribute] = "false";
        }
        else {
            const existingCanvases = domContainer.getElementsByTagName("canvas");
            if (existingCanvases.length) {
                const firstIndex = 0;
                canvasEl = existingCanvases[firstIndex];
                canvasEl.dataset[generatedAttribute] = "false";
            }
            else {
                canvasEl = document.createElement("canvas");
                canvasEl.dataset[generatedAttribute] = "true";
                domContainer.appendChild(canvasEl);
            }
        }
        if (!canvasEl.style.width) {
            canvasEl.style.width = "100%";
        }
        if (!canvasEl.style.height) {
            canvasEl.style.height = "100%";
        }
        const { Container } = await import("./Container.js"), newItem = new Container(this, id, currentOptions);
        if (oldIndex >= minIndex) {
            const deleteCount = 0;
            dom.splice(oldIndex, deleteCount, newItem);
        }
        else {
            dom.push(newItem);
        }
        newItem.canvas.loadCanvas(canvasEl);
        await newItem.start();
        return newItem;
    }
    loadOptions(options, sourceOptions) {
        for (const plugin of this.plugins) {
            plugin.loadOptions(options, sourceOptions);
        }
    }
    loadParticlesOptions(container, options, ...sourceOptions) {
        const updaters = this.updaters.get(container);
        if (!updaters) {
            return;
        }
        for (const updater of updaters) {
            updater.loadOptions?.(options, ...sourceOptions);
        }
    }
    async refresh(refresh = true) {
        if (!refresh) {
            return;
        }
        await Promise.all(this.dom().map((t) => t.refresh()));
    }
    removeEventListener(type, listener) {
        this._eventDispatcher.removeEventListener(type, listener);
    }
    setOnClickHandler(callback) {
        const dom = this.dom();
        if (!dom.length) {
            throw new Error(`${errorPrefix} can only set click handlers after calling tsParticles.load()`);
        }
        for (const domItem of dom) {
            domItem.addClickHandler(callback);
        }
    }
}