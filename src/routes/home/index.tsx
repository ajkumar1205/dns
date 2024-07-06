import { Elysia, t } from "elysia";
import { App } from "../../components/App";

const SignUp = () => {

    return <div class="flex items-center justify-center bg-gray-dark h-screen">
        <div class="flex items-center justify-center">
            <div id="welcome" class="text-3xl text-gray-light">
                Welcome Master!
                <div class="text-sm text-dark-blue">Verify it's you.</div>
            </div>
            <form >
                <input type="text" class="bg-gray m-2" /> <br />

                <input type="password" class="bg-gray m-2" />
            </form>

            <div class="border-r-4 p-2 text-white bg-dark-blue">Let's Go!</div>

        </div>
    </div>
}




export const home = new Elysia()
    .get("", () => <App><SignUp /></App>)