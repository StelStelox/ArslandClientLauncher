import "./index.css";

interface ServerSelectProps {
    server: Server;
    onClick?: () => void;
}

function ServerSelect({ server, onClick }: ServerSelectProps) {
    return (
        <button className="clientInfo-companent" onClick={onClick}>
            <span className="title">{server.name}</span>
            <span className="online">{server.online} / {server.maxOnline}</span>
            <div className="progress-block">
                <div
                    className="progress-line"
                    style={{ width: `${(server.online / server.maxOnline) * 100}%` }}
                ></div>
            </div>
        </button>
    );
}

export default ServerSelect