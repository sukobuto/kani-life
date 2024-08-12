import {useAtomValue, useSetAtom} from 'jotai';
import {decoratedCellsAtom, gameFieldSizeAtom} from "./atoms.ts";
import {useEffect} from "react";


export const useDecoration = () => {
    const gameFieldSize = useAtomValue(gameFieldSizeAtom);
    const setPaintedCells = useSetAtom(decoratedCellsAtom);

    const maxCells = gameFieldSize * gameFieldSize;

    useEffect(() => {
        const randomPaint = () => {
            const lineNo = Math.floor(Math.random() * gameFieldSize);
            const cellNo = Math.floor(Math.random() * gameFieldSize);
            const hue = Math.floor(Math.random() * 60) + 160;
            const saturation = Math.floor(Math.random() * 30) + 60;
            const color = `hsla(${hue}, ${saturation}%, 50%, 0.05)`;
            setPaintedCells((prev) => {
                // 最大50個まで描画する
                const keys = Object.keys(prev);
                if (keys.length >= maxCells) {
                    delete prev[keys[0]];
                }
                return {
                    ...prev,
                    [`${cellNo},${lineNo}`]: color
                };
            });
        };

        const intervalId = setInterval(randomPaint, 100);
        return () => clearInterval(intervalId);
    }, [gameFieldSize, maxCells, setPaintedCells]);
}

