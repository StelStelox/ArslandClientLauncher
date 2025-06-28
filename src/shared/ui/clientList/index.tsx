import { invoke } from "@tauri-apps/api/core";
import React from "react";
import { useSelectServer } from "../../hooks/useSelectClient";
import ClientSelect from "../clientSelect";
import { useNavigate } from "react-router";
import { store } from "../../libraries/storeUtilis";
import "./index.css"

function ClientList() {
    const navigate = useNavigate();
    const [client, setClient] = React.useState<GetClients[]>([]);
    const { setSelectServerState } = useSelectServer();

    React.useEffect(() => {
        const fetchServers = async () => {
            const access_token = await store.get("access_token");
            if (!access_token) return;
            const response: ApiResponse<GetClients[]> = await invoke("get_clients", {
                accessToken: access_token
            });
            setClient(response.data || []);
        };
        fetchServers();
    }, []);

    const selectClient = async (client: GetClients) => {
        setSelectServerState([client]);
        navigate('/client-panel');
    };

    return (
        <div className="clientList-comaponent">
            {client.map((client, i) => (
                <ClientSelect
                    key={i}
                    client={client}
                    onClick={() => selectClient(client)}
                />
            ))}
        </div>
    );
}

export default ClientList;