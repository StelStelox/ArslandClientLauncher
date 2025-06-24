import { invoke } from "@tauri-apps/api/core";
import React from "react";
import { useSelectServer } from "../../hooks/useSelectClient";
import ServerSelect from "../serverSelect";
import { useNavigate } from "react-router";
import { store } from "../../libraries/storeUtilis";
import "./index.css"

function ServersList() {
    const navigate = useNavigate();
    const [servers, setServers] = React.useState<Server[]>([]);
    const { setSelectServerState } = useSelectServer();

    React.useEffect(() => {
        const fetchServers = async () => {
            const accessToken = await store.get<string>("accessToken");
            if (!accessToken) return;
            const response: ApiResponse<GetClients[]> = await invoke("get_clients", {
                accessToken
            });
            const clients = response.data ?? [];
            const allServers = clients.flatMap(client => client.servers ?? []);
            setServers(allServers);
        };
        fetchServers();
    }, []);

    const selectServer = async (server: Server) => {
        setSelectServerState([server]);
        navigate('/client-panel');
    };

    return (
        <div className="clientList-comaponent">
            {servers.map((server, i) => (
                <ServerSelect
                    key={i}
                    server={server}
                    onClick={() => selectServer(server)}
                />
            ))}
        </div>
    );
}

export default ServersList;