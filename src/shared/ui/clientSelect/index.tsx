import "./index.css";

interface ClientSelectProps {
    client: GetClients;
    onClick?: () => void;
}

function ClientSelect({ client, onClick }: ClientSelectProps) {
    return (
        <button className="clientInfo-companent" onClick={onClick}>
            <span className="title">{client.displayName}</span>
            {/* <span className="online">{client.servers?.online} / {server.maxOnline}</span> */}
            {/* <div className="progress-block">
                <div
                    className="progress-line"
                    style={{ width: `${(server.online / server.maxOnline) * 100}%` }}
                ></div>
            </div> */}
        </button>
    );
}

export default ClientSelect