import SkinViewer from "../../shared/libraries/skinViewer";
import ServersList from "../../shared/ui/serversList";
import "./index.css"

function Home() {
    return (
        <div className="homePages">
            <SkinViewer/>
            <div className="clientList">
                <ServersList/>
            </div>
        </div>
    )
}

export default Home;