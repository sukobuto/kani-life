import {useAtomValue, useSetAtom} from 'jotai';
import {Crab, crabsAtom, gameFieldSizeAtom} from "./atoms.ts";
import {useEffect} from "react";


export const useRandomCommand = () => {
    const gameFieldSize = useAtomValue(gameFieldSizeAtom);
    const setCrabs = useSetAtom(crabsAtom);

    useEffect(() => {
        const moveCrabs = () => {
            setCrabs((prev) => {
                return prev.map((crab) => {
                    if (canMoveRight(gameFieldSize, crab)) {
                        const random = Math.random();
                        if (random < 0.8) {
                            return moveRight(crab);
                        } else {
                            return turnRight(crab);
                        }
                    } else {
                        return turnRight(crab);
                    }
                });
            });
        };

        const intervalId = setInterval(moveCrabs, 600);
        return () => clearInterval(intervalId);
    }, [gameFieldSize, setCrabs]);
}

function canMoveRight(gameFieldSize: number, crab: Crab) {
    switch (crab.direction) {
        case "N":
            return crab.x < gameFieldSize;
        case "E":
            return crab.y < gameFieldSize;
        case "S":
            return crab.x > 1;
        case "W":
            return crab.y > 1;
    }
}

function moveRight(crab: Crab): Crab {
    switch (crab.direction) {
        case "N":
            return {...crab, x: crab.x + 1};
        case "E":
            return {...crab, y: crab.y + 1};
        case "S":
            return {...crab, x: crab.x - 1};
        case "W":
            return {...crab, y: crab.y - 1};
    }
}

function turnRight(crab: Crab): Crab {
    switch (crab.direction) {
        case "N":
            return {...crab, direction: "E"};
        case "E":
            return {...crab, direction: "S"};
        case "S":
            return {...crab, direction: "W"};
        case "W":
            return {...crab, direction: "N"};
    }
}


