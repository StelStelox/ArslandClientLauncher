import React from "react";
import { IdleAnimation, SkinViewer } from "skinview3d";
import defautlSkin from "../../assets/default_skin.png";

// https://github.com/bs-community/skinview3d
function skinViewer() {
    const canvasRef = React.useRef<HTMLCanvasElement>(null)

    React.useEffect(() => {
        if(!canvasRef.current) return;
        const skinViewer = new SkinViewer({
            canvas: canvasRef.current,
            width: 200,
            height: 400,  
        })
        skinViewer.animation = new IdleAnimation();
        skinViewer.camera.position.x = -10;
        skinViewer.camera.position.y = 10;
        skinViewer.camera.position.z = 40;
        skinViewer.controls.enablePan = true
        skinViewer.loadSkin(defautlSkin)
    })

    return <canvas ref={canvasRef}/>
}

export default skinViewer;