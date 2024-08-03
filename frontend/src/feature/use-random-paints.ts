import {useAtomValue, useSetAtom} from 'jotai';
import {gameFieldSizeAtom, paintedCellsAtom} from "./atoms.ts";
import {useEffect} from "react";


export const useRandomPaints = () => {
    const gameFieldSize = useAtomValue(gameFieldSizeAtom);
    const setPaintedCells = useSetAtom(paintedCellsAtom);

    useEffect(() => {
        const randomPaint = () => {
            const lineNo = Math.floor(Math.random() * gameFieldSize) + 1;
            const cellNo = Math.floor(Math.random() * gameFieldSize) + 1;
            const color = `#${Math.floor(Math.random() * 16777215).toString(16).padStart(6, "0")}12`;
            setPaintedCells((prev) => {
                // 最大50個まで描画する
                return [{y: lineNo, x: cellNo, color}, ...prev.slice(0, 48)];
            });
        };

        const intervalId = setInterval(randomPaint, 100);
        return () => clearInterval(intervalId);
    }, [gameFieldSize, setPaintedCells]);
}

