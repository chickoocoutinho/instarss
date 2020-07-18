addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function createErrResponce(status) {
  const res = await fetch(`https://yuji.ne.jp/404.html`);
  return new Response(await res.text(), {
    status, headers: { "Content-Type": 'text/html; charset="UTF-8"' }
  });
}

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const url = new URL(request.url);
  if (url.pathname.slice(-4) != ".xml") { return await create404Responce(); }
  const username = url.pathname.slice(11, -4);
  const target = `https://www.instagram.com/${username}/?__a=1`;

  const res = await fetch(target);
  if (!res.ok) { return await createErrResponce(404); }
  if (res.url != target) { return await createErrResponce(403); }
  const json = await res.text();

  const { parser } = wasm_bindgen;
  await wasm_bindgen(wasm);

  return new Response(parser(json), {
    status: 200,
    headers: { "Content-Type": 'application/xml; charset="UTF-8"' }
  });
}
