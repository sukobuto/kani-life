import {useSetAtom} from 'jotai';
import {countAtom} from "./atoms.ts";
import {useEffect, useState} from "react";
import {socket} from "./socket.ts";


type State = {
    count: number
}

export type WebSocket = {
    connected: boolean
}

export const useWebSocket = (): WebSocket => {
    const [isConnected, setIsConnected] = useState(socket.connected);
    const setCount = useSetAtom(countAtom);

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
            setCount(state.count);
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
