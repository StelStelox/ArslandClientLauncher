
import { BrowserRouter, Routes, Route } from "react-router";
import Layouts from "./layouts";
import Authentication from "../pages/authentication";
import Home from "../pages/home";
import "./index.css";
import ClientPanel from "../pages/clientPanel";

function App() {

  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layouts/>}>
          <Route index element={<Authentication/>}/>
          <Route path="/home" element={<Home/>}/>
          <Route path="/client-panel" element={<ClientPanel/>} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
