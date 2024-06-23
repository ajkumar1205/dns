import { Elysia, t } from "elysia";
import { App } from "../../components/App";

const SignUp = () => {
    return <div class="flex items-center justify-center">
        <div>
            <div>
                Welcome Master!
                <div>Verify it's you.</div>
            </div>
            <input type="text" />
            <input type="password" />
            <div>
                <span>Let's Go!</span>
            </div>
        </div>
    </div>
}





export const home = new Elysia()
    .get("", () => <App><SignUp /></App>)