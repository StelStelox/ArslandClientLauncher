import { getCurrentWindow } from '@tauri-apps/api/window';
import "./index.css"
import React from 'react';


function TitleBar() {
    const appWindow = getCurrentWindow();

    const [title, setTitle] = React.useState("")

    React.useEffect(() => {
        const getTitle = async () => {
            const title = await appWindow.title()
            setTitle(title)
        }
        getTitle()
    }, [])
    
    return(
        <div className="titleBar" data-tauri-drag-region>
            <div>
                {title}
            </div>
            <div className="controls">
                <button onClick={async() => appWindow.minimize()}>
                    <svg width="14" height="2" viewBox="0 0 14 2" fill="none">
                        <path d="M14 2H0V0H14V2Z" fill="white"/>
                    </svg>
                </button>
                <button onClick={async() => appWindow.toggleMaximize()}>
                    <svg width="14" height="14" fill="white" version="1.1" viewBox="0 0 473 473">
                        <g id="SVGRepo_tracerCarrier" strokeLinecap="round" strokeLinejoin="round"></g><g id="SVGRepo_iconCarrier"> <g> <g> <path d="M459.5,0H330.4c-7.5,0-13.5,6-13.5,13.5s6,13.5,13.5,13.5h96.5L218.8,235.1c-5.3,5.3-5.3,13.8,0,19.1c2.6,2.6,6.1,4,9.5,4 s6.9-1.3,9.5-4L446,46.1v96.5c0,7.5,6,13.5,13.5,13.5s13.5-6,13.5-13.5V13.5C473,6,467,0,459.5,0z"></path> <path d="M459.5,231.2c-7.5,0-13.5,6-13.5,13.5v130.9c0,38.8-31.6,70.4-70.4,70.4H97.4C58.6,446,27,414.4,27,375.6V97.4 C27,58.6,58.6,27,97.4,27h129.9c7.5,0,13.5-6,13.5-13.5S234.8,0,227.3,0H97.4C43.7,0,0,43.7,0,97.4v278.2 C0,429.3,43.7,473,97.4,473h278.2c53.7,0,97.4-43.7,97.4-97.4V244.7C473,237.2,467,231.2,459.5,231.2z"></path> </g> </g> </g></svg>
                </button>
                <button onClick={async() => appWindow.close()}>
                    <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                        <path d="M8.46 7L14 12.54V14H12.54L7 8.46L1.46 14H0V12.54L5.54 7L0 1.46V0H1.46L7 5.54L12.54 0H14V1.46L8.46 7Z" fill="white"/>
                    </svg>
                </button>
            </div>
        </div>
    )
}

export default TitleBar