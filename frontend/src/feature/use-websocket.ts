import {useSetAtom} from 'jotai';
import {Crab, crabsAtom, Food, foodsAtom, gameFieldSizeAtom} from "./atoms.ts";
import {useEffect, useState} from "react";
import {socket} from "./socket.ts";


type State = {
    size: number
    crabs: Crab[]
    foods: Food[]
}

export type WebSocket = {
    connected: boolean
}

export const useWebSocket = (): WebSocket => {
    const [isConnected, setIsConnected] = useState(socket.connected);
    const setGameFieldSize = useSetAtom(gameFieldSizeAtom);
    const setFoods = useSetAtom(foodsAtom);
    const setCrabs = useSetAtom(crabsAtom);

    useEffect(() => {
        function onConnect() {
            setIsConnected(true);
            socket.emit('get');
            console.log('socket connected')
        }

        function onDisconnect() {
            setIsConnected(false);
            console.log('socket disconnected')
        }

        function onNewState(state: State) {
            console.log('socket state', state)
            setGameFieldSize(state.size)
            setFoods(state.foods)
            setCrabs(state.crabs)
        }

        socket.on('connect', onConnect)
        socket.on('disconnect', onDisconnect)
        socket.on('state', onNewState)

        socket.connect()
        console.log('socket connect has called')

        return () => {
            socket.disconnect()
            socket.off('connect', onConnect)
            socket.off('disconnect', onDisconnect)
            socket.off('state', onNewState)
        }
    }, []);

    return {
        connected: isConnected
    }
}
