// Deployed build only: runs the simulation in-page and answers the API routes bundle.js fetches.
const ready = WebAssembly.instantiateStreaming(fetch('/morphoid.wasm')).then(({ instance }) => {
  const wasm = instance.exports;
  const sim = wasm.mh_new(Date.now() >>> 0);
  setInterval(() => wasm.mh_tick(sim), 25);
  const json = (ptr) => new TextDecoder().decode(new Uint8Array(wasm.memory.buffer, ptr, wasm.mh_json_len(sim)));
  return { wasm, sim, json };
});

const routes = {
  'GET /world/get': ({ wasm, sim, json }) => json(wasm.mh_world_json(sim)),
  'GET /world/settings/get': ({ wasm, sim, json }) => json(wasm.mh_settings_json(sim)),
  'POST /world/reset': ({ wasm, sim }) => wasm.mh_reset(sim),
  'POST /world/settings/update': ({ wasm, sim }, body) => {
    const bytes = new TextEncoder().encode(body);
    const ptr = wasm.mh_alloc(bytes.length);
    new Uint8Array(wasm.memory.buffer, ptr, bytes.length).set(bytes);
    if (!wasm.mh_update_settings(sim, ptr, bytes.length)) {
      return new Response('mutation_probability must be within [0, 1]', { status: 400 });
    }
  },
};

const realFetch = window.fetch;
window.fetch = async (url, init = {}) => {
  const path = String(url);
  const method = (init.method || 'GET').toUpperCase();
  const cell = method === 'GET' && path.match(/^\/entity\/(-?\d+)\/(-?\d+)$/);
  const route = cell
    ? ({ wasm, sim, json }) => json(wasm.mh_cell_json(sim, Number(cell[1]), Number(cell[2])))
    : routes[`${method} ${path}`];
  if (!route) return realFetch(url, init);
  const result = route(await ready, init.body);
  if (result instanceof Response) return result;
  return new Response(result ?? null, { headers: { 'Content-Type': 'application/json' } });
};
