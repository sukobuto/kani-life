import {atom} from "jotai";


export const gameFieldSizeAtom = atom(30);
export const foodSizeMaxAtom = atom(3);

export type PaintedCell = {
    x: number
    y: number
    color: string
}
export const paintedCellsAtom = atom<PaintedCell[]>([{y: 1, x: 1, color: "#33FFFF55"}]);


export type Food = {
    id: string
    x: number
    y: number
    size: number
}
export const foodsAtom = atom<Food[]>([{id: "test001", y: 1, x: 1, size: 1}]);


export type Crab = {
    id: string
    direction: "N" | "E" | "S" | "W"
    x: number
    y: number
    hue: number
}
export const crabsAtom = atom<Crab[]>([
    {id: "test001", direction: "N", x: 10, y: 10, hue: 370},
    {id: "test002", direction: "N", x: 15, y: 3, hue: 280},
    {id: "test003", direction: "S", x: 5, y: 15, hue: 190},
]);

export const countAtom = atom(0);
