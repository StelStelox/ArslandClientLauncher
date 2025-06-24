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
    const server: Server[] = [
        {
            name: "",
            version: "",
            isOnline: false,
            online: 0,
            maxOnline: 0,
        }
    ]
    return server;
}