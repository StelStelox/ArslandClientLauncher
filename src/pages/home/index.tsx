import SkinViewer from "../../shared/libraries/skinViewer";
import ClientList from "../../shared/ui/clientList";
import "./index.css"

function Home() {
    return (
        <div className="homePages">
            <SkinViewer/>
            <div className="clientList">
                <ClientList/>
            </div>
        </div>
    )
}

export default Home;