import './App.css'
import GameField from "./components/game-field.tsx";
import {Provider} from "jotai";

function App() {

    return (
        <>
            <Provider>
                <GameField/>
            </Provider>
        </>
    )
}

export default App
