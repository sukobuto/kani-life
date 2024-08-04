import {useAtom, useAtomValue} from 'jotai';
import {foodsAtom, gameFieldSizeAtom} from "./atoms.ts";
import {useEffect} from "react";
import {v4 as uuidv4} from 'uuid';


export const useRandomFoods = () => {
    const gameFieldSize = useAtomValue(gameFieldSizeAtom);
    const [foods, setFoods] = useAtom(foodsAtom);

    useEffect(() => {
        const randomEsa = () => {
            // 最大5個までの餌を配置する
            if (foods.length >= 5) {
                return;
            }
            const y = Math.floor(Math.random() * gameFieldSize) + 1;
            const x = Math.floor(Math.random() * gameFieldSize) + 1;
            const size = Math.floor(Math.random() * 3) + 1;
            const id = uuidv4();
            // todo 既存の餌と重ならないようにする
            setFoods((prev) => {
                return [{id, y: y, x: x, size}, ...prev];
            });
        };

        const intervalId = setInterval(randomEsa, 1000);
        return () => clearInterval(intervalId);
    }, [foods.length, gameFieldSize, setFoods]);
}

