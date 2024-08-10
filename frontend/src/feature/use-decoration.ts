import {useAtomValue, useSetAtom} from 'jotai';
import {decoratedCellsAtom, gameFieldSizeAtom} from "./atoms.ts";
import {useEffect} from "react";


export const useDecoration = () => {
    const gameFieldSize = useAtomValue(gameFieldSizeAtom);
    const setPaintedCells = useSetAtom(decoratedCellsAtom);

    const maxCells = gameFieldSize * gameFieldSize * 0.15;

    useEffect(() => {
        const randomPaint = () => {
            const lineNo = Math.floor(Math.random() * gameFieldSize);
            const cellNo = Math.floor(Math.random() * gameFieldSize);
            const color = `#${Math.floor(Math.random() * 16777215).toString(16).padStart(6, "0")}12`;
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
    }, [gameFieldSize, setPaintedCells]);
}

