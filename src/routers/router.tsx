import {BrowserRouter, Routes, Route} from "react-router-dom";

export default function Router(){
    return(
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<h2>este es el menu principal</h2>} />
            </Routes> 
        </BrowserRouter>
    );
}