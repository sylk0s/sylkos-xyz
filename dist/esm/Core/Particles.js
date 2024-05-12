import { getLogger, getPosition } from "../Utils/Utils.js";
import { InteractionManager } from "./Utils/InteractionManager.js";
import { Particle } from "./Particle.js";
import { Point } from "./Utils/Point.js";
import { QuadTree } from "./Utils/QuadTree.js";
import { Rectangle } from "./Utils/Ranges.js";
import { errorPrefix } from "./Utils/Constants.js";
const qTreeCapacity = 4, squareExp = 2, defaultRemoveQuantity = 1;
const qTreeRectangle = (canvasSize) => {
    const { height, width } = canvasSize, posOffset = -0.25, sizeFactor = 1.5;
    return new Rectangle(posOffset * width, posOffset * height, sizeFactor * width, sizeFactor * height);
};
export class Particles {
    constructor(engine, container) {
        this._addToPool = (...particles) => {
            for (const particle of particles) {
                this._pool.push(particle);
            }
        };
        this._applyDensity = (options, manualCount, group) => {
            const numberOptions = options.number;
            if (!options.number.density?.enable) {
                if (group === undefined) {
                    this._limit = numberOptions.limit.value;
                }
                else if (numberOptions.limit) {
                    this._groupLimits.set(group, numberOptions.limit.value);
                }
                return;
            }
            const densityFactor = this._initDensityFactor(numberOptions.density), optParticlesNumber = numberOptions.value, minLimit = 0, optParticlesLimit = numberOptions.limit.value > minLimit ? numberOptions.limit.value : optParticlesNumber, particlesNumber = Math.min(optParticlesNumber, optParticlesLimit) * densityFactor + manualCount, particlesCount = Math.min(this.count, this.filter((t) => t.group === group).length);
            if (group === undefined) {
                this._limit = numberOptions.limit.value * densityFactor;
            }
            else {
                this._groupLimits.set(group, numberOptions.limit.value * densityFactor);
            }
            if (particlesCount < particlesNumber) {
                this.push(Math.abs(particlesNumber - particlesCount), undefined, options, group);
            }
            else if (particlesCount > particlesNumber) {
                this.removeQuantity(particlesCount - particlesNumber, group);
            }
        };
        this._initDensityFactor = (densityOptions) => {
            const container = this._container, defaultFactor = 1;
            if (!container.canvas.element || !densityOptions.enable) {
                return defaultFactor;
            }
            const canvas = container.canvas.element, pxRatio = container.retina.pixelRatio;
            return (canvas.width * canvas.height) / (densityOptions.height * densityOptions.width * pxRatio ** squareExp);
        };
        this._pushParticle = (position, overrideOptions, group, initializer) => {
            try {
                let particle = this._pool.pop();
                if (!particle) {
                    particle = new Particle(this._engine, this._container);
                }
                particle.init(this._nextId, position, overrideOptions, group);
                let canAdd = true;
                if (initializer) {
                    canAdd = initializer(particle);
                }
                if (!canAdd) {
                    return;
                }
                this._array.push(particle);
                this._zArray.push(particle);
                this._nextId++;
                this._engine.dispatchEvent("particleAdded", {
                    container: this._container,
                    data: {
                        particle,
                    },
                });
                return particle;
            }
            catch (e) {
                getLogger().warning(`${errorPrefix} adding particle: ${e}`);
            }
        };
        this._removeParticle = (index, group, override) => {
            const particle = this._array[index];
            if (!particle || particle.group !== group) {
                return false;
            }
            const zIdx = this._zArray.indexOf(particle), deleteCount = 1;
            this._array.splice(index, deleteCount);
            this._zArray.splice(zIdx, deleteCount);
            particle.destroy(override);
            this._engine.dispatchEvent("particleRemoved", {
                container: this._container,
                data: {
                    particle,
                },
            });
            this._addToPool(particle);
            return true;
        };
        this._engine = engine;
        this._container = container;
        this._nextId = 0;
        this._array = [];
        this._zArray = [];
        this._pool = [];
        this._limit = 0;
        this._groupLimits = new Map();
        this._needsSort = false;
        this._lastZIndex = 0;
        this._interactionManager = new InteractionManager(engine, container);
        this._pluginsInitialized = false;
        const canvasSize = container.canvas.size;
        this.quadTree = new QuadTree(qTreeRectangle(canvasSize), qTreeCapacity);
        this.movers = [];
        this.updaters = [];
    }
    get count() {
        return this._array.length;
    }
    addManualParticles() {
        const container = this._container, options = container.actualOptions;
        for (const particle of options.manualParticles) {
            this.addParticle(particle.position ? getPosition(particle.position, container.canvas.size) : undefined, particle.options);
        }
    }
    addParticle(position, overrideOptions, group, initializer) {
        const limitOptions = this._container.actualOptions.particles.number.limit, limit = group === undefined ? this._limit : this._groupLimits.get(group) ?? this._limit, currentCount = this.count, minLimit = 0;
        if (limit > minLimit) {
            if (limitOptions.mode === "delete") {
                const countOffset = 1, minCount = 0, countToRemove = currentCount + countOffset - limit;
                if (countToRemove > minCount) {
                    this.removeQuantity(countToRemove);
                }
            }
            else if (limitOptions.mode === "wait") {
                if (currentCount >= limit) {
                    return;
                }
            }
        }
        return this._pushParticle(position, overrideOptions, group, initializer);
    }
    clear() {
        this._array = [];
        this._zArray = [];
        this._pluginsInitialized = false;
    }
    destroy() {
        this._array = [];
        this._zArray = [];
        this.movers = [];
        this.updaters = [];
    }
    draw(delta) {
        const container = this._container, canvas = container.canvas;
        canvas.clear();
        this.update(delta);
        for (const [, plugin] of container.plugins) {
            canvas.drawPlugin(plugin, delta);
        }
        for (const p of this._zArray) {
            p.draw(delta);
        }
    }
    filter(condition) {
        return this._array.filter(condition);
    }
    find(condition) {
        return this._array.find(condition);
    }
    get(index) {
        return this._array[index];
    }
    handleClickMode(mode) {
        this._interactionManager.handleClickMode(mode);
    }
    async init() {
        const container = this._container, options = container.actualOptions;
        this._lastZIndex = 0;
        this._needsSort = false;
        await this.initPlugins();
        let handled = false;
        for (const [, plugin] of container.plugins) {
            if (plugin.particlesInitialization !== undefined) {
                handled = plugin.particlesInitialization();
            }
            if (handled) {
                break;
            }
        }
        this.addManualParticles();
        if (!handled) {
            const particlesOptions = options.particles, groups = particlesOptions.groups;
            for (const group in groups) {
                const groupOptions = groups[group];
                for (let i = this.count, j = 0; j < groupOptions.number?.value && i < particlesOptions.number.value; i++, j++) {
                    this.addParticle(undefined, groupOptions, group);
                }
            }
            for (let i = this.count; i < particlesOptions.number.value; i++) {
                this.addParticle();
            }
        }
    }
    async initPlugins() {
        if (this._pluginsInitialized) {
            return;
        }
        const container = this._container;
        this.movers = await this._engine.getMovers(container, true);
        this.updaters = await this._engine.getUpdaters(container, true);
        await this._interactionManager.init();
        for (const [, pathGenerator] of container.pathGenerators) {
            pathGenerator.init(container);
        }
    }
    push(nb, mouse, overrideOptions, group) {
        for (let i = 0; i < nb; i++) {
            this.addParticle(mouse?.position, overrideOptions, group);
        }
    }
    async redraw() {
        this.clear();
        await this.init();
        this.draw({ value: 0, factor: 0 });
    }
    remove(particle, group, override) {
        this.removeAt(this._array.indexOf(particle), undefined, group, override);
    }
    removeAt(index, quantity = defaultRemoveQuantity, group, override) {
        const minIndex = 0;
        if (index < minIndex || index > this.count) {
            return;
        }
        let deleted = 0;
        for (let i = index; deleted < quantity && i < this.count; i++) {
            if (this._removeParticle(i--, group, override)) {
                deleted++;
            }
        }
    }
    removeQuantity(quantity, group) {
        const defaultIndex = 0;
        this.removeAt(defaultIndex, quantity, group);
    }
    setDensity() {
        const options = this._container.actualOptions, groups = options.particles.groups, manualCount = 0;
        for (const group in groups) {
            this._applyDensity(groups[group], manualCount, group);
        }
        this._applyDensity(options.particles, options.manualParticles.length);
    }
    setLastZIndex(zIndex) {
        this._lastZIndex = zIndex;
        this._needsSort = this._needsSort || this._lastZIndex < zIndex;
    }
    setResizeFactor(factor) {
        this._resizeFactor = factor;
    }
    update(delta) {
        const container = this._container, particlesToDelete = new Set();
        this.quadTree = new QuadTree(qTreeRectangle(container.canvas.size), qTreeCapacity);
        for (const [, pathGenerator] of container.pathGenerators) {
            pathGenerator.update();
        }
        for (const [, plugin] of container.plugins) {
            plugin.update?.(delta);
        }
        const resizeFactor = this._resizeFactor;
        for (const particle of this._array) {
            if (resizeFactor && !particle.ignoresResizeRatio) {
                particle.position.x *= resizeFactor.width;
                particle.position.y *= resizeFactor.height;
                particle.initialPosition.x *= resizeFactor.width;
                particle.initialPosition.y *= resizeFactor.height;
            }
            particle.ignoresResizeRatio = false;
            this._interactionManager.reset(particle);
            for (const [, plugin] of this._container.plugins) {
                if (particle.destroyed) {
                    break;
                }
                plugin.particleUpdate?.(particle, delta);
            }
            for (const mover of this.movers) {
                if (mover.isEnabled(particle)) {
                    mover.move(particle, delta);
                }
            }
            if (particle.destroyed) {
                particlesToDelete.add(particle);
                continue;
            }
            this.quadTree.insert(new Point(particle.getPosition(), particle));
        }
        if (particlesToDelete.size) {
            const checkDelete = (p) => !particlesToDelete.has(p);
            this._array = this.filter(checkDelete);
            this._zArray = this._zArray.filter(checkDelete);
            for (const particle of particlesToDelete) {
                this._engine.dispatchEvent("particleRemoved", {
                    container: this._container,
                    data: {
                        particle,
                    },
                });
            }
            this._addToPool(...particlesToDelete);
        }
        this._interactionManager.externalInteract(delta);
        for (const particle of this._array) {
            for (const updater of this.updaters) {
                updater.update(particle, delta);
            }
            if (!particle.destroyed && !particle.spawning) {
                this._interactionManager.particlesInteract(particle, delta);
            }
        }
        delete this._resizeFactor;
        if (this._needsSort) {
            const zArray = this._zArray;
            zArray.sort((a, b) => b.position.z - a.position.z || a.id - b.id);
            const lengthOffset = 1;
            this._lastZIndex = zArray[zArray.length - lengthOffset].position.z;
            this._needsSort = false;
        }
    }
}