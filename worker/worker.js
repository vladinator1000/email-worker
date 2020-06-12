addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { send } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const email = send()
    return new Response(email, {status: 200})
}
