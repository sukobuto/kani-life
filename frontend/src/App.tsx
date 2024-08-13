import './App.css'
import GameField from "./components/game-field.tsx";
import {Provider} from "jotai";
import Bgm from "./components/bgm.tsx";

function App() {

    return (
        <>
            <Provider>
                <GameField/>
                <Bgm/>
            </Provider>
        </>
    )
}

export default App
