addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const url = new URL(request.url);
  const username = url.pathname.slice(11, -4);

  const res = await fetch(`https://www.instagram.com/${username}/?__a=1`);
  const json = await res.text();


  const { parser } = wasm_bindgen;
  await wasm_bindgen(wasm);

  return new Response(parser(json), {
    status: 200,
    headers: { "Content-Type": 'application/xml; charset="UTF-8"' }
  });
}
