import { Outlet } from "react-router";
import TitleBar from "../../shared/ui/titleBar";

function Layouts () {
    return(
        <>
            <TitleBar/>
            <main>
                <Outlet/>
            </main>
        </>
    );
}

export default Layouts;