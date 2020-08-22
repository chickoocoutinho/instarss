addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

function create302Response(status) {
  return Response.redirect("https://yuji.ne.jp/404.html");
}

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const url = new URL(request.url);
  if (url.pathname.slice(-4) != ".xml") {
    return create302Response();
  }

  const username = url.pathname.slice(11, -4);
  const target = `https://www.instagram.com/${username}/?__a=1`;

  const res = await fetch(target);
  if (!res.ok) { return create302Response(); }
  if (res.url != target) { return create302Response(); }
  const json = await res.text();

  const { parser } = wasm_bindgen;
  await wasm_bindgen(wasm);

  return new Response(parser(json), {
    status: 200,
    headers: { "Content-Type": 'application/xml; charset="UTF-8"' }
  });
}
