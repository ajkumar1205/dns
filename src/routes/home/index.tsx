import { Elysia, t } from "elysia";
import { App } from "../../components/App";

const ABC = () => {
    return <div class="text-center text-red-400 text-3xl">
        This is the home
    </div>
}

export const home = new Elysia()
    .get("", () => <App><ABC /></App>)