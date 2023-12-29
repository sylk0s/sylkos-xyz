import { tsParticles } from "/node_modules/@tsparticles/engine";

function init() {
    // @path-json can be an object or an array, the first will be loaded directly, and the object from the array will be randomly selected
    /* tsParticles.load(@params); */

    tsParticles
        .load({
            id: "tsparticles",
            url: "presets/default.json",
        })
        .then(container => {
            console.log("callback - tsparticles config loaded");
        })
        .catch(error => {
            console.error(error);
        });

    //or

    tsParticles.load({
        id: "tsparticles",
        options: {
            preset: "stars"
        },
    });

    // now you can control the animations too, it's possible to pause and resume the animations
    // these methods don't change the config so you're safe with all your configurations
    // domItem(0) returns the first tsParticles instance loaded in the dom
    const particles = tsParticles.domItem(0);

    // play will start the animations, if the move is not enabled it won't enable it, it just updates the frame
    particles.play();
}

export default init;