interface ApiResponse<T> {
    status: string;
    statusCode: number;
    message: string;
    errors: string[];
    data?: T;
}

interface AuthenticationResponse {
    textureSkinUrl: string,
    textureCloakUrl: string,
    textureSkinGuid: string,
    textureCloakGuid: string,
    fullSkinUrl?: string,
    uuid: string,
    name: string,
    accessToken: string,
    expiredDate: string
}

interface GetClients {
    name: string;
    displayName: string;
    createDate: string;
    escription: string;
    gameVersion: string;
    launchVersion: string;
    iconBase64?: string;
    background?: string;
    isEnabled: boolean;
    priority: number;
    jvmArguments: string;
    gameArguments: string;
    state: number;
    loader: number;
    servers?: Server[];
}

interface Server {
    name: string;
    version: string;
    isOnline: boolean;
    online: number;
    maxOnline: number;
}