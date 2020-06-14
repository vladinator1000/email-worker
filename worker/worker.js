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

    const init = {
      status: 200,
      headers: {
        'content-type': 'text/html;charset=UTF-8',
      },
    }

    return new Response(email, init)
}
