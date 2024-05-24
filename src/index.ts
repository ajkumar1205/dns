import { Elysia } from "elysia";
import { html } from "@elysiajs/html";
import { staticPlugin } from "@elysiajs/static";
import { App } from "./components/App";
import { HomePage } from "./pages/HomePage";
import { home } from "./routes/home";


const app = new Elysia()
  .use(html())
  .use(staticPlugin({
    noCache: true,
    indexHTML: false,
  }))
  .group("/", (a) => a.use(home))
  .listen(Bun.env.PORT || 3000);


console.log(
  `🦊 Elysia is running at ${app.server?.url}`
);
