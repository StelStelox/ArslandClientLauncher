import { atom, useSetAtom } from 'jotai';


export const stateServer = atom(
    defaultServer()
);

export function useSelectServer() {
    const setSelectServerState = useSetAtom(stateServer);
    return {
        setSelectServerState,
    };
}

function defaultServer() {
    const server: GetClients[] = [
        {
            name: "",
            displayName: "",
            createDate: "",
            description: "",
            gameVersion: "",
            launchVersion: "",
            iconBase64: "",
            background: "",
            isEnabled: false,
            priority: 0,
            jvmArguments: "",
            gameArguments: "",
            state: 0,
            loader: 0,
            servers: [] as Server[]
        }
    ]
    return server;
}