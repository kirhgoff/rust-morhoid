import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';

const { instance } = await WebAssembly.instantiate(readFileSync(new URL('../dist/morphoid.wasm', import.meta.url)));
const wasm = instance.exports;
const sim = wasm.mh_new(42);
const json = (ptr) => JSON.parse(new TextDecoder().decode(new Uint8Array(wasm.memory.buffer, ptr, wasm.mh_json_len(sim))));

for (let i = 0; i < 100; i++) wasm.mh_tick(sim);
const world = json(wasm.mh_world_json(sim));
assert.equal(world.width, 40);
assert.equal(world.data.length, 1600);
assert.ok(world.data.some((row) => row[0] === 'cell'));
assert.equal(json(wasm.mh_settings_json(sim)).mutation_probability, 0.5);

const send = (settings) => {
  const bytes = new TextEncoder().encode(JSON.stringify(settings));
  const ptr = wasm.mh_alloc(bytes.length);
  new Uint8Array(wasm.memory.buffer, ptr, bytes.length).set(bytes);
  return wasm.mh_update_settings(sim, ptr, bytes.length);
};
const settings = json(wasm.mh_settings_json(sim));
assert.equal(send({ ...settings, mutation_probability: 2 }), 0);
assert.equal(send({ ...settings, mutation_probability: 0.1 }), 1);
assert.equal(json(wasm.mh_settings_json(sim)).mutation_probability, 0.1);

const x = world.data.findIndex((row) => row[0] === 'cell');
assert.equal(json(wasm.mh_cell_json(sim, x % 40, Math.floor(x / 40))).genome.length, 64);
wasm.mh_reset(sim);
assert.equal(json(wasm.mh_settings_json(sim)).mutation_probability, 0.5);
console.log('wasm ok');
