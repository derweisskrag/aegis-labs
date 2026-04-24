import { serve } from "https://deno.land/std/http/server.ts";

serve(async (req) => {
  const url = new URL(req.url);
  const code = url.searchParams.get("code") ?? "print('no code')";
  
  const output = runLua(code); // твоя функция через FFI или WASM
  return new Response(output);
});
