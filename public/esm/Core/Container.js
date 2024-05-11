import { errorPrefix, millisecondsToSeconds } from "./Utils/Constants.js";
import { getLogger, safeIntersectionObserver } from "../Utils/Utils.js";
import { Canvas } from "./Canvas.js";
import { EventListeners } from "./Utils/EventListeners.js";
import { Options } from "../Options/Classes/Options.js";
import { Particles } from "./Particles.js";
import { Retina } from "./Retina.js";
import { getRangeValue } from "../Utils/NumberUtils.js";
import { loadOptions } from "../Utils/OptionsUtils.js";
function guardCheck(container) {
    return container && !container.destroyed;
}
const defaultFps = 60;
function initDelta(value, fpsLimit = defaultFps, smooth = false) {
    return {
        value,
        factor: smooth ? defaultFps / fpsLimit : (defaultFps * value) / millisecondsToSeconds,
    };
}
function loadContainerOptions(engine, container, ...sourceOptionsArr) {
    const options = new Options(engine, container);
    loadOptions(options, ...sourceOptionsArr);
    return options;
}
export class Container {
    constructor(engine, id, sourceOptions) {
        this._intersectionManager = (entries) => {
            if (!guardCheck(this) || !this.actualOptions.pauseOnOutsideViewport) {
                return;
            }
            for (const entry of entries) {
                if (entry.target !== this.interactivity.element) {
                    continue;
                }
                if (entry.isIntersecting) {
                    void this.play();
                }
                else {
                    this.pause();
                }
            }
        };
        this._nextFrame = (timestamp) => {
            try {
                if (!this._smooth &&
                    this._lastFrameTime !== undefined &&
                    timestamp < this._lastFrameTime + millisecondsToSeconds / this.fpsLimit) {
                    this.draw(false);
                    return;
                }
                this._lastFrameTime ??= timestamp;
                const delta = initDelta(timestamp - this._lastFrameTime, this.fpsLimit, this._smooth);
                this.addLifeTime(delta.value);
                this._lastFrameTime = timestamp;
                if (delta.value > millisecondsToSeconds) {
                    this.draw(false);
                    return;
                }
                this.particles.draw(delta);
                if (!this.alive()) {
                    this.destroy();
                    return;
                }
                if (this.animationStatus) {
                    this.draw(false);
                }
            }
            catch (e) {
                getLogger().error(`${errorPrefix} in animation loop`, e);
            }
        };
        this._engine = engine;
        this.id = Symbol(id);
        this.fpsLimit = 120;
        this._smooth = false;
        this._delay = 0;
        this._duration = 0;
        this._lifeTime = 0;
        this._firstStart = true;
        this.started = false;
        this.destroyed = false;
        this._paused = true;
        this._lastFrameTime = 0;
        this.zLayers = 100;
        this.pageHidden = false;
        this._sourceOptions = sourceOptions;
        this._initialSourceOptions = sourceOptions;
        this.retina = new Retina(this);
        this.canvas = new Canvas(this);
        this.particles = new Particles(this._engine, this);
        this.pathGenerators = new Map();
        this.interactivity = {
            mouse: {
                clicking: false,
                inside: false,
            },
        };
        this.plugins = new Map();
        this.effectDrawers = new Map();
        this.shapeDrawers = new Map();
        this._options = loadContainerOptions(this._engine, this);
        this.actualOptions = loadContainerOptions(this._engine, this);
        this._eventListeners = new EventListeners(this);
        this._intersectionObserver = safeIntersectionObserver((entries) => this._intersectionManager(entries));
        this._engine.dispatchEvent("containerBuilt", { container: this });
    }
    get animationStatus() {
        return !this._paused && !this.pageHidden && guardCheck(this);
    }
    get options() {
        return this._options;
    }
    get sourceOptions() {
        return this._sourceOptions;
    }
    addClickHandler(callback) {
        if (!guardCheck(this)) {
            return;
        }
        const el = this.interactivity.element;
        if (!el) {
            return;
        }
        const clickOrTouchHandler = (e, pos, radius) => {
            if (!guardCheck(this)) {
                return;
            }
            const pxRatio = this.retina.pixelRatio, posRetina = {
                x: pos.x * pxRatio,
                y: pos.y * pxRatio,
            }, particles = this.particles.quadTree.queryCircle(posRetina, radius * pxRatio);
            callback(e, particles);
        };
        const clickHandler = (e) => {
            if (!guardCheck(this)) {
                return;
            }
            const mouseEvent = e, pos = {
                x: mouseEvent.offsetX || mouseEvent.clientX,
                y: mouseEvent.offsetY || mouseEvent.clientY,
            }, radius = 1;
            clickOrTouchHandler(e, pos, radius);
        };
        const touchStartHandler = () => {
            if (!guardCheck(this)) {
                return;
            }
            touched = true;
            touchMoved = false;
        };
        const touchMoveHandler = () => {
            if (!guardCheck(this)) {
                return;
            }
            touchMoved = true;
        };
        const touchEndHandler = (e) => {
            if (!guardCheck(this)) {
                return;
            }
            if (touched && !touchMoved) {
                const touchEvent = e;
                const lengthOffset = 1;
                let lastTouch = touchEvent.touches[touchEvent.touches.length - lengthOffset];
                if (!lastTouch) {
                    lastTouch = touchEvent.changedTouches[touchEvent.changedTouches.length - lengthOffset];
                    if (!lastTouch) {
                        return;
                    }
                }
                const element = this.canvas.element, canvasRect = element ? element.getBoundingClientRect() : undefined, minCoordinate = 0, pos = {
                    x: lastTouch.clientX - (canvasRect ? canvasRect.left : minCoordinate),
                    y: lastTouch.clientY - (canvasRect ? canvasRect.top : minCoordinate),
                };
                clickOrTouchHandler(e, pos, Math.max(lastTouch.radiusX, lastTouch.radiusY));
            }
            touched = false;
            touchMoved = false;
        };
        const touchCancelHandler = () => {
            if (!guardCheck(this)) {
                return;
            }
            touched = false;
            touchMoved = false;
        };
        let touched = false, touchMoved = false;
        el.addEventListener("click", clickHandler);
        el.addEventListener("touchstart", touchStartHandler);
        el.addEventListener("touchmove", touchMoveHandler);
        el.addEventListener("touchend", touchEndHandler);
        el.addEventListener("touchcancel", touchCancelHandler);
    }
    addLifeTime(value) {
        this._lifeTime += value;
    }
    addPath(key, generator, override = false) {
        if (!guardCheck(this) || (!override && this.pathGenerators.has(key))) {
            return false;
        }
        this.pathGenerators.set(key, generator);
        return true;
    }
    alive() {
        return !this._duration || this._lifeTime <= this._duration;
    }
    destroy() {
        if (!guardCheck(this)) {
            return;
        }
        this.stop();
        this.particles.destroy();
        this.canvas.destroy();
        for (const [, effectDrawer] of this.effectDrawers) {
            effectDrawer.destroy?.(this);
        }
        for (const [, shapeDrawer] of this.shapeDrawers) {
            shapeDrawer.destroy?.(this);
        }
        for (const key of this.effectDrawers.keys()) {
            this.effectDrawers.delete(key);
        }
        for (const key of this.shapeDrawers.keys()) {
            this.shapeDrawers.delete(key);
        }
        this._engine.clearPlugins(this);
        this.destroyed = true;
        const mainArr = this._engine.dom(), idx = mainArr.findIndex((t) => t === this), minIndex = 0;
        if (idx >= minIndex) {
            const deleteCount = 1;
            mainArr.splice(idx, deleteCount);
        }
        this._engine.dispatchEvent("containerDestroyed", { container: this });
    }
    draw(force) {
        if (!guardCheck(this)) {
            return;
        }
        let refreshTime = force;
        const frame = (timestamp) => {
            if (refreshTime) {
                this._lastFrameTime = undefined;
                refreshTime = false;
            }
            this._nextFrame(timestamp);
        };
        this._drawAnimationFrame = requestAnimationFrame((timestamp) => frame(timestamp));
    }
    async export(type, options = {}) {
        for (const [, plugin] of this.plugins) {
            if (!plugin.export) {
                continue;
            }
            const res = await plugin.export(type, options);
            if (!res.supported) {
                continue;
            }
            return res.blob;
        }
        getLogger().error(`${errorPrefix} - Export plugin with type ${type} not found`);
    }
    handleClickMode(mode) {
        if (!guardCheck(this)) {
            return;
        }
        this.particles.handleClickMode(mode);
        for (const [, plugin] of this.plugins) {
            plugin.handleClickMode?.(mode);
        }
    }
    async init() {
        if (!guardCheck(this)) {
            return;
        }
        const effects = this._engine.getSupportedEffects();
        for (const type of effects) {
            const drawer = this._engine.getEffectDrawer(type);
            if (drawer) {
                this.effectDrawers.set(type, drawer);
            }
        }
        const shapes = this._engine.getSupportedShapes();
        for (const type of shapes) {
            const drawer = this._engine.getShapeDrawer(type);
            if (drawer) {
                this.shapeDrawers.set(type, drawer);
            }
        }
        await this.particles.initPlugins();
        this._options = loadContainerOptions(this._engine, this, this._initialSourceOptions, this.sourceOptions);
        this.actualOptions = loadContainerOptions(this._engine, this, this._options);
        const availablePlugins = await this._engine.getAvailablePlugins(this);
        for (const [id, plugin] of availablePlugins) {
            this.plugins.set(id, plugin);
        }
        this.retina.init();
        await this.canvas.init();
        this.updateActualOptions();
        this.canvas.initBackground();
        this.canvas.resize();
        this.zLayers = this.actualOptions.zLayers;
        this._duration = getRangeValue(this.actualOptions.duration) * millisecondsToSeconds;
        this._delay = getRangeValue(this.actualOptions.delay) * millisecondsToSeconds;
        this._lifeTime = 0;
        const defaultFpsLimit = 120, minFpsLimit = 0;
        this.fpsLimit = this.actualOptions.fpsLimit > minFpsLimit ? this.actualOptions.fpsLimit : defaultFpsLimit;
        this._smooth = this.actualOptions.smooth;
        for (const [, drawer] of this.effectDrawers) {
            await drawer.init?.(this);
        }
        for (const [, drawer] of this.shapeDrawers) {
            await drawer.init?.(this);
        }
        for (const [, plugin] of this.plugins) {
            await plugin.init?.();
        }
        this._engine.dispatchEvent("containerInit", { container: this });
        await this.particles.init();
        this.particles.setDensity();
        for (const [, plugin] of this.plugins) {
            plugin.particlesSetup?.();
        }
        this._engine.dispatchEvent("particlesSetup", { container: this });
    }
    async loadTheme(name) {
        if (!guardCheck(this)) {
            return;
        }
        this._currentTheme = name;
        await this.refresh();
    }
    pause() {
        if (!guardCheck(this)) {
            return;
        }
        if (this._drawAnimationFrame !== undefined) {
            cancelAnimationFrame(this._drawAnimationFrame);
            delete this._drawAnimationFrame;
        }
        if (this._paused) {
            return;
        }
        for (const [, plugin] of this.plugins) {
            plugin.pause?.();
        }
        if (!this.pageHidden) {
            this._paused = true;
        }
        this._engine.dispatchEvent("containerPaused", { container: this });
    }
    play(force) {
        if (!guardCheck(this)) {
            return;
        }
        const needsUpdate = this._paused || force;
        if (this._firstStart && !this.actualOptions.autoPlay) {
            this._firstStart = false;
            return;
        }
        if (this._paused) {
            this._paused = false;
        }
        if (needsUpdate) {
            for (const [, plugin] of this.plugins) {
                if (plugin.play) {
                    plugin.play();
                }
            }
        }
        this._engine.dispatchEvent("containerPlay", { container: this });
        this.draw(needsUpdate ?? false);
    }
    async refresh() {
        if (!guardCheck(this)) {
            return;
        }
        this.stop();
        return this.start();
    }
    async reset() {
        if (!guardCheck(this)) {
            return;
        }
        this._initialSourceOptions = undefined;
        this._options = loadContainerOptions(this._engine, this);
        this.actualOptions = loadContainerOptions(this._engine, this, this._options);
        return this.refresh();
    }
    async start() {
        if (!guardCheck(this) || this.started) {
            return;
        }
        await this.init();
        this.started = true;
        await new Promise((resolve) => {
            const start = async () => {
                this._eventListeners.addListeners();
                if (this.interactivity.element instanceof HTMLElement && this._intersectionObserver) {
                    this._intersectionObserver.observe(this.interactivity.element);
                }
                for (const [, plugin] of this.plugins) {
                    await plugin.start?.();
                }
                this._engine.dispatchEvent("containerStarted", { container: this });
                this.play();
                resolve();
            };
            this._delayTimeout = setTimeout(() => void start(), this._delay);
        });
    }
    stop() {
        if (!guardCheck(this) || !this.started) {
            return;
        }
        if (this._delayTimeout) {
            clearTimeout(this._delayTimeout);
            delete this._delayTimeout;
        }
        this._firstStart = true;
        this.started = false;
        this._eventListeners.removeListeners();
        this.pause();
        this.particles.clear();
        this.canvas.stop();
        if (this.interactivity.element instanceof HTMLElement && this._intersectionObserver) {
            this._intersectionObserver.unobserve(this.interactivity.element);
        }
        for (const [, plugin] of this.plugins) {
            plugin.stop?.();
        }
        for (const key of this.plugins.keys()) {
            this.plugins.delete(key);
        }
        this._sourceOptions = this._options;
        this._engine.dispatchEvent("containerStopped", { container: this });
    }
    updateActualOptions() {
        this.actualOptions.responsive = [];
        const newMaxWidth = this.actualOptions.setResponsive(this.canvas.size.width, this.retina.pixelRatio, this._options);
        this.actualOptions.setTheme(this._currentTheme);
        if (this._responsiveMaxWidth === newMaxWidth) {
            return false;
        }
        this._responsiveMaxWidth = newMaxWidth;
        return true;
    }
}
