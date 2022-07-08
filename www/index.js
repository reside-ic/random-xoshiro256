import {RandomStateXoshiro256Plus} from "random-xoshiro256";

function generate(n, seed) {
    const generator = new RandomStateXoshiro256Plus(seed);
    const ret = [];
    for (let i = 0; i < n; ++i) {
        ret.push(generator.random());
    }
    return ret.join(", ");
}

function updatePage() {
    const seed = parseInt(document.getElementById("seed").value);
    const samples = parseInt(document.getElementById("samples").value);
    document.getElementById("numbers").textContent = generate(samples, seed);
}

// I am sure that there is a better way of doing this, but this works
// for now at least.
window.updatePage = updatePage;
