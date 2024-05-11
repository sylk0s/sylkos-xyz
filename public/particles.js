tsParticles.load({
    id: "tsparticles",
    options: {
      preset: "stars",
    },
});

const particles = tsParticles.domItem(0);
particles.play();